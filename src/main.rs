use crate::vm::{vm_v2};

pub mod vm;

fn main() {
    println!("Hello world!");
    
    let mut mem = vm_v2::VmState::default();

    let mut numb: u64;

    mem.write_memory(0, 255, vm_v2::Size::Byte);
    numb = mem.read_memory(0, &vm_v2::Size::Byte);
    println!("{mem:?}");
    println!("`numb` is: {numb}");

    mem.write_memory(0, 65535, vm_v2::Size::Word);
    numb = mem.read_memory(0, &vm_v2::Size::Word);
    println!("{mem:?}");
    println!("`numb` is: {numb}");
    
    mem.write_memory(0, 65535, vm_v2::Size::Int);
    numb = mem.read_memory(0, &vm_v2::Size::Int);
    println!("{mem:?}");
    println!("`numb` is: {numb}");

    mem.write_memory(0, 18446744073709551615, vm_v2::Size::Long);
    numb = mem.read_memory(0, &vm_v2::Size::Long);
    println!("{mem:?}");
    println!("`numb` is: {numb}");

    //let mut mem = vm_v1::Memory::default();
    /*let program: Vec<u8> = vec![0b00000001, 0b01010110, 0b00000000, 0b00000111,
                                0b00000001, 0b01011010, 0b00000001, 0b00000000,
                                0b00000010, 0b01010110, 0b00000001, 0b00000001,
                                0b00000001, 0b01011000, 0b00000000, 0b00000001,
                                0b00000001, 0b01011010, 0b00000010, 0b00000001,
                                0b00000011, 0b01010110, 0b00000010, 0b00000110];
    */
    //vm_v1::run_program(program, &mut mem);
    //println!("{mem:?}");
}

// Opcodes required:
// MOV (00000001) [x1, x2], (move) moves a value from one location x2 to location x1
// ADDUN (00000010) [x1, x2], (add unsigned) adds the value from location x2 to location x1, overwrites x1
// SUBUN (00000011) [x1, x2], (subtract unsigned) subtracts the value from location x2 from location x1, overwrites x1