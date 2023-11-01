extern crate bit_vec;
use bit_vec::BitVec;

// trait alias
trait SectionCompatible:
    From<u8> + Copy + Eq +
    std::ops::Shl<usize, Output = Self> + std::ops::Shr<usize, Output = Self> +
    std::ops::BitOr<Output = Self> + std::ops::BitAnd<Output = Self>
{}
impl<T:
    From<u8> + Copy + Eq +
    std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> +
    std::ops::BitOr<Output = T> + std::ops::BitAnd<Output = T>> SectionCompatible for T
{}

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

    fn get_sections<T: SectionCompatible>(&self) -> Vec<T> {
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

    fn set_by_sections<T: SectionCompatible>(&mut self, sections: Vec<T>) -> bool {
        let type_bits = std::mem::size_of::<T>() * 8;
        if type_bits * sections.len() != self.bits.len() {
            return false;
        }
        let mut i = 0;
        for section in &sections {
            for j in 0..type_bits {
                if i + j >= self.bits.len() {
                    break;
                }
                if (*section >> j) & T::from(1u8) == T::from(1u8) {
                    self.set_bit(i + j, true);
                }
            }
            i += type_bits;
        }
        true
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

    pub fn get_sections<T: SectionCompatible>(&self, reg_type: &str, reg_index: usize) -> Option<Vec<T>> {
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

    pub fn set_by_sections<T: SectionCompatible>(&mut self, reg_type: &str, reg_index: usize, sections: Vec<T>) -> bool {
        let type_bits = std::mem::size_of::<T>() * 8;
        let register_bits = type_bits * sections.len();
        let fill_sections = (512 - register_bits) / type_bits;
        match reg_type {
            "xmm" => {
                if register_bits != 128 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
            "ymm" => {
                if register_bits != 256 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
            "zmm" => {
                if register_bits != 512 {
                    return false;
                }
                let mut fill = sections;
                fill.extend(std::iter::repeat(T::from(0u8)).take(fill_sections));
                self.simd_registers[reg_index].set_by_sections(fill);
                true
            }
            _ => false,
        }
    }
}
