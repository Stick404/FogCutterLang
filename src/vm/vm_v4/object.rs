use std::{cell::RefCell, sync::Arc};

use crate::vm::vm_v4::core::VMState;

// This describes a type of an Object, may only live as long as the VM does
#[derive(PartialEq, Debug)]
pub struct ObjectType<'vm> {
    pub size: u32, // The size in Bytes of the Object
    pub types: Vec<ObjectType<'vm>>, // The "nested" types within this Object
    pub parent_vm: Arc<VMState<'vm>> // The Parent VM this is hosted in, used for Rust Reasons:tm:
}

// This describes a created Object
#[derive(PartialEq, Debug)]
pub struct Object<'vm> {
    object_type: &'vm ObjectType<'vm>, // The ObjectType this describes
    references: u32, // How many references point to this Object
    data: Vec<u8> // The Data inside of the Object
}

impl <'vm> ObjectType<'vm> {
    // Declares a "primitive," an ObjectType that does not depend on another type(s)
    pub fn new_primitive(size: u32, vm: Arc<VMState<'vm>>) -> ObjectType<'vm> {
        ObjectType { size: size, types: vec![], parent_vm: vm }
    }

    // Declares a "struct," an ObjectType that does depend on another type(s)
    pub fn new_struct(size: u32, types: Vec<ObjectType<'vm>>, vm: Arc<VMState<'vm>>)  -> ObjectType<'vm> {
        ObjectType { size: size, types: types, parent_vm: vm }
    }
}

impl<'vm> Object<'vm> {
    // Creates a new Object, will live at most as long as the VM
    pub fn new_object(object_type: &'vm ObjectType) -> Object<'vm> {
        let size = object_type.size;
        Object { object_type: object_type, references: 1, data: {
            let mut z: Vec<u8> = Vec::with_capacity(size as usize);
            for _x in 0..size {
                z.push(0);
            };
            z
        }}
    }

    pub fn get_data(&self) -> &Vec<u8>{
        return &self.data
    }

    // Returns true if the operation was successful
    pub fn set_data(&mut self, data: Vec<u8>) -> bool {
        if data.len() > self.object_type.size as usize {
            return false;
        }

        for (i, x) in data.iter().enumerate() {
            self.data[i] = *x;
        }
        return true;
    }

    pub fn get_ref_count(&mut self) -> u32 {
        self.references
    }

    pub fn set_ref_count(&mut self, count: u32) {
        self.references = count;
    }
}