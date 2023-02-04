use select::document::Document;
use select::node::Node;
use select::predicate::{Element, Name, Text};

use nesse_common::{
    AddressingMode, CyclesCost, NesMetaOpcode, NesOpcode, StatusFlags, StatusOption,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

const RENAME_LIST: &[(&str, &str)] = &[
    ("*IGN", "*NOP"),
    ("*SKB", "*NOP"),
    ("*ADC", "*SBC"), // ? this are wildly different
    ("*ISC", "*ISB"),
];

pub fn generate_opcode_list() -> Vec<NesOpcode> {
    let basic_reference = Document::from(include_str!("..\\reference\\6502 Reference.html"));
    let mut opcodes: Vec<NesOpcode> = vec![];
    for section in basic_reference.find(Name("div")) {
        let mut opcode = read_opcode_definition(section);
        opcodes.append(&mut opcode);
    }
    let extended_reference = include_str!("..\\reference\\undocumented.md");
    let extended_opcodes: Vec<NesOpcode> = extended_reference
        .split('\n')
        .filter(|line| !line.starts_with(' '))
        .filter(|line| line.len() > 1)
        // .inspect(|line| {
        //     println!("{}", line);
        // })
        .flat_map(|line| {
            let (name, rest) = line.split_at(3);
            let name = format!("*{}", name);
            // println!("{}\t------------", name);
            let (addressing_string, rest) = rest.rsplit_once('(').unwrap();
            // println!("<{}>", addressing_string);
            let address_mode = AddressingMode::from_reference_short_version(addressing_string);
            let (variants, cycle_str) = rest.split_once(';').unwrap();
            // println!("FACEDELIMETER");
            let cycles = cycle_str
                .split_ascii_whitespace()
                .next()
                .unwrap()
                .parse::<u8>()
                .unwrap();
            // println!("cycles: {}", cycles);
            // let cycles =
            let meta = NesMetaOpcode {
                name,
                description: String::new(), // todo from one of the split lines?
                status: StatusFlags::new(),
            };
            variants.split(',').map(move |opcode_str| {
                let mut members = opcode_str.split_ascii_whitespace();
                let (_dollar_sign, number) = members.next().unwrap().split_at(1);
                let parameters_count = members.count();
                NesOpcode {
                    meta: meta.clone(),
                    addressing: address_mode,
                    opcode: u8::from_str_radix(number, 16).unwrap(),
                    bytes: (parameters_count + 1) as u8,
                    cycles: CyclesCost::Always(cycles), // todo
                }
            })
        })
        .collect();
    opcodes.extend_from_slice(&extended_opcodes);
    opcodes
    // extended_opcodes
}

struct JumpListEntryGenerator {
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

#[derive(PartialEq, PartialOrd, Eq, Ord)]
struct OpcodeName(u8, String);

impl ToTokens for OpcodeName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let num = self.0;
        let name = &self.1;
        let toks = quote! {(#num, #name)}; //(exec:#ident, addressing:#addressing, cycles:#cycles, bytes:#bytes);
        tokens.extend(toks);
    }
}

pub fn generate_opcode_name_list(known_opcodes: &[NesOpcode]) -> TokenStream {
    let mut opcodes = Vec::with_capacity(256);
    for opcode_number in 0i32..256 {
        if let Some(opcode) = known_opcodes
            .iter()
            .find(|op| op.opcode == opcode_number as u8)
        {
            let mut entry = OpcodeName(
                opcode.opcode,
                opcode.meta.name.to_string().to_ascii_uppercase(),
            );
            if let Some((_from, to)) = RENAME_LIST
                .iter()
                .find(|(from, _)| opcode.meta.name.as_str() == *from)
            {
                entry.1 = (*to).to_owned();
            }
            opcodes.push(entry);
        } else {
            println!("no data for opcode {:02X}", opcode_number);
            let entry = OpcodeName(opcode_number as u8, "*XXX".to_string());
            opcodes.push(entry);
        }
    }

    // let mut names: Vec<OpcodeName> = known_opcodes
    //     .iter()
    //     .map(|oc| OpcodeName(oc.opcode, oc.meta.name.to_string().to_ascii_uppercase()))
    //     .collect();
    // names.sort_unstable();
    // names.dedup();
    let name_list: TokenStream = opcodes.into_iter().map(|name| quote!(#name,)).collect();
    let tokens = quote! {
        //! generated in nesse_codegen, in generate_opcode_name_list, for debugging purposes
        pub const opcode_names: &[(u8, &str);256] = &[ #name_list ];
    };
    tokens
}

fn generate_opcode_stub(name: String) -> TokenStream {
    let ident = if name.len() == 3 {
        Ident::new(&name, Span::call_site())
    } else {
        // extra opcodes include asterisk in name
        let name: String = name.chars().filter(|c| c.is_ascii_alphabetic()).collect();
        Ident::new(&name, Span::call_site())
    };
    quote! {
        pub fn #ident(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8 {
            println!("{} unimplemented", #name);
            nes.cpu.running = false;
            nes.cpu.registers.pc -= 1;
            cycles
        }
    }
}
pub fn generate_stub_opcode_implementations(known_opcodes: &[NesOpcode]) -> TokenStream {
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
    let stubs: TokenStream = opcode_names.into_iter().map(generate_opcode_stub).collect();
    quote! {
        //! generated in nesse_codegen, in generate_generate_stub_opcode_implementations,
        //! edited by hand afterwards
        use crate::{Nes, AddressableMemory};
        #stubs
    }
}
pub fn generate_jumplist(known_opcodes: &[NesOpcode]) -> TokenStream {
    let mut opcodes: Vec<JumpListEntryGenerator> = vec![];
    for opcode_number in 0i32..256 {
        if let Some(opcode) = known_opcodes
            .iter()
            .find(|op| op.opcode == opcode_number as u8)
        {
            let ident = if opcode.meta.name.len() == 3 {
                Ident::new(&opcode.meta.name.to_ascii_lowercase(), Span::call_site())
            } else {
                // extra opcodes include asterisk in name
                let name: String = opcode
                    .meta
                    .name
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect();
                Ident::new(&name.to_ascii_lowercase(), Span::call_site())
            };
            // let ident = Ident::new(&opcode.meta.name.to_ascii_lowercase(), Span::call_site());
            let jle = JumpListEntryGenerator {
                // index: opcode_number as u8,
                ident,
                addresssing: opcode.addressing.to_u8(),
                cycles: opcode.cycles.to_u8(),
                bytes: opcode.bytes,
            };
            opcodes.push(jle);
        } else {
            let jle = JumpListEntryGenerator {
                // index: opcode_number as u8,
                ident: Ident::new("placeholder", Span::call_site()),
                addresssing: 0,
                cycles: 0,
                bytes: 1,
            };
            opcodes.push(jle);
        }
    }
    // we want to output code that looks like this:
    let template: TokenStream = quote! {
            //! generated in nesse_codegen, in generate_jumplist
            use crate::Nes;
            use crate::opcodes::*;
            pub type OpcodeFn = fn(nes: &mut Nes, addressing: u8, cycles: u8, bytes: u8) -> u8;
            pub struct Opcode {
                pub exec: OpcodeFn,
                pub addressing: u8,
                pub cycles: u8,
                pub bytes: u8,
            }

            #[test]
            pub fn check_jumptable_entry_size() {
                let entry_size = std::mem::size_of::<Opcode>();
                assert!(entry_size == std::mem::align_of::<usize>() * 2);
                assert!(std::mem::size_of_val(&OPCODE_JUMPTABLE) == entry_size * 256);
            }

            impl Opcode {
                #[inline(always)]
                pub fn run(&self, nes: &mut Nes) -> u8 {
                    (self.exec)(nes, self.addressing, self.cycles, self.bytes)
                }
            }
            pub const OPCODE_JUMPTABLE: [Opcode;256] = [
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
            let addressing = AddressingMode::from_reference_material(
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
