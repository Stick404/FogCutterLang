#[cfg(test)]
mod tests {
    use crate::vm::vm_v4::{object::{Object, ObjectType}, vm_state::{*, VMState}};
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
    pub fn rust_why_do_you_do_this_i_am_in_so_much_pain_right_now() {
        // Reserved for the *worst* of Rust
    }
}