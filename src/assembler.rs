use std::path::Path;

use crate::intermediate::{Header, ScopeTree, SerializedObject};

// assembly coordinator
struct Assembler {
    scope_tree: ScopeTree,

    header: Header,
    datarom: Option<SerializedObject>,
    instructionrom: Option<SerializedObject>,
}

impl Assembler {
    fn new() {}

    /// collects source code file into an intermediate representation
    fn build_intermediate_representation(&mut self, source: &Path) {
        // open file into read buffer
        //
        todo!()
    }
    fn address_intermediate_representation() {
        todo!()
    }

    fn build_datarom() {
        todo!()
    }

    fn serialize(&self) -> SerializedObject {
        todo!()
    }
}
