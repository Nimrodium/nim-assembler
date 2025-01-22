// intermediate code objects
use crate::opcode::OpcodeField;
struct Instruction {
    opcode: OpcodeField,
    operands: Vec<MemoryAddressReference>,
}
enum MemoryAddressReference {
    Literal((usize, DataType)), // stored in data
    Symbol((String, DataType)), // stored on stack
    Array((String, DataType)),  // stored in heap
    Program(usize),             // references program
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
