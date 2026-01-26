#[cfg(test)]
mod tests {
    use crate::vm::vm_v4::{bytecode::{AddressMode, Operand, Size, get_opcode}, object::{Object, ObjectType, PassBy}, vm_state::{VMState, *}};
    use crate::vm::vm_v4::vm_state::VmRef;

    #[test]
    pub fn object_init_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();

        // Registers a new type to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();
        println!("{typ:?}");
        // Creates an Object within the VM
        let object = Object::new_object(typ.clone(), vm.clone()).unwrap();

        // Writes to new Object, then reads from it
        vm.borrow_mut().write_object(object, vec![0xFF]);
        assert_eq!(vm.borrow().read_object(object).unwrap()[0], 0xFF);
    }

    #[test]
    pub fn ref_counting_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Registers a new type to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();


        // Creates an Object within the VM, starts with a Ref count of 1
        let address = Object::new_object(typ.clone(), vm.clone()).unwrap();
        vm.borrow_mut().write_object(address, vec![0xFF]);

        // Sets the count to 0 (so its cleaned up)
        vm.borrow_mut().dec_object_count(address);
        // Checks that the Object has been cleaned up
        assert!(vm.borrow().read_object(address).is_none());
        println!("Hello World")
    }

    #[test]
    pub fn mass_ref_counting_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Registers a new type to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();

        let mut objs: Vec<u32> = vec![];
        for _x in 0..100 {
            objs.push(Object::new_object(typ.clone(), vm.clone()).unwrap());
        }
        
        for obj in &objs {
            vm.borrow_mut().dec_object_count(*obj);
        }

        for obj in &objs {
            assert!(vm.borrow().read_object(*obj).is_none())
        }
    }

    #[test]
    pub fn ref_counting_test_v2() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Registers a new type to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();

        // Creates 3 new objects
        let address_1 = Object::new_object(typ.clone(), vm.clone()).unwrap();
        let address_2 = Object::new_object(typ.clone(), vm.clone()).unwrap();
        let address_3 = Object::new_object(typ.clone(), vm.clone()).unwrap();

        // Clears address_1
        vm.borrow_mut().dec_object_count(address_1);
        // Makes sure address 1 is cleared
        assert!(vm.borrow().read_object(address_1).is_none());

        // Creates new Object, makes sure it filled in Address 0
        let address_4 = Object::new_object(typ.clone(), vm.clone()).unwrap();
        assert_eq!(address_4, 0);

        // Makes sure the Object still exists after 1 inc and 1 dec
        vm.borrow_mut().inc_object_count(address_2);
        vm.borrow_mut().dec_object_count(address_2);
        assert!(vm.borrow().read_object(address_2).is_some());

        // Makes sure that address_3 has not been affected
        assert!(vm.borrow().read_object(address_3).is_some());
    }

    #[test]
    pub fn struct_test() {
        // Creates a VM
        let mut vm: VmRef = VMState::default();
        // Registers 2 new (basic) types to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();
        let wrong_typ = ObjectType::new_primitive(1, "wrong", &mut vm);

        // Registers a new Struct type with types of [typ, typ]
        let stc = ObjectType::new_struct(vec![typ.clone(), typ.clone()], "struct", &mut vm.clone());
        // Makes sure the struct was properly sized
        assert_eq!(stc.size, typ.size*2);

        // Creates the Objects
        let stc_object = Object::new_object(stc, vm.clone()).unwrap();
        let basic_1 =    Object::new_object(typ.clone(), vm.clone()).unwrap();
        let basic_2 =    Object::new_object(typ.clone(), vm.clone()).unwrap();
        
        // Writes the data to basic_1 and basic_2
        vm.borrow_mut().write_object(basic_1, vec![1]);
        vm.borrow_mut().write_object(basic_2, vec![2]);

        
        // Attempts to Write to the Struct with the correct types, and Reads to check that it worked
        assert!(vm.borrow_mut().write_object_typed(stc_object, vec![basic_1, basic_2]));
        assert_eq!(vm.borrow().read_object(stc_object).unwrap(), &vec![1, 2]);

        // Creates a wrongly typed Object
        let wrong = Object::new_object(wrong_typ.clone(), vm.clone()).unwrap();
        // Tries to use the wrongly typed Object in the struct
        assert!(!vm.borrow_mut().write_object_typed(stc_object, vec![basic_1, wrong]))
    }

    #[test]
    // This simulates bytecode creating, popping, and reading Objects on the Stack
    pub fn create_object_on_stack_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // These push new primitive objects onto the Stack
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 5, true_value: None, size: Size::Byte, address_mode: AddressMode::Direct }]);
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 1003, true_value: None, size: Size::Word, address_mode: AddressMode::Direct }]);
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 10535, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]);
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 4679824527, true_value: None, size: Size::Long, address_mode: AddressMode::Direct }]);

        // Pops a (long) pointer
        let long_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        // Creates a "reference" to compare against
        let long_vec: &Vec<u8> = &u64::to_le_bytes(4679824527).to_vec();
        // Reads and compare
        assert_eq!(vm.borrow().read_object(long_pointer).unwrap(), long_vec);
        // Clears the long object
        vm.borrow_mut().dec_object_count(long_pointer);

        // Repeat for Int, Word, and Byte

        let int_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let int_vec: &Vec<u8> = &u32::to_le_bytes(10535).to_vec();
        assert_eq!(vm.borrow().read_object(int_pointer).unwrap(), int_vec);
        vm.borrow_mut().dec_object_count(int_pointer);

        let word_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let word_vec: &Vec<u8> = &vec![0xEB, 0x03];
        assert_eq!(vm.borrow().read_object(word_pointer).unwrap(), word_vec);
        vm.borrow_mut().dec_object_count(word_pointer);

        let byte_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let byte_vec: &Vec<u8> = &vec![5];
        assert_eq!(vm.borrow().read_object(byte_pointer).unwrap(), byte_vec);
        vm.borrow_mut().dec_object_count(byte_pointer);

    }

    #[test]
    pub fn new_stack_object_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap();
        let int = vm.borrow().get_type(PRIM_INT).unwrap();
        let typ = vm.borrow_mut().new_type(ObjectType { size: 5, types: vec![byte, int], pass_by: PassBy::Reference, id: "aa".to_string() });

        // These push new primitive objects onto the Stack
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 20, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]).unwrap();
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: 5, true_value: None, size: Size::Byte, address_mode: AddressMode::Direct }]).unwrap();

        // Creates a new Object with the "type" of typ, consumes the preversously made primitive objects
        (get_opcode(0).unwrap().function)(vm.clone(), vec![Operand {direct_value: typ as u64, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]).unwrap();

        // Pops the Object and compares it to a constant
        let object = vm.borrow_mut().stack_pop(false).unwrap();
        let comp: &Vec<u8> = &vec![5, 20, 0, 0, 0];
        assert_eq!(vm.borrow().read_object(object).unwrap(), comp);
        
        // Some bookkeeping 
        vm.borrow_mut().dec_object_count(object);
    }

    #[test]
    pub fn byte_parsing() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        let program: Vec<u8> = vec![];
        vm.borrow_mut().write_program(program);
    }

    #[test]
    pub fn rust_why_do_you_do_this_i_am_in_so_much_pain_right_now() {
        // Reserved for the *worst* of Rust
    }
}