extern crate primitive_types;
use primitive_types::U256 as u256;
use primitive_types::U512 as u512;

mod registers;
mod memory;
mod utilities;
mod instructions;

use registers::Registers;
use registers::VecRegName;
use registers::GPRName;
use registers::FLAGSName;
use registers::IPName;

use memory::Memory;

use utilities::Utilities;

fn main() {
    let mut registers = Registers::new();
    let mut memory = Memory::new(0x40000000);

    test(&mut registers, &mut memory);
}

fn test(registers: &mut Registers, memory: &mut Memory) {
    registers.set_bit(VecRegName::XMM, 0, 127, true);
    println!("{:?}", registers.get_bit(VecRegName::XMM, 0, 127));
    println!("{:?}", registers.get_bit(VecRegName::YMM, 0, 127));
    println!("{:?}", registers.get_bit(VecRegName::ZMM, 0, 127));

    registers.set_bit(VecRegName::YMM, 0, 255, true);
    println!("{:?}", registers.get_bit(VecRegName::YMM, 0, 255));

    registers.set_bit(VecRegName::ZMM, 0, 511, true);
    println!("{:?}", registers.get_bit(VecRegName::ZMM, 0, 511));

    registers.set_bit(VecRegName::ZMM, 1, 0, true);
    registers.set_bit(VecRegName::ZMM, 1, 511, true);
    println!("{:?}", registers.get_by_sections::<u64>(VecRegName::ZMM, 1));

    println!("{:?}", registers.get_by_sections::<u32>(VecRegName::XMM, 2));
    println!("{}", registers.set_by_sections(VecRegName::XMM, 2, vec![2147483648u32, 2147483648u32, 2147483648u32, 2147483648u32]));
    println!("{:?}", registers.get_by_sections::<u32>(VecRegName::XMM, 2));

    println!("{}", registers.get_gpr_value(GPRName::RAX));
    registers.set_gpr_value(GPRName::RAX, 18446744073709486080u64);
    println!("{}", registers.get_gpr_value(GPRName::RAX));
    registers.set_gpr_value(GPRName::AL, 255u64);
    println!("{}", registers.get_gpr_value(GPRName::RAX));
    registers.set_gpr_value(GPRName::EAX, 65535u64);
    println!("{}", registers.get_gpr_value(GPRName::RAX));

    println!("{}", registers.set_by_sections(VecRegName::ZMM, 3, vec![u256::from(1), u256::from(2)]));
    println!("{:?}", registers.get_by_sections::<u256>(VecRegName::ZMM, 3));
    println!("{}", registers.set_by_sections(VecRegName::ZMM, 5, vec![u512::from(1)]));
    println!("{:?}", registers.get_by_sections::<u512>(VecRegName::ZMM, 5));

    println!("{}", registers.set_by_sections(VecRegName::XMM, 6, Utilities::f32vec_to_u32vec(vec![1.0f32, 2.0f32, 3.0f32, 4.0f32])));
    println!("{:?}", if let Some(u32vec) = registers.get_by_sections::<u32>(VecRegName::XMM, 6) {
        Some(Utilities::u32vec_to_f32vec(u32vec))
    } else { None });
    println!("{}", registers.set_by_sections(VecRegName::XMM, 7, Utilities::f64vec_to_u64vec(vec![1.0f64, 2.0f64])));
    println!("{:?}", if let Some(u64vec) = registers.get_by_sections::<u64>(VecRegName::XMM, 7) {
        Some(Utilities::u64vec_to_f64vec(u64vec))
    } else { None });

    println!("0x{:X}", memory.read::<u8>(0x40000000));
    memory.write::<u8>(0x40000000, 0x12);
    println!("0x{:X}", memory.read::<u8>(0x40000000));
    memory.write::<u16>(0x40000000, 0x1234);
    println!("0x{:X}", memory.read::<u16>(0x40000000));
    memory.write::<u32>(0x40000000, 0x12345678);
    println!("0x{:X}", memory.read::<u32>(0x40000000));
    memory.write::<u64>(0x40000000, 0x1234567887654321);
    println!("0x{:X}", memory.read::<u64>(0x40000000));
    memory.write::<u128>(0x40000000, 0x12345678876543211234567887654321);
    println!("0x{:X}", memory.read::<u128>(0x40000000));
    memory.write::<u256>(0x40000000, u256::from(0x12345678876543211234567887654321u128));
    println!("0x{:X}", memory.read::<u256>(0x40000000));
    memory.write::<u512>(0x40000000, u512::from(0x12345678876543211234567887654321u128));
    println!("0x{:X}", memory.read::<u512>(0x40000000));
    memory.write_vec::<u64>(0x40000000, vec![
        0, 1, 2, 3, 4, 5, 6, 7,
    ]);
    println!("{:?}", memory.read_vec::<u32>(0x40000000, 16));

    registers.set_by_sections::<u32>(VecRegName::XMM, 15, vec![
        0x12345678u32, 0x12345678u32, 0x12345678u32, 0x12345678u32,
    ]);
    println!("{:X?}", registers.get_by_sections::<u32>(VecRegName::XMM, 15));
    registers.set_by_selector::<u32>(VecRegName::XMM, 15, "[31:0]", 0x00000000u32);
    println!("{:X?}", registers.get_by_sections::<u32>(VecRegName::XMM, 15));
    registers.set_by_selector::<u32>(VecRegName::XMM, 15, "[MAX:64]", 0x00000000u32);
    println!("{:X?}", registers.get_by_sections::<u32>(VecRegName::XMM, 15));
}
