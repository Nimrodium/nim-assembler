// intermediate code objects
use crate::{
    constant::{self, SEPERATOR},
    opcode::{Opcode, OpcodeTable},
};
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

pub struct Symbol {
    name: String,
    addr: Option<MemoryAddress>,
}

pub enum OperandsField {
    Raw(String),
    Operands(Vec<MemoryAddressReference>),
}

struct Memory {}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    operands: Vec<MemoryAddressReference>,

    // metadata
    line: usize,
    scope_id: usize,
}
impl Instruction {
    /// creates new instruction from string
    pub fn new(
        string: &String,
        opcode_table: OpcodeTable,
        line: usize,
        scope_id: usize,
    ) -> Result<Self, String> {
        let split_string = respectful_split(string, SEPERATOR)?;
        println!("{}", split_string[0]);
        let opcode = if let Some(opcode) = opcode_table.get(&split_string[0].to_ascii_lowercase()) {
            opcode
        } else {
            return Err("invalid opcode".to_string());
        };
        let mut operand_objects: Vec<MemoryAddressReference> = vec![];
        for operand in split_string.iter().skip(1) {
            println!("processing {operand}");
            let obj = MemoryAddressReference::from_string(operand)?;
            operand_objects.push(obj);
        }
        Ok(Instruction {
            opcode: opcode.clone(),
            operands: operand_objects,
            line,
            scope_id,
        })
    }
}

#[derive(Debug)]
pub enum MemoryAddressReference {
    Literal((usize, DataType)), // stored in data
    Symbol((String, DataType)), // stored on stack
    Array((String, DataType)),  // stored in heap
    Program(usize),             // references program
}

impl MemoryAddressReference {
    /// builds a memory address reference object from a string
    pub fn from_string(string: &String) -> Result<Self, String> {
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
                constant::PROGRAM_NOTATION => SuperType::Program,
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

        let val_str: String = chars.skip(raw_data_type.chars().count() + 1).collect();
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
#[derive(Debug)]
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

/// split that respects string and arrays
pub fn respectful_split(string: &String, seperator: char) -> Result<Vec<String>, String> {
    let mut array: Vec<String> = vec![];
    let iterator = string.chars();

    let mut str_buffer: Vec<char> = vec![]; // individual vector element buffer

    // flag to see if inside string, adds one to nested layers until closed
    let mut is_inside_string = false;
    // how many layers deep is the data structure,
    // parser will only set is_inside flag as false, closing the data structure when this is zero.
    // each encounter of a prelimiter ( [ ) will increase this value, and every delimiter ( ] ) will decrease it.
    let mut nested_layers = 0;
    println!("seperator : [{seperator}]");

    for character in iterator {
        match character {
            // if not in a nest seperate, push and create new buffer, else treat like standard char
            c if c == seperator => {
                if nested_layers == 0 {
                    // push string buffer to return array and clear buffer
                    array.push(str_buffer.iter().collect::<String>());
                    str_buffer.clear(); // does not change allocation size, shouldnt be issue ?
                } else {
                    str_buffer.push(character);
                }
            }
            // increase nesting and then push character
            constant::ARRAY_PRELIMITER => {
                if !is_inside_string {
                    nested_layers += 1;
                    str_buffer.push(character);
                } else {
                    str_buffer.push(character);
                }
            }
            // decrease nesting and then push character
            constant::ARRAY_DELIMITER => {
                if !is_inside_string {
                    nested_layers -= 1;
                    str_buffer.push(character);
                } else {
                    str_buffer.push(character);
                }
            }
            constant::STRING_NOTATION => {
                // toggle string
                match is_inside_string {
                    true => {
                        is_inside_string = false;
                        nested_layers -= 1;
                    }
                    false => {
                        is_inside_string = true;
                        nested_layers += 1;
                    }
                };
                str_buffer.push(character)
            }
            _ => str_buffer.push(character),
        }
        // println!("str_buffer: {:?}", str_buffer)
    }
    // push last buffer on string end
    array.push(str_buffer.iter().collect::<String>());

    // verify state after parsing string
    // raise error if a nest was never closed
    if nested_layers != 0 {
        Err("nest was never closed".to_string())
    } else {
        Ok(array)
    }
}
