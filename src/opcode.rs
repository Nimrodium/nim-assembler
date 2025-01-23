// opcode definitions

use std::collections::HashMap;

pub type OpcodeTable = HashMap<String, Opcode>;

#[derive(Clone, Debug)]
pub struct Opcode {
    name: String,
    code: u16,
    fields: usize,
}

// build opcode table
pub fn build_table() -> OpcodeTable {
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
    // registry generator
    let mut table = HashMap::new();
    for opcode in opcodes {
        table.insert(opcode.name.clone(), opcode);
    }
    table
}
