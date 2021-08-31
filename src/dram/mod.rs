#![allow(dead_code)]

use crate::bus::Device;

#[derive(Debug)]
pub struct DRAM {
    dram: Vec<u8>, // Box<[u8]> doesn't wanna work
}

impl Device for DRAM {
    fn write(&mut self, addr: usize, size: usize, val: u64) -> Result<(), ()> {
        match size {
            8 => { self.write_8(addr, val as u8) }
            16 => { self.write_16(addr, val as u16) }
            32 => { self.write_32(addr, val as u32) }
            64 => { self.write_64(addr, val) }
            _ => {
                Err(())
            }
        }
    }

    fn read(&self, addr: usize, size: usize) -> Result<u64, ()> {
        match size {
            8 => { self.read_8(addr) }
            16 => { self.read_16(addr) }
            32 => { self.read_32(addr) }
            64 => { self.read_64(addr) }
            _ => {
                Err(())
            }
        }
    }
}

impl DRAM {
    pub fn new(mem_size: usize, buffer: Vec<u8>) -> DRAM {
        let mut memory = vec![0; mem_size];
        memory.splice(..buffer.len(), buffer.iter().cloned());
        Self {
            dram: memory,
        }
    }

    pub fn write_8(&mut self, addr: usize, val: u8) -> Result<(), ()> {
        // TODO: Paging
        println!("\tWriting \"{:02X}\" to addr 0x{:X}", val, addr);
        if addr < 0 || addr >= self.dram.len() {
            Err(())
        } else {
            self.dram[addr] = val;
            Ok(())
        }
    }

    pub fn write_16(&mut self, addr: usize, val: u16) -> Result<(), ()> {
        let lower = val as u8;
        let upper = (val>>8) as u8;
        if self.write_8(addr, lower).is_err() {
            return Err(());
        }
        self.write_8(addr.overflowing_add(1).0, upper)
    }

    pub fn write_32(&mut self, addr: usize, val: u32) -> Result<(), ()> {
        let lower = val as u16;
        let upper = (val>>16) as u16;
        if self.write_16(addr, lower).is_err() {
            return Err(());
        }
        self.write_16(addr.overflowing_add(2).0, upper)
    }

    pub fn write_64(&mut self, addr: usize, val: u64) -> Result<(), ()> {
        let lower = val as u32;
        let upper = (val>>32) as u32;
        if self.write_32(addr, lower).is_err() {
            return Err(());
        }
        self.write_32(addr.overflowing_add(4).0, upper)
    }

    pub fn read_8(&self, addr: usize) -> Result<u64, ()> {
        // TODO: Paging
        if addr < 0 || addr >= self.dram.len() {
            Err(())
        } else {
            //println!("\tReading \"{:02X}\" from addr 0x{:X}", self.ram[addr-DRAM_BASE], addr);
            Ok(self.dram[addr] as u64)
        }
    }

    pub fn read_16(&self, addr: usize) -> Result<u64, ()> {
        let _retval = self.read_8(addr);
        if _retval.is_err() {
            return Err(());
        }
        let mut retval = _retval.unwrap() as u16;
        let _add = self.read_8(addr.overflowing_add(1).0);
        if _add.is_err() {
            return Err(());
        }
        let add = _add.unwrap() as u16;
        retval = retval | add<<8;
        Ok(retval as u64)
    }

    pub fn read_32(&self, addr: usize) -> Result<u64, ()> {
        let _retval = self.read_16(addr);
        if _retval.is_err() {
            return Err(());
        }
        let mut retval = _retval.unwrap() as u32;
        let _add = self.read_16(addr.overflowing_add(2).0);
        if _add.is_err() {
            return Err(());
        }
        let add = _add.unwrap() as u32;
        retval = retval | add<<16;
        Ok(retval as u64)
    }

    pub fn read_64(&self, addr: usize) -> Result<u64, ()> {
        let _retval = self.read_32(addr);
        if _retval.is_err() {
            return Err(());
        }
        let mut retval = _retval.unwrap() as u64;
        let _add = self.read_32(addr.overflowing_add(4).0);
        if _add.is_err() {
            return Err(());
        }
        let add = _add.unwrap() as u64;
        retval = retval | add<<32;
        Ok(retval)
    }
}