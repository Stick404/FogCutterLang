#[cfg(test)]
mod tests {
    use crate::vm::vm_v3::*;

    #[test]
    fn memory_register_read_write() {
        let mut state = VmState::new(64, 64, 64);

        assert!(state.write(0xF5, 0, &AddressMode::Memory, &Size::Byte).is_ok());
        assert!(state.write(1, 0, &AddressMode::Register, &Size::Byte).is_ok());
        assert_eq!(state.read(0, &AddressMode::Memory, &Size::Byte).unwrap_or(50), 0xF5);
        assert_eq!(state.read(0, &AddressMode::Register, &Size::Byte).unwrap_or(50), 1);

        assert!(state.write(0xF00F, 0, &AddressMode::Memory, &Size::Word).is_ok());
        assert_eq!(state.read(0, &AddressMode::Memory, &Size::Word).unwrap_or(50), 0xF00F);

        assert!(state.write(0xF00F00FF, 0, &AddressMode::Memory, &Size::Int).is_ok());
        assert_eq!(state.read(0, &AddressMode::Memory, &Size::Int).unwrap_or(50), 0xF00F00FF);
        assert_ne!(state.read(0, &AddressMode::Memory, &Size::Word).unwrap_or(50), 0xFF00F00F)
    }

    #[test]
    fn program_true() { // Tests the very basic of starting and running a program, named after Bash's `true`
        let program: Vec<u8> = vec![
            1, 0, 0b01000000, 1, REG_RE,
        ];

        let mut vm = VmState::new(64, 64, program.len() as u64);

        match vm.set_run_program(program) {
            Ok(_x) => {},
            Err(x) => {eprintln!("Got error: {x}"); assert_ne!(x, 0)}
        };
    }

    #[test]
    fn program_mov() { // Tests the OpCode Mov
        let program: Vec<u8> = vec![
            1, 0, 0b01000000, 0, 0, // Moves 0 to r0
            1, 0, 0b10000000, 50, 0, // Moves 50 to mem0
            1, 0, 0b01001000, 0, 1, // Moves mem0 to r1

            1, 0, 0b01000000, 0, REG_RE // Moves 0 to r6, End
        ];

        let mut vm = VmState::new(64, 64, program.len() as u64);

        match vm.set_run_program(program) {
            Ok(_x) => {},
            Err(x) => {eprintln!("Got error: {x}"); assert_ne!(x, 0)}
        };
        assert_eq!(vm.read(0, &AddressMode::Memory, &Size::Byte).unwrap_or(0), 50);
        assert_eq!(vm.read(1, &AddressMode::Register, &Size::Byte).unwrap_or(0), 50)
    }

    #[test]
    fn program_usigned_math() { // Tests the Unsigned Math OpCodes
        let program: Vec<u8> = vec![
            1, 0, 0b01000000, 10, 0, // Moves 1 to r0
            1, 0, 0b01000000, 2, 1, // Moves 2 to r1
            
            2, 0, 0b01000100, 0b00000100, 0, 1, 2, // Adds r0 and r1 and puts the result in r2 (12)
            3, 0, 0b01000100, 0b00000100, 1, 0, 3, // Subs r1 from r2 and puts the result in r3 (8)

            1, 0, 0b01000000, 0, REG_RE // Moves 0 to r6, Ends
        ];

        let mut vm = VmState::new(64, 64, program.len() as u64);
    
        match vm.set_run_program(program) {
            Ok(_x) => {},
            Err(x) => {eprintln!("Got error: {x}"); assert!(false)}
        };
        assert_eq!(vm.read(2, &AddressMode::Register, &Size::Byte).unwrap_or(0), 12);
        assert_eq!(vm.read(3, &AddressMode::Register, &Size::Byte).unwrap_or(0), 8);
    }
}