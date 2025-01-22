// opcode definitions

use std::collections::HashMap;

pub type OpcodeTable = HashMap<String, Opcode>;

#[derive(Clone)]
pub struct Opcode {
    name: String,
    code: u16,
    fields: usize,
}
