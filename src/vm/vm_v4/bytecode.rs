use std::rc::Rc;

use crate::vm::vm_v4::{object::{Object, ObjectType, PassBy}, vm_state::{ERR_NO_OBJECT, ERR_NO_OP_CODE, ERR_NO_TYPE, ERR_OPERAND, ERR_TYPE_MISH, PRIM_FN_RT, VmEmpty, VmRef, VmResult}};
// An instruction will be: `OpCode Operand[0..x]` x being the count in Operand
// The ByteCode will be shaped as `Opcode byte 1, Opcode byte 2, Operand Descriptor[x], Operand[x]`
// This means the smallest bytecode is 2 bytes, with no Operands. But each Operand costs a byte + its byte size (if direct)

/* if an input `ix[n]` is defined as:
    Direct, it creates an Object as the encoded Size (primitive, Byte, Word, Long, Int) and returns the Object (automatically on the Stack)
    Stack, it copies the relative reference from the base pointer from the Stack (auto incs) and returns the Object
    Memory, directly copies the Object from memory, does not auto inc the Ref Count
    Bus, UNIMPLEMENTED, DO *NOT* USE
 */

/* if an Output `ox[n]` is defined as:
    Direct, Errors
    Stack, Overwrites the value on the Stack with the pointer to the Object (overwritten gets auto dec)
    Memory, directly writes the Object to memory
    Bus, UNIMPLEMENTED, DO *NOT* USE
 */

 /*
    Whenever a function is called, a new "stack frame" is pushed, which at index 0 holds the "Function Return" and variables.
    The Function Return states where in the code to return too, and where the stack base used to be.
    Variables are both the local variables that are operated on, and function args that are passed in.
    As well, all Opcodes *should* use stack-indexed, rather than using the top of the CallStack

    # **Example Program** (in ASM)
    ```
    PshPrm 1b ; pushes a 1 byte to the stack
    PshPrm 500i ; pushes a 500 int to the stack
    New #7 ; creates an Object at Type Index 7 with the peramiters with 1 byte, 500 int
    ```
  */

 /*
    The Format of Assembled Programs will likely be like
    ```
    Header:
        Program Start Index
        Compiled Date
        FogCutter Version
        
    Struct Declirations:
        Struct{count:TypeID, count:TypeID}
        Struct{count:TypeID, count:TypeID}
        Struct{count:TypeID}
        Struct{count:TypeID}
    
    Program:
        Instruction
        Instruction
        Instruction
        Instruction
    ```
    The Header contains all the random bits of Meta Data in known formats
        - Where to start the program u64 of byte offsets
        - The structs that are used in the program
        - The compiled FC version, etc
    The Struct Declriations states all the structs that the program uses.
        - Uses TypeID in the basic Delclirations
        - TypeIDs gets converted into indexes when the program first starts
    The Program stores all the instructions and 

 */

pub struct OpCode {
    pub name: &'static str,                             // Name of the OpCode, used for VM level errors
    pub count: u8,                                      // This is the amount of Operands required for the OpCode, should *never* be more than 255
    pub function: fn(VmRef, Vec<Operand>) -> VmEmpty    // This is the function to run, should assume that the Vec is the size of count
}

#[derive(Debug)]
pub struct Operand {
    pub direct_value: u64,          // The direct byte code value of the Operand
    pub true_value: Option<Object>, // The read value of the Operand based on the AddressMode
    pub size: Size,                 // Size in Bytes of the Operand
    pub address_mode: AddressMode,  // Address Mode of the Operand
}

 /* Op Codes:
    - End    () This Ends the current running Program

    - New    (ix1) This creates an Object with the bytes ontop of the stack
    - PshPrm (ix1) This pushes a built-in primitive onto the Stack
    - Dup    () Duplicates the top Object, if its a pass by reference, it dups the reference

    - PshLst (ix1) This creates a Linked Array with the type of (x1)
    - SetIdx () This takes an array on top of the stack, an int, and an Object Pointer; and inserts the Object Pointer to that index (int) in the list
    - GetIdx () This takes an array on top of the stack, and an int; and pushes the Object Pointer in the array at the index (int) to the stack
    - IdxOf  () This takes an array, and an Object Pointer; and pushes the index (int) of the object to the Stack, pushes u32::MAX if not found

    - ~~Pul    (ix1) Pulls the Object at x1 to the top~~
    - Psh
    - Add    () This adds a built-in primitive at the top of the Stack
    - Sub    () This subs a built-in primitive at the top of the Stack
    - Mul    () This multiples a built-in primitive at the top of the Stack

    - JmpUnc (ix1) This uncondtinally jumps to ix1 in the program
    - CmpEq  () This compares the top 2 (primitives) values at the top of the stack, equal, and sets `jump_truth`
    - CmpLs  () This compares the top 2 (primitives) values at the top of the stack, less than, and sets `jump_truth`
    - CmpGr  () This compares the top 2 (primitives) values at the top of the stack, greater than, and sets `jump_truth`
    - JmpCon (ix1, ix2) This takes 2 inputs from the byte code, if jump_truth is true, it jumps to the bytecode at ix1, else it jumps to ix2

    - Cal    (ix1) Calls a function at at code ix1
    - Ret    () Removes all items on the stack until it reaches a "function_return" //TODO: Make this take an ix1 of how many objs to return
 */

pub fn get_opcode(opcode: u16) -> VmResult<OpCode> {
    return match opcode {
        // TODO: Finish this!
        0 => Ok(OpCode { name: "End", count: 0, function: |vm, _| -> VmEmpty {
            vm.borrow_mut().running = false;
            vm.borrow_mut().err_code = 0;
            return Ok(());
        }}),

        1 => Ok(OpCode {name: "New", count: 1, function: |vm, operands| -> VmEmpty {
            let temp = operands.get(0).ok_or(ERR_OPERAND)?;
            let typ = vm.borrow().get_type(temp.direct_value as u32)?;

            let mut bytes: Vec<u8> = vec![];
            for z in 0..typ.types.len() {
                let mut mu_vm = vm.borrow_mut();
                let pnt = mu_vm.stack_pop(false)?;
                let y = mu_vm.read_object_type(pnt)?;

                // If the types are wrong, fail
                if typ.types.get(z).ok_or(ERR_NO_TYPE)?.id != y.id {
                    return Err(ERR_TYPE_MISH);
                }

                match y.pass_by {
                    PassBy::Reference => {
                        let temp = &mut u32::to_le_bytes(pnt).to_vec();
                        for byte in temp {
                            bytes.push(*byte);
                        }
                        // Because a new reference of it is being stored in the Struct, we inc its count
                        mu_vm.inc_object_count(pnt)?;
                    },
                    PassBy::Value => {
                        for byte in mu_vm.read_object(pnt)? {
                            bytes.push(*byte);
                        }
                    }
                }
                mu_vm.dec_object_count(pnt)?;
            }
            let obj = Object::new_object(typ, vm.clone())?;
            vm.borrow_mut().write_object(obj, bytes)?;
            vm.borrow_mut().stack_push(obj, false)?;
            return Ok(());
        }}),

        2 => Ok(OpCode { name: "PshPrm", count: 1, function: |vm, operands| -> VmEmpty {
            let operand = operands.get(0).ok_or(ERR_OPERAND)?;
            let typ = vm.borrow().get_type(match operand.size {
                Size::Byte => 0,
                Size::Word => 1,
                Size::Int => 2,
                Size::Long => 3,
            } as u32)?;
            let obj = Object::new_object(typ, vm.clone())?;
            let vec: Vec<u8> = match operand.size {
                Size::Byte => u8::to_le_bytes(operand.direct_value as u8).to_vec(),
                Size::Word => u16::to_le_bytes(operand.direct_value as u16).to_vec(),
                Size::Int => u32::to_le_bytes(operand.direct_value as u32).to_vec(),
                Size::Long => u64::to_le_bytes(operand.direct_value as u64).to_vec(),
            };

            vm.borrow_mut().write_object(obj, vec)?;
            vm.borrow_mut().stack_push(obj, false)?;
            return Ok(());
        }}),

        3 => Ok(OpCode { name: "Dup", count: 0, function: |vm, _operands| -> VmEmpty {
            let z = vm.borrow_mut().stack_pop(false)?;
            
            //let mut bind = vm.borrow_mut();
            let typ = vm.borrow().read_object_type(z)?.pass_by;
            match typ {
                PassBy::Reference => {
                    vm.borrow_mut().stack_push(z, true)?;
                },

                PassBy::Value => {
                    let bytes = vm.borrow().read_object(z).clone()?.clone();
                    let obj_typ = vm.borrow().read_object_type(z)?.clone();
                    let new_obj = Object::new_object(obj_typ, vm.clone())?;
                    vm.borrow_mut().write_object(new_obj, bytes)?;

                    vm.borrow_mut().stack_push(new_obj, false)?;
                }
            }
            
            vm.borrow_mut().stack_push(z, false)?;
            //vm.borrow_mut().dec_object_count(z)?;
            return Ok(());
        }}),

        4 | 5 | 6 | 7 | 8 => Err(ERR_NO_OP_CODE), // Place holders for now

        9 => Ok(OpCode { name: "Add", count: 0, function: |vm, _operands| -> VmEmpty {
            let add_1 = get_primitive_value_unsigned(&vm)?;
            let add_2 = get_primitive_value_unsigned(&vm)?;

            let larger_type = if add_1.1.size <= add_2.1.size { add_2.1 } else { add_1.1 };
            let size = larger_type.size;
            let added = add_1.0.wrapping_add(add_2.0);

            let obj = Object::new_object(larger_type, vm.clone())?;
            let data = added.to_le_bytes().to_vec();
            vm.borrow_mut().write_object(obj, trim_value(&data, size as u8))?;
            vm.borrow_mut().stack_push(obj, false)?;
            return Ok(());
        }}),

        10 => Ok(OpCode { name: "Sub", count: 0, function: |vm, _operands| -> VmEmpty {
            let add_1 = get_primitive_value_unsigned(&vm)?;
            let add_2 = get_primitive_value_unsigned(&vm)?;

            let larger_type = if add_1.1.size <= add_2.1.size { add_2.1 } else { add_1.1 };
            let size = larger_type.size;
            let added = add_2.0.wrapping_sub(add_1.0);
            
            let obj = Object::new_object(larger_type, vm.clone())?;
            let data = added.to_le_bytes().to_vec();
            vm.borrow_mut().write_object(obj, trim_value(&data, size as u8))?;
            vm.borrow_mut().stack_push(obj, false)?;
            return Ok(());
        }}),

        11 => Ok(OpCode { name: "JmpUnc", count: 1, function: |vm, operands| -> VmEmpty {
            let pos = operands.get(0).ok_or(ERR_NO_OBJECT)?.direct_value;
            vm.borrow_mut().program_pointer = pos;
            return Ok(());
        }}),

        12 => Ok(OpCode { name: "CmpEq", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x == y;
            return Ok(());
        }}),

        13 => Ok(OpCode { name: "CmpLs", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x < y;
            return Ok(());
        }}),

        14 => Ok(OpCode { name: "CmpGr", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x > y;
            return Ok(());
        }}),
        
        15 => Ok(OpCode { name: "CmpNe", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x != y;
            return Ok(());
        }}),

        16 => Ok(OpCode { name: "CmpLsEq", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x <= y;
            return Ok(());
        }}),

        17 => Ok(OpCode { name: "CmpGrEq", count: 0, function: |vm, _operands| -> VmEmpty {
            let x = get_primitive_value_unsigned(&vm)?.0;
            let y = get_primitive_value_unsigned(&vm)?.0;
            vm.borrow_mut().jump_trueth = x >= y;
            return Ok(());
        }}),

        18 => Ok(OpCode { name: "JmpCnd", count: 2, function: |vm, operands | -> VmEmpty {
            let jmp: u64 = if vm.borrow().jump_trueth {
                operands.get(0).ok_or(ERR_NO_OBJECT)?.direct_value
            } else {
                operands.get(1).ok_or(ERR_NO_OBJECT)?.direct_value
            };

            vm.borrow_mut().program_pointer = jmp;   
            return Ok(());
        }}),

        19 => Ok(OpCode { name: "Cal", count: 2, function: |vm, operands | -> VmEmpty {
            let len = vm.borrow().stack.len();

            let data_1 = &mut vm.borrow().program_pointer.to_be_bytes().to_vec();
            let data_2 = &mut vm.borrow().base_pointer.to_be_bytes().to_vec();
            vm.borrow_mut().base_pointer = len as u64 +1;

            let typ = vm.borrow_mut().get_type(PRIM_FN_RT)?.clone();
            let obj = Object::new_object(typ, vm.clone())?;
            let mut data: Vec<u8> = vec![];
            let mut vm_mut = vm.borrow_mut();

            
            data.append(data_1);
            data.append(data_2);
            data.push(operands.get(1).ok_or(ERR_OPERAND)?.direct_value as u8);

            vm_mut.write_object(obj, data)?;
            vm_mut.stack_push(obj, false)?;

            vm_mut.program_pointer = operands.get(0).ok_or(ERR_OPERAND)?.direct_value;
            
            return Ok(());
        }}),

        20 => Ok(OpCode { name: "Ret", count: 0, function: |vm, _operands | -> VmEmpty {        
            let mut vm_mut = vm.borrow_mut();
            
            let fr = &vm_mut.get_type(PRIM_FN_RT)?;
            
            loop {
                let obj = vm_mut.stack_pop(false)?;
                let ts = vm_mut.read_object_type(obj)?;
                
                if fr == ts {
                    let mut base: u64 = 0;
                    let mut func: u64 = 0;
                    let val: Vec<u8> = (*vm_mut.read_object(obj)?).clone();
                    func = val[7] as u64;
                    func |= (val[6] as u64) << 8;
                    func |= (val[5] as u64) << 16;
                    func |= (val[4] as u64) << 24;
                    func |= (val[3] as u64) << 32;
                    func |= (val[2] as u64) << 40;
                    func |= (val[1] as u64) << 48;
                    func |= (val[0] as u64) << 56;

                    base = val[15] as u64;
                    base |= (val[14] as u64) << 8;
                    base |= (val[13] as u64) << 16;
                    base |= (val[12] as u64) << 24;
                    base |= (val[11] as u64) << 32;
                    base |= (val[10] as u64) << 40;
                    base |= (val[9] as u64) << 48;
                    base |= (val[8] as u64) << 56;
                    

                    vm_mut.base_pointer = base;
                    vm_mut.program_pointer = func + (val[16] as u64);

                    vm_mut.dec_object_count(obj)?;
                    return Ok(());
                } else {
                    vm_mut.dec_object_count(obj)?;
                }
            }
        }}),

        _ => Err(ERR_NO_OP_CODE),
    }
}

// Pops the top value off of the stack, returns error if it is not a valid primitive
fn get_primitive_value_unsigned(vm: &VmRef) -> VmResult<(u64, Rc<ObjectType>)> {
    let mut vm_mut: std::cell::RefMut<'_, super::vm_state::VMState> = vm.borrow_mut();
    let x1_obj = vm_mut.stack_pop(false)?;

    if vm_mut.is_basic_primitive(x1_obj) {
        let x1_type = vm_mut.get_type(x1_obj)?;
        let x1_data = vm_mut.read_object(x1_obj)?;
        let ret: u64;
        
        // Pretty trash, but it works
        match x1_type.size {
            1 => ret = u8::from_be_bytes([*x1_data.get(0).unwrap_or(&0)]) as u64,
            2 => ret = u16::from_le_bytes([*x1_data.get(0).unwrap_or(&0), *x1_data.get(1).unwrap_or(&0)]) as u64,
            4 => ret = u32::from_be_bytes([*x1_data.get(0).unwrap_or(&0), *x1_data.get(1).unwrap_or(&0), *x1_data.get(2).unwrap_or(&0), *x1_data.get(3).unwrap_or(&0)]) as u64,
            8 => ret = u64::from_be_bytes([*x1_data.get(0).unwrap_or(&0), *x1_data.get(1).unwrap_or(&0), *x1_data.get(2).unwrap_or(&0), *x1_data.get(3).unwrap_or(&0), *x1_data.get(4).unwrap_or(&0), *x1_data.get(5).unwrap_or(&0), *x1_data.get(6).unwrap_or(&0), *x1_data.get(7).unwrap_or(&0)]) as u64,
            _ => return Err(ERR_TYPE_MISH)
        }
        vm_mut.dec_object_count(x1_obj)?;
        
        return Ok((ret, x1_type));
    } else {
        return Err(ERR_TYPE_MISH);
    }
}

fn trim_value(vec: &Vec<u8>, bytes: u8) -> Vec<u8> {
    let mut copy = vec.clone();
    if copy.len() < bytes as usize {
        for _ in 0..((bytes as usize) -copy.len()) {
            copy.push(0);
        }
    } else if copy.len() > bytes as usize {
        copy = vec![];
        for loc in 0..bytes {
            copy.push(vec[loc as usize]);
        }
    }

    return copy;
}


#[derive(Debug)]
pub enum Size {
    Byte, // u8
    Word, // u16
    Int,  // u32
    Long, // u64
}

impl Size {
    pub fn get_size(&self) -> u8 {
        match self {
            Size::Byte => 1,
            Size::Word => 2,
            Size::Int => 4,
            Size::Long => 8,
        }
    }

    pub fn to_size(size: u8) -> Self {
        return match size {
            0 => Size::Byte,
            1 => Size::Word,
            2 => Size::Int,
            3 => Size::Word,
            _ => Size::Byte // Not happy with this, but it works
        }
    }
}

// Each of these points to "where" the value should be read/written
#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Direct, // A value hard coded within the ByteCode
    Stack,  // Refers to an offset on the Stack
    Memory, // A location in Memory
    Bus,    // The current target of the Bus
}

impl AddressMode {
    pub fn to_mode(size: u8) -> Self {
        return match size {
            0 => AddressMode::Direct,
            1 => AddressMode::Stack,
            2 => AddressMode::Memory,
            3 => AddressMode::Bus,
            _ => AddressMode::Memory,
        };
    }
}