#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use crate::vm::vm_v4::{_bytecode::{AddressMode, Operand, Size, get_opcode}, _function::Function, _object::{Object, ObjectType, PassBy}, _vm_state::{VMState, *}};
    use crate::vm::vm_v4::_vm_state::VmRef;

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
        vm.borrow_mut().write_object(object, vec![0xFF]).unwrap();
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
        vm.borrow_mut().write_object(address, vec![0xFF]).unwrap();

        // Sets the count to 0 (so its cleaned up)
        vm.borrow_mut().dec_object_count(address).unwrap();
        // Checks that the Object has been cleaned up
        assert!(vm.borrow().read_object(address).is_err());
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
            vm.borrow_mut().dec_object_count(*obj).unwrap();
        }

        for obj in &objs {
            assert!(vm.borrow().read_object(*obj).is_err())
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
        vm.borrow_mut().dec_object_count(address_1).unwrap();
        // Makes sure address 1 is cleared
        assert!(vm.borrow().read_object(address_1).is_err());

        // Creates new Object, makes sure it filled in Address 0
        let address_4 = Object::new_object(typ.clone(), vm.clone()).unwrap();
        assert_eq!(address_4, 0);

        // Makes sure the Object still exists after 1 inc and 1 dec
        vm.borrow_mut().inc_object_count(address_2).unwrap();
        vm.borrow_mut().dec_object_count(address_2).unwrap();
        assert!(vm.borrow().read_object(address_2).is_ok());

        // Makes sure that address_3 has not been affected
        assert!(vm.borrow().read_object(address_3).is_ok());
    }

    #[test]
    pub fn struct_test() {
        // Creates a VM
        let mut vm: VmRef = VMState::default();
        // Registers 2 new (basic) types to the VM
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();
        let wrong_typ = ObjectType::new_primitive(1, "wrong", &mut vm);

        // Registers a new Struct type with types of [typ, typ]
        let stc_pointer = ObjectType::new_struct(vec![typ.clone(), typ.clone()], "struct".to_string(), PassBy::Reference, &mut vm.clone());
        let stc = vm.borrow().get_type(stc_pointer).unwrap();
        // Makes sure the struct was properly sized
        assert_eq!(stc.size, typ.size*2);

        // Creates the Objects
        let stc_object = Object::new_object(stc, vm.clone()).unwrap();
        let basic_1 =    Object::new_object(typ.clone(), vm.clone()).unwrap();
        let basic_2 =    Object::new_object(typ.clone(), vm.clone()).unwrap();
        
        // Writes the data to basic_1 and basic_2
        vm.borrow_mut().write_object(basic_1, vec![1]).unwrap();
        vm.borrow_mut().write_object(basic_2, vec![2]).unwrap();

        
        // Attempts to Write to the Struct with the correct types, and Reads to check that it worked
        assert!(vm.borrow_mut().write_object_typed(stc_object, vec![basic_1, basic_2]).is_ok());
        assert_eq!(vm.borrow().read_object(stc_object).unwrap(), &vec![1, 2]);

        // Creates a wrongly typed Object
        let wrong = Object::new_object(wrong_typ.clone(), vm.clone()).unwrap();
        // Tries to use the wrongly typed Object in the struct
        assert!(vm.borrow_mut().write_object_typed(stc_object, vec![basic_1, wrong]).is_err())
    }

    #[test]
    // This simulates bytecode creating, popping, and reading Objects on the Stack
    pub fn create_object_on_stack_test() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // These push new Primitive Objects onto the Stack
        // Also simulates OpCodes getting called with the right Operands
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 5, true_value: None, size: Size::Byte, address_mode: AddressMode::Direct }]).unwrap();
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 1003, true_value: None, size: Size::Word, address_mode: AddressMode::Direct }]).unwrap();
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 10535, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]).unwrap();
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 4679824527, true_value: None, size: Size::Long, address_mode: AddressMode::Direct }]).unwrap();

        // Pops a (long) pointer
        let long_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        // Creates a "reference" to compare against
        let long_vec: &Vec<u8> = &u64::to_le_bytes(4679824527).to_vec();
        // Reads and compare
        assert_eq!(vm.borrow().read_object(long_pointer).unwrap(), long_vec);
        // Clears the long object
        vm.borrow_mut().dec_object_count(long_pointer).unwrap();

        // Repeat for Int, Word, and Byte

        let int_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let int_vec: &Vec<u8> = &u32::to_le_bytes(10535).to_vec();
        assert_eq!(vm.borrow().read_object(int_pointer).unwrap(), int_vec);
        vm.borrow_mut().dec_object_count(int_pointer).unwrap();

        let word_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let word_vec: &Vec<u8> = &vec![0xEB, 0x03];
        assert_eq!(vm.borrow().read_object(word_pointer).unwrap(), word_vec);
        vm.borrow_mut().dec_object_count(word_pointer).unwrap();

        let byte_pointer = vm.borrow_mut().stack_pop(false).unwrap();
        let byte_vec: &Vec<u8> = &vec![5];
        assert_eq!(vm.borrow().read_object(byte_pointer).unwrap(), byte_vec);
        vm.borrow_mut().dec_object_count(byte_pointer).unwrap();

    }

    #[test]
    // This tests creating a Struct/Non-Primitive Object via the Stack
    pub fn new_stack_object_test() {
        // Creates a VM
        let mut vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap();
        // Gets the Int ObjectType
        let int = vm.borrow().get_type(PRIM_INT).unwrap();
        // Creates an "Object" called `aa` that takes 5 bytes, and holds
        
        // Creates a new ObjectType that holds byte, int, named "aa," that is pass by Reference
        // in C this could be:
        // struct AA {
        //      byte x;
        //      int  y;
        //};
        let typ = ObjectType::new_struct(vec![byte, int], "aa".to_string(), PassBy::Reference, &mut vm);

        // These push new primitive objects onto the Stack
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 20, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]).unwrap();
        (get_opcode(2).unwrap().function)(vm.clone(), vec![Operand {direct_value: 5, true_value: None, size: Size::Byte, address_mode: AddressMode::Direct }]).unwrap();

        // Creates a new Object with the "type" of typ, consumes the preversously made primitive objects
        (get_opcode(1).unwrap().function)(vm.clone(), vec![Operand {direct_value: typ as u64, true_value: None, size: Size::Int, address_mode: AddressMode::Direct }]).unwrap();

        // Pops the Object and compares it to a constant
        let object = vm.borrow_mut().stack_pop(false).unwrap();
        let comp: &Vec<u8> = &vec![5, 20, 0, 0, 0];
        assert_eq!(vm.borrow().read_object(object).unwrap(), comp);
        
        // Some bookkeeping 
        vm.borrow_mut().dec_object_count(object).unwrap();
    }

    #[test]
    // This runs a very basic program, copies what `new_stack_object_test` does but in ByteCode
    pub fn basic_program() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap();
        // Gets the Int ObjectType
        let int = vm.borrow().get_type(PRIM_INT).unwrap();

        // Creates a Struct Object
        let typ = ObjectType::new_struct(vec![byte, int], "aa".to_string(), PassBy::Reference, &mut vm.clone());
        let program: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000010, /* Direct, Int  */ 0x14, 0x00, 0x00, 0x00, /* 20 */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */

            0x01, 0x00, /* New     */ 0b00000000, /* Direct, Byte */ typ as u8, // Shouldn't ever have more than 255 built in types, so this is safe,
            0x00, 0x00, /* Ends the program */
        ];

        VMState::run_program(vm.clone(), program).unwrap();

        let object = vm.borrow_mut().stack_pop(false).unwrap();
        let comp: &Vec<u8> = &vec![5, 20, 0, 0, 0];
        assert_eq!(vm.borrow().read_object(object).unwrap(), comp);
    }

    #[test]
    // Tests Serlizing/Unserlizing of a Primitive ObjectType
    pub fn basic_serialize_object_type() {
        // Creates a VM
        let vm: VmRef = VMState::default();

        // Gets the type for Byte
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();
        // Serlizes Byte to a Byte Array
        let serz = typ.to_bytes();

        print!("{serz:?}");

        // Unserlizes the Byte Array to an ObjectType
        let unserz = ObjectType::from_bytes(VecDeque::from(serz), vm).unwrap();

        let re_type = typ.as_ref();
        // Does deeper sanity checks to make sure the Serlizing/Deserlizing worked
        assert_eq!(unserz.id, re_type.id);
        assert_eq!(unserz.pass_by, re_type.pass_by);
        assert_eq!(unserz.size, re_type.size);
        assert_eq!(unserz.types.len(), re_type.types.len());
    }

    #[test]
    // Tests Serlizing/Unserlizing of a Complex ObjectType (Struct)
    pub fn complex_serialize_object_type() {
        // Creates a VM
        let vm: VmRef = VMState::default();

        // Gets a basic Byte ObjectType
        let typ = vm.borrow().get_type(PRIM_BYTE).unwrap();

        // Creates a new Struct called "struct" of Object of Byte, Byte
        let stc_pointer = ObjectType::new_struct(vec![typ.clone(), typ.clone()], "struct".to_string(), PassBy::Reference, &mut vm.clone());
        // Gets the ObjectType "struct"
        let stc_type = vm.borrow().get_type(stc_pointer).unwrap();
        // Serlizes "struct" to a Byte Array
        let serz = stc_type.to_bytes();

        print!("{serz:?}");

        // Unserlizes the Byte Array to an ObjectType
        let unserz = ObjectType::from_bytes(VecDeque::from(serz), vm).unwrap();
        let re_type = stc_type.as_ref();
        // Does deeper sanity checks to make sure the Serlizing/Deserlizing worked
        assert_eq!(unserz.id, re_type.id);
        assert_eq!(unserz.pass_by, re_type.pass_by);
        assert_eq!(unserz.size, re_type.size);
        assert_eq!(unserz.types.len(), re_type.types.len());
    }

    #[test]
    pub fn op_code_dup() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap();
        // Gets the Int ObjectType
        let int = vm.borrow().get_type(PRIM_INT).unwrap();

        // Creates a Struct Object
        let typ = ObjectType::new_struct(vec![byte, int], "aa".to_string(), PassBy::Reference, &mut vm.clone());
        let program: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000010, /* Direct, Int  */ 0x14, 0x00, 0x00, 0x00, /* 20 */

            0x03, 0x00, /* Dup */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */

            0x01, 0x00, /* New */ 0b00000000, /* Direct, Byte */ typ as u8, // Shouldn't ever have more than 255 built in types, so this is safe,

            0x03, 0x00, /* Dup */

            0x00, 0x00, /* Ends the program */
        ];

        VMState::run_program(vm.clone(), program).unwrap();

        let dupped = vm.borrow_mut().stack_pop(false).unwrap();
        let original = vm.borrow_mut().stack_pop(false).unwrap();
        let primitive = vm.borrow_mut().stack_pop(false).unwrap();

        assert_eq!(dupped, original);
        assert_ne!(primitive, 0);
    }

    #[test]
    fn test_basic_add() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let program: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000010, /* Direct, Word  */ 0x14, 0x00, 0x00, 0x00, /* 20 */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */

            0x09, 0x00, /* Add */ // Should now have w25 on the stack 
            0x00, 0x00, /* Ends the program */
        ];

        VMState::run_program(vm.clone(), program).unwrap();

        let added = vm.borrow_mut().stack_pop(false).unwrap();
        let typ = vm.borrow().read_object_type(added).unwrap().id.clone();
        assert_eq!(vm.borrow().read_object(added).unwrap(), &vec![25, 0, 0, 0]);
        assert_eq!(typ, "int")
    }

    #[test]
    fn test_basic_sub() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let program: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000010, /* Direct, Word  */ 0x14, 0x00, 0x00, 0x00, /* 20 */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */

            0x0A, 0x00, /* Sub */ // Should now have w15 on the stack 
            0x00, 0x00, /* Ends the program */
        ];

        VMState::run_program(vm.clone(), program).unwrap();

        let added = vm.borrow_mut().stack_pop(false).unwrap();
        let typ = vm.borrow().read_object_type(added).unwrap().id.clone();
        assert_eq!(vm.borrow().read_object(added).unwrap(), &vec![15, 0, 0, 0]);
        assert_eq!(typ, "int")
    }

    #[test]
    fn test_jump_unc() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let program: Vec<u8> = vec![
            11, 0, /* JmpUnc */ 0b00000000, 0x08, /* To byte 8 */

            255, 255, 255, 255, // Wall of "Crash Program"
            /* byte 8 -> */ 0x00, 0x00, /* Ends the program */
            255, 255, 255, 255, // Wall of "Crash Program"
        ];

        VMState::run_program(vm.clone(), program).unwrap();
    }

    #[test]
    fn test_jump_cnd() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let program: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            12, 0, /* CmpEq, should be true */
            18, 0, 0b00000000, /* Direct, Byte */ 19, /* byte 19, if true */ /* Direct, Byte */ 16, /* byte 16, if false */

            255, 255, 255, 255,
            /* byte 19 -> */ 0x00, 0x00, /* Ends the program */
            255, 255, 255, 255,
        ];

        VMState::run_program(vm.clone(), program).unwrap();
    }

    #[test]
    fn test_cal() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let main: Vec<u8> = vec![
            19, 0, 0b00000000, 0, // Calls function 0

            0x00, 0x00,
            255, 255, 255, 255,
        ];

        let func: Vec<u8> = vec![
            20, 0 // A basic function that just returns
        ];

        vm.borrow_mut().write_function(Function::new(vec![], None, func)).unwrap();

        VMState::run_program(vm.clone(), main).unwrap();
    }

    #[test]
    fn test_ret_ret() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType
        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap().clone();
        let func = Function::new(vec![], Some(byte.clone()), vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            0x09, 0x00, /* Add */ // Should now have b10 on the stack 

            21, 0, 0b00000000, 1, // Returns
        ]);
        let add_func = vm.borrow_mut().write_function(func).unwrap();

        let main: Vec<u8> = vec![
            19, 0, 0b00000000, add_func as u8, // Calls function 0

            0x00, 0x00,
            255, 255, 255, 255,
        ];      

        VMState::run_program(vm.clone(), main).unwrap();
        let added = vm.borrow_mut().stack_pop(false).unwrap();
        let typ = vm.borrow().read_object_type(added).unwrap().id.clone();
        assert_eq!(vm.borrow().read_object(added).unwrap(), &vec![10]);
        assert_eq!(typ, "byte");
    }

    #[test]
    fn test_cal_ret_types() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType

        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap().clone();
        let func = Function::new(vec![byte.clone()], Some(byte), vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            0x09, 0x00, /* Add */ // Just adds 5 to the given arg

            21, 0, 0b00000000, 1, // Returns the add
        ]);

        let fnc = vm.borrow_mut().write_function(func).unwrap();

        let main: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            19, 0, 0b00000000, fnc as u8, // Calls function 0
            19, 0, 0b00000000, fnc as u8, // Calls function 0
            19, 0, 0b00000000, fnc as u8, // Calls function 0
            // Should now have 20 on the stack

            0x00, 0x00,
            255, 255, 255, 255,
        ];
        
        VMState::run_program(vm.clone(), main).unwrap();
        let added = vm.borrow_mut().stack_pop(false).unwrap();
        let typ = vm.borrow().read_object_type(added).unwrap().id.clone();
        assert_eq!(vm.borrow().read_object(added).unwrap(), &vec![20]);
        assert_eq!(typ, "byte");
    }

    #[test]
    fn test_call_inner_function() {
        // Creates a VM
        let vm: VmRef = VMState::default();
        // Gets the Byte ObjectType

        let byte = vm.borrow().get_type(PRIM_BYTE).unwrap().clone();
        // Very basic function that returns `b2`
        let func_get_2= Function::new(vec![], Some(byte.clone()), vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x02, /* 2 */            
            21, 0, 0b00000000, 1, // Returns the b2
        ]);

        // Registers get_2
        let fnc_get_2 = vm.borrow_mut().write_function(func_get_2).unwrap();

        // Adds 5b to a given byte
        let func_add_5= Function::new(vec![byte.clone()], Some(byte), vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x03,
            19, 0, 0b00000000, fnc_get_2 as u8, // Calls function get_2
            0x09, 0x00, /* Add */ // Just adds 2 to the given arg
            0x09, 0x00, /* Add */ // Just adds 3 to the given arg

            21, 0, 0b00000000, 1, // Returns the add
        ]);

        let fnc_add_5 = vm.borrow_mut().write_function(func_add_5).unwrap();

        // the main function
        let main: Vec<u8> = vec![
            0x02, 0x00, /* PshPrim */ 0b00000000, /* Direct, Byte */ 0x05, /* 5 */
            19, 0, 0b00000000, fnc_add_5 as u8, // Calls function add_5
            19, 0, 0b00000000, fnc_add_5 as u8, // Calls function add_5
            19, 0, 0b00000000, fnc_add_5 as u8, // Calls function add_5
            // Should now have 20 on the stack

            0x00, 0x00,
            255, 255, 255, 255,
        ];
        
        VMState::run_program(vm.clone(), main).unwrap();
        let added = vm.borrow_mut().stack_pop(false).unwrap();
        let typ = vm.borrow().read_object_type(added).unwrap().id.clone();
        assert_eq!(vm.borrow().read_object(added).unwrap(), &vec![20]);
        assert_eq!(typ, "byte");
    }

    #[test]
    pub fn rust_why_do_you_do_this_i_am_in_so_much_pain_right_now() {
        // Reserved for the *worst* of Rust
    }
}