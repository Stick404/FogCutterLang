use std::cell::{Ref, RefCell};

use crate::vm::vm_v4::object::{Object, ObjectType};

#[derive(PartialEq, Debug)]
pub struct VMState<'vm> {
    struct_ids: Vec<ObjectType<'vm>>
}

impl <'vm> VMState<'vm> {
    pub fn new() -> VMState<'vm> {
        VMState { struct_ids: vec![] }
    }

    // Registiers a new type of the VMState, returns the Index
    pub fn new_type(&mut self, tpy: ObjectType<'vm>) -> u32 {
        self.struct_ids.push(tpy);
        self.struct_ids.len() as u32
    }

    pub fn get_type(&self, index: u32) -> Option<& ObjectType<'vm>> {
        return self.struct_ids.get(index as usize);
    }
}