use crate::vm::vm_v4::{object::Object, vm_state::VMState};
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

/* Op Codes:
    - Mov (ix1, ox2) This copies the value at x1 to x2, the AddressMode of x1 states where to read it, and the AddressMode of x2 states what to read/write
    - New (ix1, ox2) This creates an Object with VMState::get_type(&self, x1), and writes the pointer to x2
    - Wrt (ox1, ix2, ix3) This writes to an Object at x1, with the "array" of x2, size defined by x3
    - Add (ix1, ix2, ox3) This adds a built-in primitive in x1 and x2, and outputs to x3
    - Sub (ix1, ix2, ox3) This subs a built-in primitive in x1 and x2, and outputs to x3
    - Mul (ix1, ix2, ox3) This multiples a built-in primitive in x1 and x2, and outputs to x3
    - Jmp (ix1)
    - JmpGr (ix1, ix2, ix3)
 */
pub struct OpCode {
    pub name: &'static str, // Name of the OpCode
    pub count: u8, // This is the amount of Operands required for the OpCode, should *never* be more than 255
    pub function: fn(&mut VMState, Vec<Operand>) -> bool // This is the function to run, should assume that the Vec is the size of count
}

#[derive(Debug)]
pub struct Operand {
    pub direct_value: u64, // The direct byte code value of the Operand
    pub true_value: Object, // The read value of the Operand based on the AddressMode
    pub size: Size, // Size in Bytes of the Operand
    pub address_mode: AddressMode, // Address Mode of the Operand. If this is `AddressMode::Memory` then we assume the size is Size::Int
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