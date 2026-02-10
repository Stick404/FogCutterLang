use std::{cell::RefCell, collections::{HashMap}, rc::Rc, u32, u64};

use crate::vm::vm_v4::{bytecode::*, function::{Function}, object::{Object, ObjectType, PassBy}};

// TODO: make a "Get Object" method, and clean *everything* up

pub type VmRef = Rc<RefCell<VMState>>;
pub type VmResult<X> = Result<X, (u32, &'static str)>;
pub type VmEmpty = VmResult<()>;

#[derive(Debug)]
pub struct VMState {
    struct_list: Vec<Rc<ObjectType>>, // All ObjectTypes known by the VM
    struct_ids: HashMap<String, u32>, // All known ObjectTypes
    objects: Vec<Option<Object>>,     // All Objects held within the VM, they can either be Empty, or Used
    pub program: Vec<Function>,           // All the functions within this program, function at index 0 is `main`
    max_memory: u64,                  // Max memory in bytes allocated to hold Objects
    allocated_size: u64,              // Current memory of bytes allocated (not recalculated)
    pub stack: Vec<u32>,              // Holds: Function Returns, Function Values, local Function Values
    pub base_pointer: u64,            // Points to the local "bottom" of the Stack
    pub program_pointer: u64,         // Points the section in the function to run
    pub function_pointer: u64,        // Points to the function to run in `program`
    pub running: bool,                // States if this VM is currently running or not
    pub err_code: u32,                // The error code of the program, if not 0 the program has errored
    pub jump_trueth: bool             // States if the VM's next jump should be Jump or not Jump
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

pub static ERR_FAILED_END  : u32 =  u32::MAX;
pub static ERR_OOM         : u32 =  1; // When the VM can not allocate more RAM
pub static ERR_NO_TYPE     : u32 =  2; // When the type can not be found
pub static ERR_NO_OBJECT   : u32 =  3; // When the VM can not find an Object at the given index
pub static ERR_TYPE_MISH   : u32 =  4; // When there is a Type Mishmatch
pub static ERR_OBJECT_WRITE: u32 =  5; // When an Object fails to write
pub static ERR_STACK_EMPTY : u32 =  6; // When the stack is Empty
pub static ERR_PROGRAM_READ: u32 =  7; // When theres an error in Program reading
pub static ERR_OPERAND     : u32 =  8; // When theres an error in reading Operands
pub static ERR_NO_OP_CODE  : u32 =  9; // When an OpCode can not be found
pub static ERR_PROGRAM_SHUT: u32 = 10; // When a program does not exit correctly
pub static ERR_OUT_OF_BOUND: u32 = 11; // When tries to read a list/struct out of bounds

impl VMState {
    pub fn new(memory: u64) -> VmRef {
        let state = VMState {
            struct_list: vec![],
            struct_ids: HashMap::new(),
            objects: vec![],
            program: vec![],
            max_memory: memory,
            allocated_size: 0,
            stack: vec![],
            base_pointer: 0,
            program_pointer: 0,
            function_pointer: 0,
            running: false,
            err_code: u32::MAX,
            jump_trueth: false,

        };
        return state.default_types();
    }

    pub fn default() -> VmRef {
        let state = VMState {
            struct_list: vec![],
            struct_ids: HashMap::new(),
            objects: vec![],
            program: vec![],
            max_memory: 1024,
            allocated_size: 0,
            stack: vec![],
            base_pointer: 0,
            program_pointer: 0,
            function_pointer: 0,
            running: false,
            err_code: u32::MAX,
            jump_trueth: false,
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
        // This stores 3 longs (pointers). First one is to the point in the Program to jump back to, second one is the old Base Pointer, and third is the original function
        ObjectType::new_primitive(24, "function_return", &mut vm_ref);

        let vm_typ = vm_ref.borrow().get_type(PRIM_INT).unwrap();

        // This is used for Linked Lists/Arrays
        ObjectType::new_struct(vec![
            vm_typ.clone(), // The first int is for the type of the held Object
            vm_typ.clone(), // The second int is used for the held Object Pointer
            vm_typ  // The third int is used for the next Cell in the Linked List
                                                  // If this is u32::MAX, then this is the end of the list
        ], "linked_list".to_string(), PassBy::Reference, &mut vm_ref.clone());
        return vm_ref;
    }

    // Registiers a new type of the VMState, returns the Index, if this fails, this returns u32.MAX
    pub fn new_type(&mut self, tpy: ObjectType) -> u32 {
        if self.struct_ids.contains_key(&tpy.id) {
            return 4294967295;
        }
        self.struct_list.push(Rc::new(tpy));
        
        let x= self.struct_list.len() as u32 -1;
        let id = self.struct_list.get(x as usize).unwrap().clone().id.clone();
        self.struct_ids.insert(id, x);
        return x;
    }

    pub fn get_type_string(&self, string: &String) -> VmResult<Rc<ObjectType>> {
        let x = self.struct_ids.get(string).ok_or((ERR_NO_TYPE, "vm_state::get_type_string"))?;
        return self.get_type(*x);
    }

    pub fn get_type(&self, index: u32) -> VmResult<Rc<ObjectType>> {
        let typ = self.struct_list.get(index as usize);
        return match typ {
            Some(x) => Ok(x.clone()),
            None => Err((ERR_NO_TYPE, "vm_state::get_type"))
        }
    }

    // This pushs an Object to the first empty slot found
    // If Option is empty, then the VM could not allocate new space
    pub fn new_object_direct(&mut self, mut object: Object) -> VmResult<u32> {
        // Checks to see if there is am empty slot in Memory
        for (count, opt) in self.objects.iter().enumerate() {
            match opt {
                Some(_x) => {},
                None => {
                    object.location = count as u32;
                    self.allocated_size += object.object_type.size as u64;
                    self.objects[count] = Some(object);
                    return VmResult::Ok(count as u32)
                }
            }
        }
        // If theres is no empty slots, make a new one (if it doesn't push the VM over allocated memory)
        if self.allocated_size + (object.object_type.size as u64) < self.max_memory { // TODO: Make this check for memory allocated rather than list length
            object.location = self.objects.len() as u32;
            self.allocated_size += object.object_type.size as u64;
            self.objects.push(Option::Some(object));
            return Ok(self.objects.len() as u32 -1);
        }
        return Err((ERR_OOM, "vm_state::new_object_direct"));
    }

    // Returns true if the operation was a susccess 
    pub fn write_object(&mut self, index: u32, data: Vec<u8>) -> VmEmpty {
    let obj = &mut self.objects.get_mut(index as usize);
        return match obj {
            Some(x) => {
                match x {
                    Some(y) => {
                        y.clear_data();
                        if !y.set_data(data) {
                            return Err((ERR_OBJECT_WRITE, "vm_state::write_object::set_data"));
                        }
                        Ok(())
                    }
                    None => Err((ERR_NO_OBJECT, "vm_state::write_object"))
                }
            }
            None => Err((ERR_NO_OBJECT, "vm_state::write_object"))
        }
    }

    pub fn write_object_typed(&mut self, index: u32, objects: Vec<u32>) -> VmEmpty {
        let mut data: Vec<u8> = vec![];
        let set_object = match self.objects.get(index as usize) {
            Some(x) => match x {
                Some(z) => z,
                None => return Err((ERR_NO_OBJECT, "vm_state::write_object_typed"))
            },
            None => return Err((ERR_NO_OBJECT, "vm_state::write_object_typed"))
        };

        for (index, address) in objects.iter().enumerate() {
            let object = match self.objects.get(*address as usize) {
                Some(x) => match x {
                    Some(z) => z,
                    None => return Err((ERR_NO_OBJECT, "vm_state::write_object_typed"))
                },
                None => return Err((ERR_NO_OBJECT, "vm_state::write_object_typed"))
            };
            let set_obj_typ = match set_object.object_type.types.get(index) {
                Some(x) => x.clone(),
                None => return Err((ERR_NO_TYPE, "vm_state::write_object_typed"))
            };

            if object.object_type.id != set_obj_typ.id  {
                // The types did not match up
                return Err((ERR_TYPE_MISH, "vm_state::write_object_typed"));
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
            return Err((ERR_OBJECT_WRITE, "vm_state::write_object_typed")); 
        }

        self.write_object(index, data)?;
        return Ok(());
    }

    pub fn read_object(&self, index: u32) -> VmResult<&Vec<u8>> {
        let obj = &self.objects[index as usize];
        return match obj {
            Some(x) => {
                Ok(x.get_data())
            }
            None => Err((ERR_NO_OBJECT, "vm_state::read_object"))
        }
    }

    pub fn read_object_type(&self, index: u32) -> VmResult<&Rc<ObjectType>> {
        let obj = &self.objects[index as usize];
        return match obj {
            Some(x) => {
                Ok(& x.object_type)
            }
            None => Err((ERR_NO_OBJECT, "vm_state::read_object_type"))
        }
    }

    pub fn inc_object_count(&mut self, index: u32) -> VmEmpty {
        let obj = &mut self.objects[index as usize];
        return match obj {
            Some(x) => {
                let count = x.get_ref_count();
                x.set_ref_count(count +1);
                Ok(())
            }
            None => Err((ERR_NO_OBJECT, "vm_state::inc_object_count"))
        }
    }

    // TODO: possibly have this scale `self.objects` down so it wont be a slow/small memory leak
    pub fn dec_object_count(&mut self, index: u32) -> VmEmpty {
        let obj = &mut self.objects[index as usize];
        return match obj {
            Some(x) => {
                let count = x.get_ref_count() -1;
                if count <= 0 {
                    // Since the Object is getting removed, we want to remove it from the allocated size
                    self.allocated_size -= x.object_type.size as u64;
                    *obj = None;
                    return Ok(());
                }
                x.set_ref_count(count);
                Ok(())
            }
            None => Err((ERR_NO_OBJECT, "vm_state::dec_object_count"))
        }
    }


    // TBH, we have no clue what we are doing, just doing what seems right
    // TODO: Make these all take either pointers or references to an Object, and have the real Objects live in Memory
    pub fn stack_push(&mut self, object: u32, autoinc: bool) -> VmEmpty {
        let obj = match self.objects.get(object as usize) {
            Some(x) => { match x {
                    Some(z) => z,
                    None => return Err((ERR_NO_OBJECT, "vm_state::stack_push"))
                }
            }
            None => return Err((ERR_NO_OBJECT, "vm_state::stack_push"))
        };
        if autoinc {
            self.inc_object_count(object)?;
        }
    
        self.stack.push(object);
        return Ok(());
    }

    pub fn stack_pop(&mut self, autodec: bool) -> VmResult<u32> {
        let object = self.stack.pop().ok_or((ERR_STACK_EMPTY, "vm_state::stack_pop"))?;

        let obj = match self.objects.get(object as usize) {
            Some(x) => {match x {
                    Some(z) => z,
                    None => return Err((ERR_NO_OBJECT, "vm_state::stack_pop"))
                }
            }
            None => return Err((ERR_NO_OBJECT, "vm_state::stack_pop"))
        };

        if autodec {
            self.dec_object_count(object)?;
        }
        return Ok(object) // Temp
    }

    // Returns the Pointer to the object at index
    pub fn stack_local_var(&self, index: u64) -> VmResult<u32> {
        let tmp = self.base_pointer;
        println!("reading: {index} and {tmp}");
        let obj: &u32 = self.stack.get((self.base_pointer + index) as usize).ok_or((ERR_NO_OBJECT, "vm_state::stack_local_var"))?;
        
        return Ok(*obj);
    }

    // Returns if it could write the function or not
    pub fn write_function(&mut self, function: Function) -> VmResult<u64> {
        if self.max_memory < function.function.len() as u64 {
            return Err((ERR_OOM, "vm_state::write_function"));
        }
        self.allocated_size += function.function.len() as u64;
        self.program.push(function);
    
        return Ok((self.program.len() -1) as u64);
    }

    // A quick shorthand for taking a program and running it
    pub fn run_program(vm_ref: VmRef, func: Vec<u8>) -> VmEmpty {
        // Declares the main function
        let loc = vm_ref.borrow_mut().write_function(Function::new(vec![], None, func))?;
        vm_ref.borrow_mut().function_pointer = loc;
        return VMState::run(vm_ref);
    }

    pub fn run(vm_ref: VmRef) -> VmEmpty {
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
                    
                    operands.get_mut( operads_len).ok_or((ERR_OPERAND, "vm_state::run"))?.direct_value = operand_value;
                }
                read_operand_description = !read_operand_description;
            }

            // Operands is now fully ready to be passed into the OpCode
            // Thus we run the OpCode
            (op_code.function)(vm_ref.clone(), operands)?;

            if vm_ref.borrow().err_code != u32::MAX && vm_ref.borrow().err_code != 0 {
                return Err((vm_ref.borrow().err_code, "vm_state::run"));
            }
            
        }
        
        if vm_ref.borrow().err_code != 0 {
            return Err((ERR_PROGRAM_SHUT, "vm_state::run"));
        }
        return Ok(());
    }

    pub fn read_program_byte(&mut self) -> VmResult<u8> {
        let z = *self.program.get(self.function_pointer as usize).ok_or((ERR_PROGRAM_READ, "vm_state::read_program_byte"))?.function
                .get(self.program_pointer as usize).ok_or((ERR_PROGRAM_READ, "vm_state::read_program_byte"))?;
        self.program_pointer += 1;
        return Ok(z);
    }

    pub fn is_basic_primitive(&self, pointer: u32) -> bool {
        let typ = self.read_object_type(pointer);
        if typ.is_err() {
            return false;
        }
        
        let type_id = typ.unwrap().id.clone();

        if type_id == "byte"
        || type_id == "word"
        || type_id == "int"
        || type_id == "long" {
            return true
        }

        return false;
    }
}