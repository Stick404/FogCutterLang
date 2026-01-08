pub fn run_program(program: Vec<u8>, mem: &mut Memory) -> bool {
    if program.len() % 2 != 0 || program.len() < 2 {
        return false; // Invalid size
    }
    
    let mut op_code: u8 = 0;
    let mut just_checked_op = false;

    let mut x1_address_mode: u8 = 1;
    let mut x2_address_mode: u8 = 1;

    let mut x1_size: u8 = 1;
    let mut x2_size: u8 = 1;

    let mut x1_value: u64 = 0;
    let mut x2_value: u64 = 0;
    let mut x1_byte_count: u8 = 0;
    let mut x2_byte_count: u8 = 0;

    for (i, val) in program.iter().enumerate() {
        if i % (x1_size + x2_size +2) as usize == 0 {
            op_code = *val;
            just_checked_op = true;
        } else if just_checked_op {
            just_checked_op = false;
            let val: u8 = *val;

            //println!("ARGS");
            x1_address_mode = val & 0b00000011;
            x2_address_mode = (val & 0b00001100) >> 2;

            x1_size = (val & 0b00110000) >> 4;
            x2_size = (val & 0b11000000) >> 6;
            //println!("x1: adress mode: {x1_address_mode}, byte size: {x1_size}");
            //println!("x2: adress mode: {x2_address_mode}, byte size: {x2_size}");
        } else if x1_byte_count <= x1_size.into() && x2_byte_count <= x2_size.into() {
            //println!("reading bytes!");
            let val: u8 = *val;
            if x1_byte_count < x1_size.into() {
               //println!("x1 value: {val}");
               x1_value = x1_value | ((val as u64) << x1_byte_count); 
               x1_byte_count += 1;
            } else if x2_byte_count < x2_size.into() {
                //println!("x2 value: {val}");
               x2_value = x2_value | ((val as u64) << x2_byte_count); 
               x2_byte_count += 1;
            }

            if x1_byte_count == x1_size.into() && x2_byte_count == x2_size.into() {
                //println!("Running OpCode!");
                if !use_op_code(mem, op_code, x1_value, x2_value, x1_address_mode, x2_address_mode) {
                    return false;
                }

                x1_value = 0;
                x2_value = 0;
                x1_byte_count = 0;
                x2_byte_count = 0;
            }
        }
        
    }
    return true;
}

fn use_op_code(mem: &mut Memory, op_code: u8, x1: u64, x2: u64, x1_address_mode: u8, x2_address_mode: u8) -> bool {
    let x2_value = match x2_address_mode {
        0 => mem.get_memory(x2 as usize),
        1 => x2 as u8,
        2 => mem.get_register(x2 as usize),
        _ => return false
    };

    match op_code {
        1 => {
            match x1_address_mode {
                0 => mem.write_memory(x1 as usize, x2_value as u8),
                2 => mem.write_registor(x1 as usize, x2_value as u8),
                _ => return false
            };
            return true;
        } 2 => {
            match x1_address_mode {
                0 => { mem.write_memory(x1 as usize, mem.get_memory(x1 as usize) + x2_value as u8) },
                2 => { mem.write_registor(x1 as usize, mem.get_register(x1 as usize) + x2_value as u8) },
                _ => return false
            };
            return true;
        } 3 => {
            match x1_address_mode {
                0 => { mem.write_memory(x1 as usize, mem.get_memory(x1 as usize) - x2_value as u8) },
                2 => { mem.write_registor(x1 as usize, mem.get_register(x1 as usize) - x2_value as u8) },
                _ => return false
            };
            return true;
        }
        _ => return false,
    };
}

#[derive(PartialEq, Debug)]
pub struct Memory {
    memory: [u8; 256],
    register: [u8; 6]
}

impl Memory {
    fn get_memory(&self, slot: usize) -> u8 {
        return self.memory[slot];
        //return 0;
    }

    fn get_register(&self, register: usize) -> u8 {
        return self.register[register];
    }

    fn write_memory(&mut self, slot: usize, value: u8){ // TODO: make this do fucky bit shifting stuff, and take an u64
        println!("writting to memory slot {slot} with {value}");
        self.memory[slot] = value;
    }

    fn write_registor(&mut self, register: usize, value: u8){ // TODO: make this do fucky bit shifting stuff, and take an u64
        println!("writting to register {register} with {value}");
        self.register[register] = value;
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            memory: [0; 256],
            register: [0; 6]
        }
    }
}