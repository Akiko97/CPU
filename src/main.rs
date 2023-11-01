mod registers;
use registers::Registers;

fn main() {
    let mut registers = Registers::new();

    registers.set_bit("xmm", 0, 127, true);
    println!("{:?}", registers.get_bit("xmm", 0, 127));
    println!("{:?}", registers.get_bit("ymm", 0, 127));
    println!("{:?}", registers.get_bit("zmm", 0, 127));

    registers.set_bit("ymm", 0, 255, true);
    println!("{:?}", registers.get_bit("ymm", 0, 255));

    registers.set_bit("zmm", 0, 511, true);
    println!("{:?}", registers.get_bit("zmm", 0, 511));

    registers.set_bit("zmm", 1, 0, true);
    registers.set_bit("zmm", 1, 511, true);
    println!("{:?}", registers.get_sections::<u64>("zmm", 1));

    println!("{:?}", registers.get_sections::<u32>("xmm", 2));
    println!("{}", registers.set_by_sections("xmm", 2, vec![2147483648u32, 2147483648u32, 2147483648u32, 2147483648u32]));
    println!("{:?}", registers.get_sections::<u32>("xmm", 2));
}
