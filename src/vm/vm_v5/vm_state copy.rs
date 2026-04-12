use std::{cell::RefCell, ops::Add, rc::Rc, string, u32};

use crate::vm::vm_v5::objects::{ReferentialObject, Object, ObjectType};

pub type VmRef = Rc<RefCell<VMState>>;
pub type VmResult<X> = Result<X, (u32, &'static str)>;
pub type VmEmpty = VmResult<()>;

#[derive(PartialEq, Debug, Clone)]
pub struct VMState {
    pub basic_reference: Option<VmRef>,
    max_memory: u64,
    used_memory: u64,
    objects: Vec<Option<MemoryObject>>, // All Objects held within the VM, they can either be Empty, or Used
}

#[derive(PartialEq, Debug, Clone)]
struct MemoryObject {
    object: Object,
}

impl VMState {
    pub fn new(memory: u64) -> VmRef {
        let x= Rc::new(RefCell::new(
            VMState {
                basic_reference: None,
                max_memory: memory,
                used_memory: 0,
                objects: vec![]
            }));
        x.borrow_mut().basic_reference = Some(x.clone());
        return x
    }

    pub fn new_number_object(&mut self, typ: ObjectType) -> VmResult<ReferentialObject> {
        let result = Object::create_basic_number(typ);
        if result.is_ok() {
            return self.alloc_object(result.unwrap());
        }
        return Err((0, ""));
    }

    pub fn new_string_object(&mut self, string: String) -> VmResult<ReferentialObject> {
        let obj = Object::String(string);
        self.alloc_object(obj)
    }

    pub fn get_object_size(&self, obj: &ReferentialObject ) -> VmResult<u32> {
        return match self.objects.get(obj.index as usize) {
            Some(x) => match x {
                Some(y) => Ok(y.object.get_size()),
                None => Err((0, ""))
            }
            None => Err((0, ""))
        }
    }

    fn alloc_object(&mut self, mut object: Object) -> VmResult<ReferentialObject> {
        // Checks to see if there is am empty slot in Memory
        for (count, opt) in self.objects.iter_mut().enumerate() {
            match opt {
                Some(_x) => {},
                None => {
                    let x = ReferentialObject { typ: object.to_type(), index: count as u32,
                        vm: self.basic_reference.clone().ok_or((0, "vm_state::alloc_object"))? };
                    self.used_memory += object.get_size() as u64;
                    (*opt) = Some(MemoryObject { object });
                    return VmResult::Ok(x)
                }
            }
        }
        // If theres is no empty slots, make a new one (if it doesn't push the VM over allocated memory)
        if self.used_memory + (object.get_size() as u64) < self.max_memory {
            self.used_memory += object.get_size() as u64;
            let obj_typ = object.to_type();

            self.objects.push(Option::Some(MemoryObject { object }));
            return Ok(ReferentialObject { typ: obj_typ, index: (self.objects.len() -1) as u32,
                vm: self.basic_reference.clone().ok_or((0, "vm_state::alloc_object"))? });
        }
        return Err((0, "vm_state::new_object_direct"));
    }
}