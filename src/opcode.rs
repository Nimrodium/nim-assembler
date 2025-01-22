// opcode definitions

pub enum OpcodeField {
    String(String),
    Opcode(Opcode),
}

pub struct Opcode {
    name: String,
    code: u16,
    fields: usize,
}
