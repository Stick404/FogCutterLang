/* Memory Mangemetment Systems:
*  * Borrow Checker: 
*      * Moves the Memory Computations to pre compile time
*      * Hard to understand for newer people
*      * Locked to Procedural Style of coding
*      * Enforces memory safety (mgiht not be required)
*      * Low resource usages during run time
* 
*  * Refrence Counting
*      * More open to other paradimes
*      * Moves the memory managment to run time
*      * Might be hard to understand
*      * Without a solid knowledge level, easy to make memory leaks
*/
/* Bytecode layout:
*   `OpCode Byte, OpCode Byte, Description[n0, n1], Description[n2...], Operand[n0], Operand[n1], Operand[n2...]`

*   Word of space for the OpCode, Byte for 2 descriptions (will use a full byte if theres only 1 remaining Operand),
*   byte/word/int/long per operand value
*
*   For example:
*   `00000000, 00000001, 00010000, 11111111, 00000000`
*   * Calls OpCode 1, Mov (00000000, 00000001)
*   * Describes Operand 1 and 2 (0000, direct byte) (0001, memory byte)
*   * Operand 1: 255 byte
*   * Operand 2: register 0
*   * End Result is this moves 255 (11111111) to register r0 with 5 bytes of program
*
*/

// Big o'l TODO list:
// TODO: Memory Reading and Writing
// TODO: Reigster Reading and Writing
// TODO: Array Register Reading and Writing
// TODO: The Stack's Pushing, Poping, and Peaking
// TODO: Byte Parsing
// TODO: OpCode Reading/Running
// TODO: Basic Math OpCodes
// TODO: Copy the OpCodes from vm_v2

// Memory is indexed by 64 bits
pub struct VmState {
    memory: Vec<u8>,
    regisers: [u64; 5],
    array_regisers: [[u64; 32]; 3],
    standard_memory: u64, // This is the point in memory where standered operations end (inclusive)
    stack_memory: u64, // This is the point in memory where stack operations end (inclusive)
    program_memory: u64, // This is the point in program where stack operations end (inclusive)
}

pub struct Operand {
    direct_value: u64, // The direct byte code value of the Operand
    true_value: u64, // The read value of the Operand 
    size: Size, // Size in Bytes of the Operand
    address_mode: AddressMode, // Address Mode of the Operand
}

pub struct OpCode {
    count: u8, // This is the amount of Operands required for the OpCode, should *never* be more than 255
    function: fn(&mut VmState, Vec<Operand>) // This is the function to run, should assume that the Vec is the size of count
}

#[derive(Debug)]
pub enum Size {
    Byte, // u8
    Word, // u16
    Int, // u32
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
}

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Direct,
    Register,
    Memory,
    Bus,
}