// opcode definitions

use std::collections::HashMap;

pub type OpcodeTable = HashMap<String, Opcode>;

#[derive(Clone)]
pub struct Opcode {
    name: String,
    code: u16,
    fields: usize,
}

// build opcode table
pub fn build_table() {
    // define all opcodes here
    let opcodes = [
        Opcode {
            name: "load".to_string(),
            code: 0x1,
            fields: 2,
        },
        Opcode {
            name: "add".to_string(),
            code: 0x2,
            fields: 3,
        },
        Opcode {
            name: "print".to_string(),
            code: 0x3,
            fields: 1,
        },
    ];
}
