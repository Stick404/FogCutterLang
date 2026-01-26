use crate::vm::vm_v4::{object::{Object, PassBy}, vm_state::VmRef};
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
 
/* OLD Op Codes:
    - Mov (ix1, ox2) This copies the value at x1 to x2, the AddressMode of x1 states where to read it, and the AddressMode of x2 states what to read/write
    - New (ix1, ox2) This creates an Object with VMState::get_type(&self, x1), and writes the pointer to x2
    - Wrt (ox1, ix2, ix3) This writes to an Object at x1, with the "array" of x2, size defined by x3
    - Add (ix1, ix2, ox3) This adds a built-in primitive in x1 and x2, and outputs to x3
    - Sub (ix1, ix2, ox3) This subs a built-in primitive in x1 and x2, and outputs to x3
    - Mul (ix1, ix2, ox3) This multiples a built-in primitive in x1 and x2, and outputs to x3
    - Jmp (ix1)
    - JmpGr (ix1, ix2, ix3)
 */

 /* Op Codes:
    - New    (ix1) This creates an Object with the bytes ontop of the stack
    - PshPrm (ix1) This pushes a built-in primitive onto the Stack
    - LrdMem (ix1) This pushes an Object in memory onto the stack (auto incs loaded Object)
    - PutMem () This puts an Object on the stack into Memory, pushes the pointer back onto the Stack
    - Dup    () Duplicates the top Object, if its a pass by reference, it dups the reference
    - Pul    (ix1) Pulls the Object at x1 to the top
    - Add    () This adds a built-in primitive at the top of the Stack
    - Sub    () This subs a built-in primitive at the top of the Stack
    - Mul    () This multiples a built-in primitive at the top of the Stack
    - Jmp    (ix1)
    - JmpGr  (ix1, ix2, ix3)
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

pub struct OpCode {
    pub name: &'static str, // Name of the OpCode
    pub count: u8, // This is the amount of Operands required for the OpCode, should *never* be more than 255
    pub function: fn(VmRef, Vec<Operand>) -> Option<()> // This is the function to run, should assume that the Vec is the size of count
}

#[derive(Debug)]
pub struct Operand {
    pub direct_value: u64, // The direct byte code value of the Operand
    pub true_value: Option<Object>, // The read value of the Operand based on the AddressMode
    pub size: Size, // Size in Bytes of the Operand
    pub address_mode: AddressMode, // Address Mode of the Operand. If this is `AddressMode::Memory` then we assume the size is Size::Int
}


pub fn get_opcode(opcode: u16) -> Option<OpCode> {
    return match opcode {
        // TODO: Finish this!
        0 => Some(OpCode {name: "New", count: 1, function: |vm, operands| -> Option<()> {
            let temp = operands.get(0)?;
            let typ = vm.borrow().get_type(temp.direct_value as u32)?;

            let mut bytes: Vec<u8> = vec![];
            for z in 0..typ.types.len() {
                let mut mu_vm = vm.borrow_mut();
                let pnt = mu_vm.stack_pop(false)?;
                let y = mu_vm.read_object_type(pnt)?;

                // If the types are wrong, fail
                if typ.types.get(z)?.id != y.id {
                    return None;
                }

                match y.pass_by {
                    PassBy::Reference => {
                        let temp = &mut u32::to_le_bytes(pnt).to_vec();
                        for byte in temp {
                            bytes.push(*byte);
                        }
                        // Because a new reference of it is being stored in the Struct, we inc its count
                        mu_vm.inc_object_count(pnt);
                    },
                    PassBy::Value => {
                        for byte in mu_vm.read_object(pnt)? {
                            bytes.push(*byte);
                        }
                    }
                }
                mu_vm.dec_object_count(pnt);
            }
            let obj = Object::new_object(typ, vm.clone())?;
            vm.borrow_mut().write_object(obj, bytes);
            vm.borrow_mut().stack_push(obj, false);
            return Some(());
        }}),
        1 => Some(OpCode { name: "PshPrm", count: 1, function:  |vm, operands| -> Option<()> {
            let operand = operands.get(0)?;
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

            vm.borrow_mut().write_object(obj, vec);
            vm.borrow_mut().stack_push(obj, false);
            return Some(());
        }}),
        _ => None,
    }
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