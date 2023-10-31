extern crate bit_vec;
use bit_vec::BitVec;

struct SIMDRegister {
    bits: BitVec,
}

pub struct Registers {
    simd_registers: [SIMDRegister; 16],
}

impl SIMDRegister {
    fn new(size: usize) -> Self {
        SIMDRegister {
            bits: BitVec::from_elem(size, false),
        }
    }

    fn set_bit(&mut self, position: usize, value: bool) {
        self.bits.set(position, value);
    }

    fn get_bit(&self, position: usize) -> bool {
        self.bits[position]
    }

    // fn get_32bit_sections(&self) -> Vec<u32> {
    //     let mut sections = Vec::new();
    //     for i in (0..self.bits.len()).step_by(32) {
    //         let mut section_value: u32 = 0;
    //         for j in 0..32 {
    //             if self.bits[i + j] {
    //                 section_value |= 1 << j;
    //             }
    //         }
    //         sections.push(section_value);
    //     }
    //     sections
    // }

    fn get_sections<T>(&self) -> Vec<T>
        where
            T: From<u8> + std::ops::Shl<usize, Output = T> + std::ops::BitOr<Output = T> + Copy,
    {
        let mut sections = Vec::new();
        let type_bits = std::mem::size_of::<T>() * 8;
        for i in (0..self.bits.len()).step_by(type_bits) {
            let mut section_value: T = T::from(0u8);
            for j in 0..type_bits {
                if i + j >= self.bits.len() {
                    break;
                }
                if self.bits[i + j] {
                    section_value = section_value | (T::from(1u8) << j);
                }
            }
            sections.push(section_value);
        }
        sections
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            simd_registers: [
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
                SIMDRegister::new(512),
            ],
        }
    }

    pub fn set_bit(&mut self, reg_type: &str, reg_index: usize, bit_position: usize, value: bool) {
        match reg_type {
            "xmm" if bit_position < 128 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            "ymm" if bit_position < 256 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            "zmm" if bit_position < 512 => {
                self.simd_registers[reg_index].set_bit(bit_position, value);
            }
            _ => eprintln!("Invalid register type or bit position"),
        }
    }

    pub fn get_bit(&self, reg_type: &str, reg_index: usize, bit_position: usize) -> Option<bool> {
        match reg_type {
            "xmm" if bit_position < 128 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            "ymm" if bit_position < 256 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            "zmm" if bit_position < 512 => {
                Some(self.simd_registers[reg_index].get_bit(bit_position))
            }
            _ => None,
        }
    }

    pub fn get_sections<T>(&self, reg_type: &str, reg_index: usize) -> Option<Vec<T>>
        where
            T: From<u8> + std::ops::Shl<usize, Output = T> + std::ops::BitOr<Output = T> + Copy,
    {
        let sections: Vec<T> = self.simd_registers[reg_index].get_sections();
        match reg_type {
            "xmm" => {
                let n = 128 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
            "ymm" => {
                let n = 256 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
            "zmm" => {
                let n = 512 / (std::mem::size_of::<T>() * 8);
                let slice = &sections[..n];
                Some(slice.to_vec())
            }
            _ => None,
        }
    }
}
