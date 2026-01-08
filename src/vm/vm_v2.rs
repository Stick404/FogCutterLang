
#[derive(PartialEq, Debug)]
pub struct VmState {
    memory: [u8; 256],
    //[0]:r0 [1]:r1 [2]:r2 [3]:r3 [4]:rC [5]:rS [6]:rE [7]:rR [8]:rT
    // Unimplt: rC, rS, rE, rR, rT
    registers: [u64; 9], 
}

impl VmState {

    pub fn read_memory(&self, slot: usize, size: Size) -> u64 {
        let mut ret: u64;
        match size {
            Size::Byte => {
                println!("Byte!");
                ret = self.memory[slot] as u64;
            },
            Size::Word => {
                ret = self.memory[slot] as u64;
                ret |= (self.memory[slot +1] as u64) << 8;
            },
            Size::Int => {
                ret = self.memory[slot] as u64;
                ret |= (self.memory[slot +1] as u64) << 8;
                ret |= (self.memory[slot +2] as u64) << 16;
                ret |= (self.memory[slot +3] as u64) << 24;
            },
            Size::Long => {
                ret = self.memory[slot] as u64;
                ret |= (self.memory[slot +1] as u64) << 8;
                ret |= (self.memory[slot +2] as u64) << 16;
                ret |= (self.memory[slot +3] as u64) << 24;
                ret |= (self.memory[slot +4] as u64) << 32;
                ret |= (self.memory[slot +5] as u64) << 40;
                ret |= (self.memory[slot +6] as u64) << 48;
                ret |= (self.memory[slot +7] as u64) << 56;
            }
        }
        return ret;
    }

    pub fn write_memory(&mut self, slot: usize, value: u64, size: Size){
        println!("writting to memory slot {slot} with {value} as {size:?}");
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
        return self.registers[register];
    }

    pub fn write_registor(&mut self, register: usize, value: u64){
        println!("writting to register {register} with {value}");
        match register {
            _ => self.registers[register] = value
        }
    }
}

impl Default for VmState {
    fn default() -> Self {
        VmState {
            memory: [0; 256],
            registers: [0, 0, 0, 0, 0, 0, 0, 0, 0],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_read_write() {
        let mut mem = VmState::default();
        let mut numb: u64;

        mem.write_memory(0, 255, Size::Byte);
        numb = mem.read_memory(0, Size::Byte);
        assert_eq!(numb, 255);

        mem.write_memory(0, 65535, Size::Word);
        numb = mem.read_memory(0, Size::Word);
        assert_eq!(numb, 65535);
        
        mem.write_memory(0, 4294967295, Size::Int);
        numb = mem.read_memory(0, Size::Int);  
        assert_eq!(numb, 4294967295);

        mem.write_memory(0, 18446744073709551615, Size::Long);
        numb = mem.read_memory(0, Size::Long);
        assert_eq!(numb, 18446744073709551615);
    }
}