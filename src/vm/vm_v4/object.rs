use std::{collections::VecDeque, rc::Rc};

use crate::vm::vm_v4::vm_state::{VmRef, VmResult};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum PassBy {
    Value,    // Does a shallow copy
    Reference // Returns the pointer
}

impl PassBy {
    pub fn to_byte(&self) -> u8 {
        return match self {
            PassBy::Value => 0,
            PassBy::Reference => 1,
        }
    } 

    pub fn from_bytes(byte: u8) -> Self {
        return match byte {
            0 => PassBy::Value,
            _ => PassBy::Reference
        }
    }
}

// This describes a type of an Object
#[derive(PartialEq, Debug)]
pub struct ObjectType {
    pub size: u32,                  // The size in Bytes of the Object
    pub types: Vec<Rc<ObjectType>>, // The "nested" types within this Object
    pub pass_by: PassBy,            // If this type should be pass by Value, or pass by Reference
    pub id: String                  // String ID of the ObjectType, used for Serlization
}

// This describes a created Object
#[derive(Debug)]
pub struct Object {
    pub object_type: Rc<ObjectType>, // The ObjectType this describes
    pub references: u32,             // How many references point to this Object
    data: Vec<u8>,                   // The Data inside of the Object
    pub parent_vm: VmRef,            // The parent VM of this Object
    pub location: u32,               // What "memory" slot this Object is in, in its parent VM
}

impl ObjectType {
    // TODO: make the "constructors" have
    // Declares a "primitive," an ObjectType that does not depend on another type(s)
    pub fn new_primitive(size: u32, id: &str, vm: &mut VmRef) -> Rc<ObjectType> {
        let z = ObjectType { size: size, types: vec![], pass_by: PassBy::Value, id: id.to_string() };
        let x = vm.borrow_mut().new_type(z);
        return vm.borrow().get_type(x).unwrap();
    }

    // Declares a "struct," an ObjectType that does depend on another type(s)
    pub fn new_struct(types: Vec<Rc<ObjectType>>, id: String, pass_by: PassBy, vm: &mut VmRef) -> u32 {
        let mut size: u32 = 0;
        // Makes the "size" of the struct the combined size of all the other Object Types
        for typ in &types {
            size += match typ.pass_by {
                PassBy::Reference => 4, 
                PassBy::Value => typ.size,
            };
        }

        let z = ObjectType { size: size, types: types, pass_by: pass_by, id: id };
        return vm.borrow_mut().new_type(z);
    }

    // Formatted like: Size of ID, ID, Byte Size, PassBy Type, SubType Count, SubId Length-N, SubId-N
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = vec![];
        // ID Length
        // If the ID is larger than 255, there are other issues
        vec.push(self.id.len() as u8);

        // ID
        for char in self.id.as_bytes() {
            vec.push(*char);
        }

        // Byte Size
        let typ_size = & mut self.size.to_be_bytes().to_vec();
        vec.append(typ_size);

        // PassBy type
        vec.push(self.pass_by.to_byte());

        // Again, if struct has more than 255 sub types, theres other problems
        vec.push(self.types.len() as u8);

        for typ in &self.types {
            // SubType ID Length
            vec.push(typ.id.len() as u8);

            // SubType ID
            for char in typ.id.as_bytes() {
                vec.push(*char);
            }    
        }

        return vec;
    }
    // The bytes might be malformed, so we return an Option *just* in case
    pub fn from_bytes(mut vec: VecDeque<u8>, vm: VmRef) -> Option<ObjectType> {
        let id_length = vec.pop_front()?;

        let mut id_utf8: Vec<u8> = vec![];
        for _ in 0..id_length {
            id_utf8.push(vec.pop_front()?);
        }

        let id = match String::from_utf8(id_utf8) {
            Ok(x) => x,
            Err(_) => return None,
        };

        let byte_size = u32::from_be_bytes([
            vec.pop_front()?,
            vec.pop_front()?,
            vec.pop_front()?,
            vec.pop_front()?
            ]);

        let pass_by = PassBy::from_bytes(vec.pop_front()?);


        let mut sub_typs: Vec<Rc<ObjectType>> = vec![];
        // Gets how many sub types there *should* be
        for _ in 0..vec.pop_front()? {
            let size = vec.pop_front()?;

            let mut id_utf8: Vec<u8> = vec![];
            for _ in 0..size {
                id_utf8.push(vec.pop_front()?);
            }
            let string = match String::from_utf8(id_utf8) {
                Ok(x) => x,
                Err(_) => return None,
            };

            let typ = match vm.borrow().get_type_string(&string) {
                Ok(x) => x,
                Err(_) => return None,
            };

            sub_typs.push(typ);
        }

        return Some(ObjectType { size: byte_size, types: sub_typs, pass_by, id });
    }
}

impl Object {
    // Creates a new Object, if the Option is empty, that means the Object could not be allocated
    pub fn new_object(object_type: Rc<ObjectType>, vm: VmRef) -> VmResult<u32> {
        let size = object_type.size;
        let obj = Object { object_type: object_type, references: 1, data: {
            let mut z: Vec<u8> = Vec::with_capacity(size as usize);
            for _x in 0..size {
                z.push(0);
            };
            z
        }, parent_vm: vm.clone(), location: 4294967295};
        return vm.borrow_mut().new_object_direct(obj);
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

    pub fn clear_data(&mut self) {
        for x in self.data.iter_mut() {
            *x = 0;
        };
    }

    pub fn get_ref_count(&self) -> u32 {
        self.references
    }

    pub fn set_ref_count(&mut self, count: u32) {
        self.references = count;
    }
}