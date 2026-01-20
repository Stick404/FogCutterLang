use std::{cell::{RefCell}, rc::Rc};

use crate::vm::vm_v4::object::{Object, ObjectType, PassBy};

pub type VmRef = Rc<RefCell<VMState>>;

#[derive(PartialEq, Debug)]
pub struct VMState {
    struct_ids: Vec<Rc<ObjectType>>, // All Types known by the VM
    objects: Vec<Option<Object>>, // All Objects held within the VM, they can either be Empty, or Used
    max_memory: u64, // Max memory in bytes allocated to hold Objects
    allocated_size: u64 // Current memory of bytes allocated
}

impl VMState {
    pub fn new(memory: u64) -> VMState {
        VMState {
            struct_ids: vec![],
            objects: vec![],
            max_memory: memory,
            allocated_size: 0,
        }
    }

    pub fn default() -> VMState {
        VMState {
            struct_ids: vec![],
            objects: vec![],
            max_memory: 1024,
            allocated_size: 0,
        }
    }

    // Registiers a new type of the VMState, returns the Index
    pub fn new_type(&mut self, tpy: ObjectType) -> u32 {
        self.struct_ids.push(Rc::new(tpy));
        self.struct_ids.len() as u32
    }

    pub fn get_type(&self, index: u32) -> Option<Rc<ObjectType>> {
        let typ = self.struct_ids.get(index as usize);
        return match typ {
            Some(x) => Some(x.clone()),
            None => None
        }
    }

    // This pushs an Object to the first empty slot found
    // If Option is empty, then the VM could not allocate new space
    pub fn new_object_direct(&mut self, object: Object) -> Option<u32> {
        // Checks to see if there is am empty slot in Memory
        for (count, opt) in self.objects.iter().enumerate() {
            match opt {
                Some(_x) => {},
                None => {
                    self.allocated_size += object.object_type.size as u64;
                    self.objects[count] = Some(object);
                    return Some(count as u32)
                }
            }
        }
        // If theres is no empty slots, make a new one (if it doesn't push the VM over allocated memory)
        if self.allocated_size + (object.object_type.size as u64) < self.max_memory { // TODO: Make this check for memory allocated rather than list length
            self.allocated_size += object.object_type.size as u64;
            self.objects.push(Option::Some(object));
            return Some(self.objects.len() as u32 -1);
        }
        return None;
    }

    // Returns true if the operation was a susccess 
    pub fn write_object(&mut self, index: u32, data: Vec<u8>) -> bool {
    let obj = &mut self.objects.get_mut(index as usize);
        return match obj {
            Some(x) => {
                match x {
                    Some(y) => {
                        y.clear_data();
                        y.set_data(data);
                        true
                    }
                    None => false
                }
            }
            None => false
        }
    }

    pub fn write_object_typed(&mut self, index: u32, objects: Vec<u32>) -> bool{
        let mut data: Vec<u8> = vec![];
        let set_object = match self.objects.get(index as usize) {
            Some(x) => match x {
                Some(z) => z,
                None => return false
            },
            None => return false
        };

        for (index, address) in objects.iter().enumerate() {
            let object = match self.objects.get(*address as usize) {
                Some(x) => match x {
                    Some(z) => z,
                    None => return false
                },
                None => return false
            };
            let set_obj_typ = match set_object.object_type.types.get(index) {
                Some(x) => x.clone(),
                None => return false
            };

            if object.object_type.id != set_obj_typ.id  {
                // The types did not match up
                return false;
            }

            match object.object_type.pass_by {
                PassBy::Reference => { // Pushes the reference of the Object to the future data
                    data.push((address & 0xFF) as u8);
                    data.push(((address & 0xFF00) >> 8) as u8);
                    data.push(((address & 0xFF0000) >> 16) as u8);
                    data.push(((address & 0xFF000000) >> 24) as u8);
                }
                PassBy::Value => {
                    for value in object.get_data() {
                        data.push(*value);
                    }
                }
            };
        }
    
        if data.len() != set_object.object_type.size as usize {
            // The data somehow did not match the size of ObjectType's size
            return false; 
        }

        self.write_object(index, data);
        return true;
    }

    pub fn read_object(&self, index: u32) -> Option<&Vec<u8>> {
        let obj = &self.objects[index as usize];
        return match obj {
            Some(x) => {
                Some(x.get_data())
            }
            None => None
        }
    }

    pub fn inc_object_count(&mut self, index: u32) -> bool {
        let obj = &mut self.objects[index as usize];
        return match obj {
            Some(x) => {
                let count = x.get_ref_count();
                x.set_ref_count(count +1);
                true
            }
            None => false
        }
    }

    // TODO: possibly have this scale `self.objects` down so it wont be a slow/small memory leak
    pub fn dec_object_count(&mut self, index: u32) -> bool {
        let obj = &mut self.objects[index as usize];
        return match obj {
            Some(x) => {
                let count = x.get_ref_count() -1;
                if count <= 0 {
                    // Since the Object is getting removed, we want to remove it from the allocated size
                    self.allocated_size -= x.object_type.size as u64;
                    *obj = None;
                    return true;
                }
                x.set_ref_count(count);
                true
            }
            None => false
        }
    }
}