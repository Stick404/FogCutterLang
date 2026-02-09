// This file contains the function struct, this is used when the OpCode `Cal` calls a function

use std::rc::Rc;

use crate::vm::vm_v4::{object::ObjectType};

#[derive(PartialEq, Debug)]
pub struct Function {
    pub arg_type: Vec<Rc<ObjectType>>,    // The Arguments of the function
    pub ret_type: Option<Rc<ObjectType>>, // The Return types of the function
    pub function: Vec<u8>                 // The Bytecode of the function
}

impl Function {
    // Creates a function
    pub fn new(args: Vec<Rc<ObjectType>>, ret: Option<Rc<ObjectType>>, code: Vec<u8>) -> Self{
        Function { arg_type: args, ret_type: ret, function: code }
    }
}