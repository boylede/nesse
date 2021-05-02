/// All code related to loading and running each day
use std::fmt;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Seek, SeekFrom, Write};

use select::document::Document;
use select::node::Node;
use select::predicate::{Element, Name, Predicate, Text};

use nesse_common::{
    AddressingMode, CyclesCost, NesMetaOpcode, NesOpcode, StatusFlags, StatusOption,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

pub fn generate_opcode_list() -> Vec<NesOpcode> {
    let basic_reference = Document::from(include_str!("..\\reference\\6502 Reference.html"));
    let mut opcodes: Vec<NesOpcode> = vec![];
    for section in basic_reference.find(Name("div")) {
        let mut opcode = read_opcode_definition(section);
        opcodes.append(&mut opcode);
    }

    opcodes
}

struct JumpListEntryGenerator {
    index: u8,
    ident: Ident,
    addresssing: u8,
    cycles: u8,
    bytes: u8,
}

impl ToTokens for JumpListEntryGenerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident.clone();
        let addressing = self.addresssing;
        let cycles = self.cycles;
        let bytes = self.bytes;
        let toks = quote!(exec:#ident, addressing:#addressing, cycles:#cycles, bytes:#bytes);
        tokens.extend(toks);
    }
}

fn generate_opcode_stub(name: String) -> TokenStream {
    let ident = Ident::new(&name, Span::call_site());
    quote! {
        pub fn #ident(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
            println!("{} unimplemented", #name);
            nes.cpu.running = false;
            nes.cpu.registers.pc -= 1;
            cycles
        }
    }
}
pub fn generate_stub_opcode_implementations(known_opcodes: &Vec<NesOpcode>) -> TokenStream {
    let mut opcode_names: Vec<String> = known_opcodes
        .iter()
        .map(|oc| oc.meta.name.to_string().to_ascii_lowercase())
        .collect();
    opcode_names.sort_unstable();
    opcode_names.dedup();
    // let names: Vec<Ident> = opcode_names
    //     .iter()
    //     .map(|name| Ident::new(name, Span::call_site()))
    //     .collect();
    let stubs: TokenStream = opcode_names
        .into_iter()
        .map(|name| generate_opcode_stub(name))
        .collect();
    quote! {
        use crate::Nes;
        #stubs
    }
}
pub fn generate_jumplist(known_opcodes: &Vec<NesOpcode>) -> TokenStream {
    let mut opcodes: Vec<JumpListEntryGenerator> = vec![];
    for opcode_number in 0i32..256 {
        if let Some(opcode) = known_opcodes
            .iter()
            .find(|op| op.opcode == opcode_number as u8)
        {
            let ident = Ident::new(&opcode.meta.name.to_ascii_lowercase(), Span::call_site());
            let jle = JumpListEntryGenerator {
                index: opcode_number as u8,
                ident,
                addresssing: opcode.addressing.to_u8(),
                cycles: opcode.cycles.to_u8(),
                bytes: opcode.bytes,
            };
            opcodes.push(jle);
        } else {
            let jle = JumpListEntryGenerator {
                index: opcode_number as u8,
                ident: Ident::new("placeholder", Span::call_site()),
                addresssing: 0,
                cycles: 0,
                bytes: 0,
            };
            opcodes.push(jle);
        }
    }
    // we want to output code that looks like this:
    let template: TokenStream = quote! {
            use crate::Nes;
            use crate::opcodes::*;
            pub type OpcodeFn = fn(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8;
            pub struct Opcode {
                pub exec: OpcodeFn,
                pub addressing: u8,
                pub cycles: u8,
                pub bytes: u8,
            }

            impl Opcode {
                #[inline(always)]
                pub fn run(&self, nes: &mut Nes) -> u8 {
                    (self.exec)(nes, self.addressing, self.cycles, self.bytes)
                }
            }
            pub const opcode_jumptable: [Opcode;256] = [
                #(Opcode {#opcodes},)*
            ];

            pub fn placeholder(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
                println!("opcode not implemented.");
                0
            }
    };
    template
}

// we need a function that generates code that looks like:
// register_opcode!(number, IDENT, addressing, cycles, bytes);

fn read_opcode_definition(section: Node) -> Vec<NesOpcode> {
    let header = section.find(Name("h3")).next().unwrap();
    let mut children = header.children();
    let name = children.next().unwrap().attr("name").unwrap().to_string();
    let description = children.next().unwrap().as_text().unwrap()[6..].to_string();

    let mut tables = section.find(Name("table"));

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
            )
            .unwrap();
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
                .text()
                .split_whitespace()
                .collect();
            // println!("bytes_str: {}", bytes_str);
            let bytes = bytes_str.parse::<u8>().unwrap();
            // println!("by: {:?}", bytes);

            let cycles_str = columns
                .next()
                .unwrap()
                .find(Text)
                .next()
                .unwrap()
                .as_text()
                .unwrap();
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
                meta,
                addressing,
                opcode,
                bytes,
                cycles,
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
