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

    pub fn read_byte(&self, address: usize) -> u8 {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            self.segments[index].data[real_address - self.segments[index].start_address]
        } else {
            // return 0 if the address is not found
            0
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        let real_address = address - self.base_address;
        if let Some(index) = self.find_segment(real_address) {
            let start = self.segments[index].start_address;
            self.segments[index].data[real_address - start] = value;
        } else {
            let mut new_data = Vec::with_capacity(DEFAULT_SIZE);
            new_data.resize(DEFAULT_SIZE, 0);
            new_data[0] = value;
            let new_segment = MemorySegment {
                start_address: real_address,
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
}
