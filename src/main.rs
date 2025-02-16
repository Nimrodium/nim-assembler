use std::{
    fs::File,
    io::{self, BufWriter, Read, Write},
    path::Path,
};

use intermediate::{respectful_split, Instruction, SerializedObject};
use opcode::build_table;

mod assembler;
mod constant;
mod intermediate;
mod opcode;
mod serialize;

use crate::intermediate::MemoryAddressReference;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    // let parsed = MemoryAddressReference::from_string(&args[1]).unwrap();
    println!("input: {}", args[1]);
    // let parsed = respectful_split(&args[1], constant::SEPERATOR).unwrap();
    let opcode_table = build_table();
    let instruction = Instruction::new(&args[1], opcode_table, 0, 0).unwrap();
    println!("output: {instruction:?}");
}

// flash assembled code to file
fn flash(code: SerializedObject, file_path: &Path) -> Result<(), String> {
    // open file into buffer
    let mut out = BufWriter::new(match File::create(file_path) {
        Ok(bufwriter) => bufwriter,
        Err(why) => return Err(why.to_string()),
    });
    //flash byte vector
    match out.write_all(&code) {
        Ok(()) => (),
        Err(why) => return Err(why.to_string()),
    };
    // read back the file
    let mut file_ro = match File::open(file_path) {
        Ok(file) => file,
        Err(why) => return Err(why.to_string()),
    };
    // let signature = file_ro.
    let mut program_signature_buffer = vec![0; constant::SIGNATURE.len()];
    match file_ro.read_exact(&mut program_signature_buffer) {
        Ok(()) => (),
        Err(why) => return Err(why.to_string()),
    };
    let program_signature = match String::from_utf8(program_signature_buffer) {
        Ok(string) => string,
        Err(why) => return Err(why.to_string()),
    };
    if program_signature != constant::SIGNATURE {
        Err("flashing code to file failed somehow, failed to verify signature".to_string())
    } else {
        Ok(())
    }
}
