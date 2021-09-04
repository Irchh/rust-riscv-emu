use crate::dram::DRAM;

pub const DRAM_BASE: usize = 0x8000_0000;

pub trait Device {
    fn write(&mut self, addr: usize, size: usize, val: u64) -> Result<(), ()>;

    fn read(&self, addr: usize, size: usize) -> Result<u64, ()>;
}

#[derive(Debug)]
pub struct BUS {
    dram: DRAM, // Box<[u8]> doesn't wanna work
}

impl BUS {
    pub fn new(mem_size: usize, buffer: Vec<u8>) -> BUS {
        Self {
            dram: DRAM::new(mem_size, buffer),
        }
    }

    pub fn write(&mut self, addr: usize, size: usize, val: u64) -> Result<(), ()> {
        if addr < DRAM_BASE {
            Err(())
        } else {
            self.dram.write(addr-DRAM_BASE, size, val)
        }
    }

    pub fn read(&self, addr: usize, size: usize) -> Result<u64, ()> {
        if addr < DRAM_BASE {
            Err(())
        } else {
            self.dram.read(addr-DRAM_BASE, size)
        }
    }
}