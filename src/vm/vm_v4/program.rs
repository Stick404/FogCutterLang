use std::{time::{SystemTime, UNIX_EPOCH}, vec};

use crate::vm::vm_v4::object::ObjectType;


pub struct Program {
    // Header info:
    program_start: u64, // When the program section starts (AKA: bytes from byte 0 in this file)
    compiled_date: u64, // The SystemTime of when the program was compiled, written as second since Unix Epoch
    fc_major: u16,
    fc_minor: u16,
    fc_bug_fix: u16,
    // Structs
    structs: Vec<ObjectType>, // All structs that are made in the Program
    // Code
    code: Vec<u8>, // The Bytecode to run
}


impl Program {
    pub fn new(fc_major: u16, fc_minor: u16, fc_bug_fix: u16, code: Vec<u8>, structs: Vec<ObjectType>) -> Self {
        return Program { program_start: 0, compiled_date: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(), fc_major, fc_minor, fc_bug_fix, structs: structs, code: code };
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut sers: Vec<u8> = vec![];

        // Header info:
        let prog_start = &mut u64::MAX.to_be_bytes().to_vec();
        // This will be over written
        sers.append(prog_start);

        let prog_date = &mut self.compiled_date.to_be_bytes().to_vec();
        sers.append(prog_date);

        let prog_major = &mut self.fc_major.to_be_bytes().to_vec();
        sers.append(prog_major);

        let prog_minor = &mut self.fc_minor.to_be_bytes().to_vec();
        sers.append(prog_minor);

        let prog_fix = &mut self.fc_bug_fix.to_be_bytes().to_vec();
        sers.append(prog_fix);

        // Struts:
        let mut struct_byte_count: u64 = 0;
        for stc in &self.structs {
            let type_serz = &mut stc.to_bytes();
            struct_byte_count += type_serz.len() as u64;
            sers.append(type_serz);
        }

        for code in &self.code {
            sers.push(*code);
        }

        // Sets the value of program_start
        for (count, _) in (0..8 as usize).enumerate() {
            sers[count] = (struct_byte_count << count*8) as u8;
        }
    
        return sers;
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        return Self { program_start: 0, compiled_date: 0, fc_major: 0, fc_minor: 0, fc_bug_fix: 0, structs: vec![], code: vec![] };
    }
}