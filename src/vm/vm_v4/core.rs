use std::cell::{Ref, RefCell};

use crate::vm::vm_v4::object::{Object, ObjectType};

#[derive(PartialEq, Debug)]
pub struct VMState {
    struct_ids: Vec<ObjectType>
}

impl VMState {
    pub fn new() -> VMState {
        VMState { struct_ids: vec![] }
    }

    // Registiers a new type of the VMState, returns the Index
    pub fn new_type(&mut self, tpy: ObjectType) -> u32 {
        self.struct_ids.push(tpy);
        self.struct_ids.len() as u32
    }

    pub fn get_type(&self, index: u32) -> Option<& ObjectType> {
        return self.struct_ids.get(index as usize);
    }
}