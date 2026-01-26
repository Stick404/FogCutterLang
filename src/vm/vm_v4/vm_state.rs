use std::{cell::RefCell, collections::HashSet, rc::Rc, u64};

use crate::vm::{vm_v4::{bytecode::*, object::{Object, ObjectType, PassBy}}};

pub type VmRef = Rc<RefCell<VMState>>;
pub type VmResult<X> = Result<X, u32>;
pub type VmEmpty = VmResult<()>;

// TODO: Make a VMReturn type, and Error Codes rather than using `bool`

#[derive(Debug)]
pub struct VMState {
    struct_ids: Vec<Rc<ObjectType>>, // All ObjectTypes known by the VM
    struct_hashes: HashSet<String>,  // All known ObjectTypes
    objects: Vec<Option<Object>>,    // All Objects held within the VM, they can either be Empty, or Used
    program: Vec<u8>,                // The program this VMState will run, this is counted in allocated_size
    max_memory: u64,                 // Max memory in bytes allocated to hold Objects
    allocated_size: u64,             // Current memory of bytes allocated (not recalculated)
    stack: Vec<u32>,                 // Holds: Function Returns, Function Values, local Function Values
    base_pointer: u64,               // Points to the local "bottom" of the Stack
    program_pointer: u64,            // Points the section in `program` to run
    pub running: bool,                   // States if this VM is currently running or not
}

// The Stack, and the Stacking issues
// The Stack holds *all* values used in functions and function returns
// What we will likely do is have stack methods to push Objects to the Stack
// Objects can either be Runtime made Objects, or "place holders" that point back to Program Locations
pub static PRIM_BYTE: u32 = 0;
pub static PRIM_WORD: u32 = 1;
pub static PRIM_INT : u32 = 2;
pub static PRIM_LONG: u32 = 3;

pub static PRIM_FN_RT: u32 = 4;

impl VMState {
    pub fn new(memory: u64) -> VmRef {
        let state = VMState {
            struct_ids: vec![],
            struct_hashes: HashSet::new(),
            objects: vec![],
            program: vec![],
            max_memory: memory,
            allocated_size: 0,
            stack: vec![],
            base_pointer: 0,
            program_pointer: 0,
            running: false,
        };
        return state.default_types();
    }

    pub fn default() -> VmRef {
        let state = VMState {
            struct_ids: vec![],
            struct_hashes: HashSet::new(),
            objects: vec![],
            program: vec![],
            max_memory: 1024,
            allocated_size: 0,
            stack: vec![],
            base_pointer: 0,
            program_pointer: 0,
            running: false
        };
        return state.default_types();
    }

    fn default_types(self) -> VmRef {
        let mut vm_ref: VmRef = Rc::new(RefCell::new(self));
        
        ObjectType::new_primitive(1, "byte", &mut vm_ref);
        ObjectType::new_primitive(2, "word", &mut vm_ref);
        ObjectType::new_primitive(4, "int" , &mut vm_ref);
        ObjectType::new_primitive(8, "long", &mut vm_ref);

        // This is used for Stack Calls, once a type of "function return" is ran
        // This stores 2 ints (pointers), first one is to the point in the Program to jump back to, second one is the old Base Pointer
        ObjectType::new_primitive(8, "function_return", &mut vm_ref);
        return vm_ref;
    }

    // Registiers a new type of the VMState, returns the Index, if this fails, this returns u32.MAX
    pub fn new_type(&mut self, tpy: ObjectType) -> u32 {
        if !self.struct_hashes.insert(tpy.id.clone()) {
            return 4294967295;
        }
        self.struct_ids.push(Rc::new(tpy));
        self.struct_ids.len() as u32 -1
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
    pub fn new_object_direct(&mut self, mut object: Object) -> Option<u32> {
        // Checks to see if there is am empty slot in Memory
        for (count, opt) in self.objects.iter().enumerate() {
            match opt {
                Some(_x) => {},
                None => {
                    object.location = count as u32;
                    self.allocated_size += object.object_type.size as u64;
                    self.objects[count] = Some(object);
                    return Some(count as u32)
                }
            }
        }
        // If theres is no empty slots, make a new one (if it doesn't push the VM over allocated memory)
        if self.allocated_size + (object.object_type.size as u64) < self.max_memory { // TODO: Make this check for memory allocated rather than list length
            object.location = self.objects.len() as u32;
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

    pub fn read_object_type(&self, index: u32) -> Option<& Rc<ObjectType>> {
        let obj = &self.objects[index as usize];
        return match obj {
            Some(x) => {
                Some(& x.object_type)
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


    // TBH, we have no clue what we are doing, just doing what seems right
    // TODO: Make these all take either pointers or references to an Object, and have the real Objects live in Memory
    pub fn stack_push(&mut self, object: u32, autoinc: bool) -> bool {
        let obj = match self.objects.get(object as usize) {
            Some(x) => { match x {
                    Some(z) => z,
                    None => return false
                }
            }
            None => return false
        };
        // If we are pushing a Function Return, reset the Base Pointer to the (current) top of the stack
        if obj.object_type.id == self.get_type(PRIM_FN_RT).unwrap().id {
            // We do not change anything else, since that is expected by the OpCodes to do
            self.base_pointer = self.stack.len() as u64 +1;
            
        }
        if autoinc {
            self.inc_object_count(object);
        }
    
        self.stack.push(object);
        return true;
    }

    pub fn stack_pop(&mut self, autodec: bool) -> Option<u32> {
        let object = self.stack.pop()?;

        let obj = match self.objects.get(object as usize) {
            Some(x) => {match x {
                    Some(z) => z,
                    None => return None
                }
            }
            None => return None
        };

        // If we are popping a Function Return, we want to set both the Base Pointer and the Program Pointer to where it states
        if obj.object_type.id == self.get_type(PRIM_FN_RT).unwrap().id {
            let mut base: u64 = 0;
            let mut func: u64;
            let val: &Vec<u8> = obj.get_data();
            func = val[0] as u64;
            func |= (val[1] as u64) << 8;
            func |= (val[2] as u64) << 16;
            func |= (val[3] as u64) << 24;
            base |= (val[4] as u64) << 32;
            base |= (val[5] as u64) << 40;
            base |= (val[6] as u64) << 48;
            base |= (val[7] as u64) << 56;
            // TODO: Hmm, this may not be right?
            self.base_pointer = base;
            self.program_pointer = func;
        }

        if autodec {
            self.dec_object_count(object);
        }
        return Some(object) // Temp
    }

    // Returns the Pointer to the object at index
    pub fn stack_local_var(&mut self, index: u64) -> Option<u32> {
        let obj: &Object = match self.objects.get((self.base_pointer + index) as usize)? {
            Some(x) => x,
            None => return None
        };
        
        return Some(obj.location)
    }

    // Returns if it could write the program or not
    pub fn write_program(&mut self, program: Vec<u8>) -> bool {
        if self.max_memory < program.len() as u64 {
            return false;
        }
        self.allocated_size += program.len() as u64;
        self.program.clear();
        self.program.clone_from(&program);
        return true;
    }

    pub fn run_program(vm_ref: VmRef, program: Vec<u8>) -> Option<()> {
        if !vm_ref.borrow_mut().write_program(program) {
            return None;
        }
        
        // TODO: run a real program
        return VMState::run(vm_ref);
    }

    /* TODO: implement:
        Structs         
    */

    // An example ByteCode is: 0x01 0x00| 0x00 | 0x01
    // 0 byte: 
    pub fn run(vm_ref: VmRef) -> Option<()> {
        vm_ref.borrow_mut().program_pointer = 0;
        vm_ref.borrow_mut().running = true;
        while vm_ref.borrow().running {
            // Holds the raw OpCode call
            let mut op_code_value: u16 = vm_ref.borrow_mut().read_program_byte()? as u16;
            
            op_code_value |= vm_ref.borrow_mut().read_program_byte()? as u16 >> 8;

            // Contains an OpCode to be used
            let op_code: &OpCode = &get_opcode(op_code_value)?;
            let mut operands: Vec<Operand> = vec![];

            let mut read_operand_description = true;
            for _ in 0..op_code.count*2 {
                // Checks if the program is reading a description or Operand
                if read_operand_description {
                    // Pushes an half inited Operand
                    let desc_byte = vm_ref.borrow_mut().read_program_byte()?;
                    operands.push(Operand {
                        direct_value: 0,
                        true_value: None,
                        size: Size::to_size(desc_byte & 0b11),
                        address_mode: AddressMode::to_mode(desc_byte & 0b1100 >> 2)});
                        
                } else {
                    // Finishes initing the Operand
                    let mut operand_value: u64 = 0;
                    let operads_len = operands.len() -1;

                    for (count, _) in (0..operands.last().unwrap().size.get_size()).enumerate() {
                        operand_value |= (vm_ref.borrow_mut().read_program_byte()? as u64) << count*8;
                    }
                    
                    operands.get_mut( operads_len)?.direct_value = operand_value;
                }
                read_operand_description = !read_operand_description;
            }
            // Operands is now fully ready to be passed into the OpCode
            // Thus we run the OpCode

            (op_code.function)(vm_ref.clone(), operands);
        }
        return Some(());
    }

    pub fn read_program_byte(&mut self) -> Option<u8> {
        let z = *self.program.get(self.program_pointer as usize)?;
        self.program_pointer += 1;
        return Some(z);
    }
}