pub static MM1: u8 = 0b00; // (Address) Mode: Memory x1
pub static MD1: u8 = 0b01; // (Address) Mode: Direct x1
pub static MR1: u8 = 0b10; // (Address) Mode: Register x1

pub static MM2: u8 = 0b0000; // (Address) Mode: Memory x2
pub static MD2: u8 = 0b0100; // (Address) Mode: Direct x2
pub static MR2: u8 = 0b1000; // (Address) Mode: Register x2

pub static SB1: u8 = 0b000000; // Size Byte x1
pub static SW1: u8 = 0b010000; // Size Word x1
pub static SI1: u8 = 0b100000; // Size Int x1
pub static SL1: u8 = 0b110000; // Size Long x1

pub static SB2: u8 = 0b00000000; // Size Byte x2
pub static SW2: u8 = 0b01000000; // Size Word x2
pub static SI2: u8 = 0b10000000; // Size Int x2
pub static SL2: u8 = 0b11000000; // Size Long x2

pub static REG_RC: u8 = 4;
pub static REG_RE: u8 = 4;

#[derive(PartialEq, Debug)]
pub struct VmState {
    // 128+ is used for program space
    memory: [u8; 256],
    //[0]:r0 [1]:r1 [2]:r2 [3]:r3 [4]:rC [5]:rS [6]:rE [7]:rR [8]:rT
    // Un-used: rS, ~rE, rR, rT
    registers: [u64; 9],
    running: bool,
}

impl VmState {
    fn read_inc_memory(&self, slot: usize, size: &Size, count: &mut u64, mode: &AddressMode) -> u64 {
        match mode { // TODO: Maybe add Memory here? So we can specify the address size?
            AddressMode::Direct => { match size {
                    Size::Byte => *count += 1,
                    Size::Word => *count += 2,
                    Size::Int => *count += 4,
                    Size::Long => *count += 8,
                };
                return self.read_memory(slot, size);
            }
            AddressMode::Register => {*count += 1; return self.read_memory(slot, &Size::Byte);},
            _ => *count += 1,

        }
        return self.read_memory(slot, size);
    }
    
    pub fn read_memory(&self, slot: usize, size: &Size) -> u64 {
        let mut ret: u64;
        match size {
            Size::Byte => {
                ret = self.read_mem_safe(slot) as u64;
            },
            Size::Word => {
                ret = self.read_mem_safe(slot) as u64;
                ret |= (self.read_mem_safe(slot +1) as u64) << 8;
            },
            Size::Int => {
                ret = self.memory[slot] as u64;
                ret |= (self.read_mem_safe(slot +1) as u64) << 8;
                ret |= (self.read_mem_safe(slot +2) as u64) << 16;
                ret |= (self.read_mem_safe(slot +3) as u64) << 24;
            },
            Size::Long => {
                ret = self.memory[slot] as u64;
                ret |= (self.read_mem_safe(slot +1) as u64) << 8;
                ret |= (self.read_mem_safe(slot +2) as u64) << 16;
                ret |= (self.read_mem_safe(slot +3) as u64) << 24;
                ret |= (self.read_mem_safe(slot +4) as u64) << 32;
                ret |= (self.read_mem_safe(slot +5) as u64) << 40;
                ret |= (self.read_mem_safe(slot +6) as u64) << 48;
                ret |= (self.read_mem_safe(slot +7) as u64) << 56;
            }
        }
        return ret;
    }
    fn read_mem_safe(&self, slot: usize) -> u8 {
        return match self.memory.get(slot) {
            None => 0,
            Some(x) => *x
        }
    }

    pub fn write_memory(&mut self, slot: usize, value: u64, size: Size){
        //println!("writting to memory slot {slot} with {value} as {size:?}");
        match size {
            Size::Byte => {
                self.memory[slot] = value as u8
            },
            Size::Word => {
                self.memory[slot] = (value & 0xFF) as u8;
                self.memory[slot +1] = (value & 0xFF00 >> 8) as u8;
            },
            Size::Int => {
                self.memory[slot] = (value & 0xFF) as u8;
                self.memory[slot +1] = (value & 0xFF00 >> 8) as u8;
                self.memory[slot +2] = (value & 0xFF0000 >> 16) as u8;
                self.memory[slot +3] = (value & 0xFF000000 >> 24) as u8;
            },
            Size::Long => {
                self.memory[slot] = (value & 0xFF) as u8;
                self.memory[slot +1] = (value & 0xFF00 >> 8) as u8;
                self.memory[slot +2] = (value & 0xFF0000 >> 16) as u8;
                self.memory[slot +3] = (value & 0xFF000000 >> 24) as u8;
                self.memory[slot +4] = (value & 0xFF00000000 >> 32) as u8;
                self.memory[slot +5] = (value & 0xFF0000000000 >> 40) as u8;
                self.memory[slot +6] = (value & 0xFF000000000000 >> 48) as u8;
                self.memory[slot +7] = (value & 0xFF00000000000000 >> 56) as u8;
            }
        }
    }

    pub fn read_register(&self, register: usize) -> u64 {
        return match self.registers.get(register) {
            None => 0,
            Some(x) => *x
        };
    }

    pub fn write_register(&mut self, register: usize, value: u64) {
        //println!("writting to register {register} with {value}");
        match register {
            _ => self.registers[register] = value
        }
    }

    pub fn write_program(&mut self, program: &Vec<u8>) -> Result<(), &'static str> {
        if program.iter().size_hint().0 > 120 {
            return Result::Err("TooLarge"); //TODO: make the program cap larger/changable
        }
        let mut i: usize = 0;
        for byte in program.iter() {
            self.memory[128 +i] = *byte;
            i += 1;
        }
        return Result::Ok(());
    }

    // TODO: Make these `bool`s into Result<(), &'static str> for better errors
    pub fn run_program(&mut self, program: &Vec<u8>) -> bool {
        if self.write_program(program).is_err() {
            return false;
        }
        return self.run_set_program();
    }

    pub fn run_set_program(&mut self) -> bool {
        self.running = true;

        while self.running {
            let mut local_count = self.read_register(REG_RC as usize);
            println!("count: {local_count}");

            let op_code: u8 = match self.read_byte(local_count as usize) {
                Result::Err(_x) => {self.write_register(REG_RE as usize, 2); return false} //Error Code 2: Ended wrongly!
                Result::Ok(x) => x
            };
            println!("op code: {op_code}");
            local_count += 1;
            
            let description: u8 = match self.read_byte(local_count as usize) {
                Result::Err(_x) => {self.write_register(REG_RE as usize, 2); return false} //Error Code 2: Ended wrongly!
                Result::Ok(x) => x
            };
            local_count += 1;
            
            let x1_mode = match description & 0b11 {
                0 => AddressMode::Memory,
                1 => AddressMode::Direct,
                2 => AddressMode::Register,
                3 => AddressMode::UNUSED,
                _ => {self.write_register(REG_RE as usize, 3); return false},  //Error Code 3: Could not parse description!
            };

            let x2_mode = match (description & 0b1100) >> 2 {
                0 => AddressMode::Memory,
                1 => AddressMode::Direct,
                2 => AddressMode::Register,
                3 => AddressMode::UNUSED,
                _ => {self.write_register(REG_RE as usize, 3); return false},  //Error Code 3: Could not parse description!
            };

            let x1_size = match (description & 0b110000) >> 4 {
                0 => Size::Byte,
                1 => Size::Word,
                2 => Size::Int,
                3 => Size::Long,
                _ => {self.write_register(REG_RE as usize, 3); return false},  //Error Code 3: Could not parse description!
            };

            let x2_size = match (description & 0b11000000) >> 6 {
                0 => Size::Byte,
                1 => Size::Word,
                2 => Size::Int,
                3 => Size::Long,
                _ => {self.write_register(REG_RE as usize, 3); return false},  //Error Code 3: Could not parse description!
            };

            let x1_value = self.read_inc_memory(local_count as usize, &x1_size, &mut local_count, &x1_mode);
            let x2_value = self.read_inc_memory(local_count as usize, &x2_size, &mut local_count, &x2_mode);

            println!("State: {self:?}");

            let x1_true_value: u64 = match x1_mode {
                AddressMode::Direct => x1_value,
                AddressMode::Memory => self.read_memory(x1_value as usize, &x1_size),
                AddressMode::Register => self.read_register(x1_value as usize),
                _ => return false,
            };
        
            let x2_true_value: u64 = match x2_mode {
                AddressMode::Direct => x2_value,
                AddressMode::Memory => self.read_memory(x2_value as usize, &x2_size),
                AddressMode::Register => self.read_register(x2_value as usize),
                _ => return false,
            };
            self.write_register(REG_RC as usize, local_count); // Update the rC with the count
            if !self.op_code(op_code, x1_value, x2_value, x1_true_value, x2_true_value, x1_mode, x2_mode, x1_size, x2_size) {
                return false;
            }

            if self.read_register(REG_RE as usize) == 1 { // We ended safely and can now exit the program and end everything
                //for mem in self.memory.iter_mut() {*mem = 0} //resets everything
                //for reg in self.registers.iter_mut() {*reg = 0}
                self.running = false;
                return true;
            }
            if !self.running { // Uh oh! Something went wrong and we had to end early. Keep the memory/registries open for debug
                return false;
            }
        }
        return false;
    }

    fn read_byte(&self, loc: usize) -> Result<u8, &'static str> {
        return match self.memory.get(loc) {
            Some(x) => Result::Ok(*x),
            None => Result::Err("RanOut")
        };
    }

    fn op_code(&mut self, op_code: u8, x1_value: u64, x2_value: u64, x1_true_value: u64, x2_true_value: u64, x1_mode: AddressMode, x2_mode: AddressMode, x1_size: Size, x2_size: Size) -> bool {
        println!("Op: {op_code}, raw x1: {x1_value}, raw x2: {x2_value}, x1: {x1_true_value}, x2: {x2_true_value}");
        match op_code {
            0 => /* Halt */ self.running = false, // Should error the program after op code eval
            1 => /* Mov */ {
                println!("Moving {x1_true_value} to {x2_mode:?}:{x2_value} as {x1_size:?}");
                match x2_mode {
                    AddressMode::Memory => self.write_memory(x2_value as usize, x1_true_value, x1_size),
                    AddressMode::Register => self.write_register(x2_value as usize, x1_true_value),
                    _ => return false,
                }
                return true;
            },
            2 => /* Add */ self.write_register(2, x1_true_value + x2_true_value),
            3 => /* Sub */ self.write_register(2, x1_true_value - x2_true_value),
            4 => /* Mul */ self.write_register(2, x1_true_value * x2_true_value),
            5 => /* Div */ self.write_register(2, x1_true_value / x2_true_value),

            _ => return false,
        }
        return true;
    }
}

impl Default for VmState {
    fn default() -> Self {
        VmState {
            memory: [0; 256],
            registers: [0, 0, 0, 0, 128, 0, 0, 0, 0],
            running: false,
        }
    }
}

#[derive(Debug)]
pub enum Size {
    Byte, // u8
    Word, // u16
    Int, // u32
    Long, // u64
}

#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Memory,
    Direct,
    Register,
    UNUSED, // TODO: pick a use for this (bus?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_read_write() {
        let mut mem = VmState::default();

        mem.write_memory(0, 255, Size::Byte);
        assert_eq!(mem.read_memory(0, &Size::Byte), 255);

        mem.write_memory(0, 65535, Size::Word);
        assert_eq!(mem.read_memory(0, &Size::Word), 65535);
        
        mem.write_memory(0, 4294967295, Size::Int);
        assert_eq!(mem.read_memory(0, &Size::Int), 4294967295);

        mem.write_memory(0, 18446744073709551615, Size::Long);
        assert_eq!(mem.read_memory(0, &Size::Long), 18446744073709551615);
    }

    #[test]
    fn program_run_mov() { // Tests the Mov OpCode
        let mut vm = VmState::default();
        let program: Vec<u8> = vec![
            1, MD1|MR2, 1, 0, // Moves value 1 to r0
            1, MR1|MR2, 0, 1, // Moves r0 to r1
            1, MR1|MM2, 1, 0, // Moves r1 to memory 0
            1, MM1|MR2, 0, 2, // Moves mem0 to r2
            1, MR1|MR2, 1, REG_RE // Moves r1 to rE, successfully ending the program
        ];
        assert!(vm.run_program(&program));
        assert_eq!(vm.read_register(0), 1);
        assert_eq!(vm.read_register(1), 1);
        assert_eq!(vm.read_register(2), 1);
        assert_eq!(vm.read_memory(0, &Size::Byte), 1);
    }

    #[test]
    fn program_sizes(){ // Tests the Sizes
        let mut vm = VmState::default();
        let program: Vec<u8> = vec![
            1, MD1|MR2|SW1, 0b11111111, 0b11111111, 0, // Moves value word max to r0
            1, MR1|MM2|SW1, 0, 5, // Moves r0 to mem5

            1, MD1|MR2|SI1, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 1, // Moves value int max to r1
            1, MR1|MM2|SI1, 1, 10, // Moves r1 to mem10

            1, MD1|MR2|SL1, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 0b11111111, 2, // Moves value long max to r2
            1, MR1|MM2|SL1, 2, 15, // Moves r2 to mem15

            1, MD1|MR2, 1, REG_RE // Moves value 1 to rE, successfully ending the program
        ];
        assert!(vm.run_program(&program));
        assert_eq!(vm.read_register(0), 65535);
        assert_eq!(vm.read_register(1), 4294967295);
        assert_eq!(vm.read_register(2), 18446744073709551615);

        assert_eq!(vm.read_memory(5, &Size::Word), 65535);
        assert_eq!(vm.read_memory(10, &Size::Int), 4294967295);
        assert_eq!(vm.read_memory(15, &Size::Long), 18446744073709551615);
    }

    #[test]
    fn program_math() { // Tests the Math OpCodes
        let mut vm = VmState::default();
        let program: Vec<u8> = vec![
            1, MD1|MR2, 2, 0, // Moves value 1 to r0
            1, MD1|MR2, 3, 1, // Moves value 3 to r1

            2, MR1|MR2, 0, 1, // Adds values in r0 r1 to r2
            1, MR1|MM2, 2, 0, // Moves r2 to mem0

            3, MR1|MR2, 1, 0, // Sub values in r0 r1 to r2
            1, MR1|MM2, 2, 1, // Moves r2 to mem1

            4, MR1|MR2, 1, 0, // Mults values in r0 r1 to r2
            1, MR1|MM2, 2, 2, // Moves r2 to mem2

            5, MR1|MR2, 1, 0, // Divs values in r0 r1 to r2
            1, MR1|MM2, 2, 3, // Moves r2 to mem3

            1, MD1|MR2, 1, REG_RE // Moves value 1 to rE, successfully ending the program
        ];
        assert!(vm.run_program(&program));
        assert_eq!(vm.read_memory(0, &Size::Byte), 5); // Adds to 5
        assert_eq!(vm.read_memory(1, &Size::Byte), 1); // Subs to 1
        assert_eq!(vm.read_memory(2, &Size::Byte), 6); // Muts to 6
        assert_eq!(vm.read_memory(3, &Size::Byte), 1); // Divs to 1?
    }
}