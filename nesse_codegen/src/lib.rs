/// All code related to loading and running each day
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate, Text, Element};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NesMetaOpcode {
    name: String,
    description: String,
    status: StatusFlags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NesOpcode {
    meta: NesMetaOpcode,
    addressing: AddressingMode,
    opcode: u8,
    bytes: u8,
    cycles: CyclesCost,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

impl AddressingMode {
    pub fn from_str(str: &str) -> AddressingMode {
        use AddressingMode::*;
        let st = str.to_string();
        let stripped: String = st.split_whitespace().collect();
        match stripped.as_str() {
            "Implied" => Implicit,
            "Accumulator" => Accumulator,
            "Immediate" => Immediate,
            "ZeroPage" => ZeroPage,
            "ZeroPage,X" => ZeroPageX,
            "ZeroPage,Y" => ZeroPageY,
            "Relative" => Relative,
            "Absolute" => Absolute,
            "Absolute,X" => AbsoluteX,
            "Absolute,Y" => AbsoluteY,
            "Indirect" => Indirect,
            "(Indirect,X)" => IndexedIndirect,
            "(Indirect),Y" => IndirectIndexed,
            _ => panic!("not found {}", str),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StatusOption {
    Conditional,
    NotAffected,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CyclesCost {
    Always(u8),
    PageDependant(u8),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusFlags {
    carry: StatusOption,
    zero: StatusOption,
    interupt_disable: StatusOption,
    decimal: StatusOption,
    break_command: StatusOption,
    overflow: StatusOption,
    negative: StatusOption,
}

fn main() {
    let basic_reference = Document::from(include_str!("..\\reference\\6502 Reference.html"));
    let mut opcodes: Vec<NesOpcode> = vec![];
    for section in basic_reference.find(Name("div")) {
        let mut opcode = read_opcode_definition(section);
        opcodes.append(&mut opcode);
    }
    // println!("found {} opcodes", opcodes.len());

    let config = ron::ser::PrettyConfig::new();
    println!("{}", ron::ser::to_string_pretty(&opcodes, config).unwrap());
}

fn read_opcode_definition(section: Node) -> Vec<NesOpcode> {
    let header = section.find(Name("h3")).next().unwrap();
    let mut children = header.children();
    let name = children.next().unwrap().attr("name").unwrap().to_string();
    let description = children.next().unwrap().as_text().unwrap()[6..].to_string();

    let mut tables = section.find(Name("table"));
    // let starting_point = header.index();

    // let arithmatic_string = header.next().unwrap().next().unwrap();
    // // println!("arithmatic: {:?}", arithmatic_string);
    // let long_description = arithmatic_string.next().unwrap().next().unwrap();
    // let psau = long_description.next().unwrap().next().unwrap();
    // let (status_paragraph, opcode_paragraph) = if psau.children().next().unwrap().as_text() == Some("Processor Status after use:") {
    //     println!("had arithmatic, psau = {:?}", psau);
    //     let status_paragraph = psau.next().unwrap().next().unwrap();
    //     let opcode_paragraph = status_paragraph.next().unwrap();
    //     (status_paragraph, opcode_paragraph)
    // } else {
    //     println!("skipped arithmatic, psau = {:?}", psau);
    //     let opcode_paragraph = psau.next().unwrap();
    //     (psau, opcode_paragraph)
    // };
    let status = read_status_definition(tables.next().unwrap());
    let opcode = NesMetaOpcode {
        name,
        description,
        status,
    };

    read_opcodes(tables.next().unwrap(), opcode)
}

fn read_status_definition(table: Node) -> StatusFlags {
    // println!("status definition paragraph: {:?}", table);
    let mut flags = table.find(Name("tr"));
    let carry = is_not_affected(flags.next().unwrap());
    let zero = is_not_affected(flags.next().unwrap());
    let interupt_disable = is_not_affected(flags.next().unwrap());
    let decimal = is_not_affected(flags.next().unwrap());
    let break_command = is_not_affected(flags.next().unwrap());
    let overflow = is_not_affected(flags.next().unwrap());
    let negative = is_not_affected(flags.next().unwrap());
    StatusFlags {
        carry,
        zero,
        interupt_disable,
        decimal,
        break_command,
        overflow,
        negative,
    }
}

fn read_opcodes(paragraph: Node, metas: NesMetaOpcode) -> Vec<NesOpcode> {
    // println!("Generating opcodes for {:?}", metas.name);

    paragraph
        .find(Name("tr"))
        .skip(1)
        .map(|row| {
            // println!("opcode row: {:?}", row);
            let meta = metas.clone();
            let mut columns = row.find(Name("td"));
            let addressing = AddressingMode::from_str(
                columns
                    .next()
                    .unwrap()
                    .find(Text)
                    .next()
                    .unwrap()
                    .as_text()
                    .unwrap(),
            );
            // println!("ad: {:?}", addressing);

            let opcode = u8::from_str_radix(
                &columns
                    .next()
                    .unwrap()
                    .find(Name("center"))
                    .next()
                    .unwrap()
                    .find(Text)
                    .next()
                    .unwrap()
                    .as_text()
                    .unwrap()[1..],
                16,
            ).unwrap();
            // println!("op: {:?}", opcode);
            
            let bytes_str: String = columns
            .next()
            .unwrap()
            // .find(Name("center"))
            // .next()
            // .unwrap()
            // .find(Text)
            // .next()
            // .unwrap()
            .text().split_whitespace().collect();
            // println!("bytes_str: {}", bytes_str);
            let bytes = bytes_str.parse::<u8>().unwrap();
            // println!("by: {:?}", bytes);

            let cycles_str = columns.next().unwrap().find(Text).next().unwrap().as_text().unwrap();
            let cycles = match cycles_str.parse::<u8>() {
                Ok(n) => CyclesCost::Always(n),
                Err(_) => {
                    // println!("couldnt figure out cycles count: {:?}", cycles_str);
                    let n = cycles_str.split(' ').next().unwrap().parse::<u8>().unwrap();
                    CyclesCost::PageDependant(n)
                }
            };
            // println!("cy: {:?}", cycles);

            NesOpcode {
                meta, addressing, opcode, bytes, cycles
            }
        })
        .collect()
}

fn is_not_affected(row: Node) -> StatusOption {
    // let text = row.children().nth(2).unwrap().as_text().unwrap();
    let text = row.find(Element).nth(4).unwrap().text();
    // println!("text: {:?}", text);
    if text == "Not affected" {
        StatusOption::NotAffected
    } else {
        StatusOption::Conditional
    }
}
