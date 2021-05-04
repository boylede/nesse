use nesse_codegen::*;
use nesse_common::*;
use std::fs::File;
use std::io::Write;

fn main() {
    let known_opcodes = generate_opcode_list();

    let jumplist = generate_jumplist(&known_opcodes);
    let mut file = File::create("jumptable.rs").unwrap();
    file.write_all(jumplist.to_string().as_bytes()).unwrap();

    let function_placeholders = generate_stub_opcode_implementations(&known_opcodes);
    let mut file = File::create("stub_opcodes.rs").unwrap();
    file.write_all(function_placeholders.to_string().as_bytes())
        .unwrap();

    let opcode_names = generate_opcode_name_list(&known_opcodes);
    let mut file = File::create("opcode_debug.rs").unwrap();
    file.write_all(opcode_names.to_string().as_bytes()).unwrap();
}
