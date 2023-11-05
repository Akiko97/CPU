extern crate primitive_types;
use primitive_types::U256 as u256;
use primitive_types::U512 as u512;

extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

trait MemoryIO {
    fn from_bytes(bytes: &[u8]) -> Self;
    fn to_bytes(&self) -> Vec<u8>;
    fn size() -> usize;
}

macro_rules! impl_memory_io {
    ($t:ty, $type_str:ident, $size:expr) => {
        impl MemoryIO for $t {
            fn from_bytes(bytes: &[u8]) -> Self {
                let mut rdr = std::io::Cursor::new(bytes);
                match stringify!($type_str) {
                    "u8" => rdr.read_u8().unwrap() as $t,
                    "u16" => rdr.read_u16::<LittleEndian>().unwrap() as $t,
                    "u32" => rdr.read_u32::<LittleEndian>().unwrap() as $t,
                    "u64" => rdr.read_u64::<LittleEndian>().unwrap() as $t,
                    "u128" => rdr.read_u128::<LittleEndian>().unwrap() as $t,
                    _ => panic!("Unsupported type"),
                }
            }

            fn to_bytes(&self) -> Vec<u8> {
                let mut wtr = vec![];
                match stringify!($type_str) {
                    "u8" => wtr.write_u8(*self as u8).unwrap(),
                    "u16" => wtr.write_u16::<LittleEndian>(*self as u16).unwrap(),
                    "u32" => wtr.write_u32::<LittleEndian>(*self as u32).unwrap(),
                    "u64" => wtr.write_u64::<LittleEndian>(*self as u64).unwrap(),
                    "u128" => wtr.write_u128::<LittleEndian>(*self as u128).unwrap(),
                    _ => panic!("Unsupported type"),
                };
                wtr
            }

            fn size() -> usize {
                $size
            }
        }
    };
}

impl_memory_io!(u8, u8, 1);
impl_memory_io!(u16, u16, 2);
impl_memory_io!(u32, u32, 4);
impl_memory_io!(u64, u64, 8);
impl_memory_io!(u128, u128, 16);

impl MemoryIO for u256 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u256::from_little_endian(bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = vec![0; 32];
        self.to_little_endian(&mut wtr);
        wtr
    }

    fn size() -> usize {
        32
    }
}

impl MemoryIO for u512 {
    fn from_bytes(bytes: &[u8]) -> Self {
        u512::from_little_endian(bytes)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut wtr = vec![0; 64];
        self.to_little_endian(&mut wtr);
        wtr
    }

    fn size() -> usize {
        64
    }
}

const DEFAULT_SIZE: usize = 512; // 512 bytes

struct MemorySegment {
    start_address: usize,
    data: Vec<u8>,
}

pub struct Memory {
    segments: Vec<MemorySegment>,
    base_address: usize,
}

impl Memory {
    pub fn new(base: usize) -> Self {
        Memory {
            segments: Vec::new(),
            base_address: base,
        }
    }

    fn find_segment(&self, real_address: usize) -> Option<usize> {
        for (index, segment) in self.segments.iter().enumerate() {
            if real_address >= segment.start_address && real_address < segment.start_address + segment.data.len() {
                return Some(index);
            }
        }
        None
    }

    fn read_byte(&self, address: usize) -> u8 {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            self.segments[index].data[real_address - self.segments[index].start_address]
        } else {
            // return 0 if the address is not found
            0
        }
    }

    fn write_byte(&mut self, address: usize, value: u8) {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            let start = self.segments[index].start_address;
            self.segments[index].data[real_address - start] = value;
        } else {
            let adjusted_address = (real_address / DEFAULT_SIZE) * DEFAULT_SIZE;
            let mut new_data = Vec::with_capacity(DEFAULT_SIZE);
            new_data.resize(DEFAULT_SIZE, 0);
            new_data[real_address - adjusted_address] = value;
            let new_segment = MemorySegment {
                start_address: adjusted_address,
                data: new_data,
            };
            self.segments.push(new_segment);
            // sort by address
            self.segments.sort_by(|a, b| a.start_address.cmp(&b.start_address));
        }
        // merge segments if they are contiguous
        let mut i = 0;
        while i + 1 < self.segments.len() {
            if self.segments[i].start_address + self.segments[i].data.len() == self.segments[i + 1].start_address {
                let next = self.segments.remove(i + 1);
                self.segments[i].data.extend(next.data);
            } else {
                i += 1;
            }
        }
    }

    pub fn read<T: MemoryIO>(&self, address: usize) -> T {
        let mut bytes = Vec::new();
        for i in 0..T::size() {
            bytes.push(self.read_byte(address + i));
        }
        T::from_bytes(&bytes)
    }

    pub fn write<T: MemoryIO>(&mut self, address: usize, value: T) {
        let bytes = value.to_bytes();
        for (i, byte) in bytes.iter().enumerate() {
            self.write_byte(address + i, *byte);
        }
    }

    pub fn read_vec<T: MemoryIO>(&self, address: usize, number_of_value: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![];
        for i in 0..number_of_value {
            result.push(self.read(address + i * T::size()));
        }
        result
    }

    pub fn write_vec<T: MemoryIO + Clone>(&mut self, address: usize, values: Vec<T>) {
        for (i, value) in values.iter().enumerate() {
            self.write(address + i * T::size(), value.clone());
        }
    }
}
