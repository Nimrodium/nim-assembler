// intermediate code objects
use crate::{constant, opcode::Opcode};
use std::collections::HashMap;
pub type SerializedObject = Vec<u8>;
pub type SymbolMap = HashMap<String, Symbol>;
pub type ScopeTree = HashMap<usize, Symbol>;

pub struct Header {
    signature: String,
    version: u16,
    datarom_size: u32,
    instructionrom_size: u32,
}

struct Symbol {
    name: String,
    addr: Option<MemoryAddress>,
}

pub enum OpcodeField {
    Raw(String),
    Opcode(Opcode),
}

pub enum OperandsField {
    Raw(String),
    Operands(Vec<MemoryAddressReference>),
}

struct Memory {}

struct Instruction {
    opcode: OpcodeField,
    operands: Vec<MemoryAddressReference>,

    // metadata
    line: usize,
    scope_id: usize,
}
enum MemoryAddressReference {
    Literal((usize, DataType)), // stored in data
    Symbol((String, DataType)), // stored on stack
    Array((String, DataType)),  // stored in heap
    Program(usize),             // references program
}

impl MemoryAddressReference {
    /// builds a memory address reference object from a string
    fn from_string(string: String) -> Result<Self, String> {
        enum SuperType {
            Literal,
            Symbol,
            Array,
            Program,
        }
        let mut chars = string.chars();
        // collect super type
        let super_type = match chars.next() {
            Some(c) => match c {
                constant::SYMBOL_NOTATION => SuperType::Symbol,
                constant::LITERAL_NOTATION => SuperType::Literal,
                constant::ARRAY_NOTATION => SuperType::Array,
                _ => return Err(format!("invalid operand prelimiter [ {c} ]").to_string()),
            },
            None => return Err("empty operand".to_string()),
        };
        // collect data type
        let raw_data_type: String = chars
            .clone()
            .take_while(|&c| c != constant::TYPE_NOTATION)
            .collect();
        let data_type = match raw_data_type.as_str() {
            "u8" => DataType::Unsigned8,
            "u16" => DataType::Unsigned16,
            "u32" => DataType::Unsigned32,
            "i8" => DataType::Signed8,
            "i16" => DataType::Signed16,
            "i32" => DataType::Signed32,
            "arr" => DataType::Array,
            "str" => DataType::String,
            _ => return Err("invalid datatype".to_string()),
        };

        let val_str: String = chars.skip(raw_data_type.chars().count()).collect();
        match super_type {
            SuperType::Literal => {
                let val_lit: usize = match val_str.parse() {
                    Ok(val) => val,
                    Err(why) => return Err(why.to_string()),
                };
                Ok(Self::Literal((val_lit, data_type)))
            }
            SuperType::Symbol => Ok(Self::Symbol((val_str, data_type))),
            SuperType::Array => Ok(Self::Array((val_str, data_type))),
            SuperType::Program => {
                let val_lit: usize = match val_str.parse() {
                    Ok(val) => val,
                    Err(why) => return Err(why.to_string()),
                };
                Ok(Self::Program(val_lit))
            }
        }
    }
    ///
    fn resolve_address(&self, memory: &mut Memory) -> MemoryAddress {
        todo!()
    }
}

enum Pool {
    Program,
    Data,
    Stack,
    Heap,
}
struct MemoryAddress {
    pool: Pool,
    offset: bool,
    data_type: DataType,
    address: u32,
}

impl MemoryAddress {
    fn serialize(&self) -> Vec<u8> {
        todo!()
    }
}
enum DataType {
    // unsigned
    Unsigned8,
    Unsigned16,
    Unsigned32,

    // signed
    Signed8,
    Signed16,
    Signed32,

    // complex
    Array,
    String,
}
