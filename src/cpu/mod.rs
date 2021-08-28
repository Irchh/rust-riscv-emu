#![allow(dead_code)]

use crate::cpu::decode::Instructions;
use std::fmt;
use std::fmt::{Formatter, Error};

mod decode;
mod mmu;

#[derive(Debug)]
pub struct CPU {
    regs: [u64; 32],
    pc: u64,
    running: bool,
    mmu: mmu::MMU,
}

impl fmt::Display for CPU {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        #![allow(unused_must_use)]
        f.write_str("CPU {\n");
        f.write_str(format!("\tregs: {:?},\n", self.regs).as_str());
        f.write_str(format!("\tpc: {:?},\n", self.pc).as_str());
        f.write_str(format!("\trunning: {:?},\n", self.running).as_str());
        f.write_str(format!("\tmmu: MMU {{ ... }},\n").as_str());
        f.write_str("}\n");
        Ok(())
    }
}

pub const REG_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp",
    "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1",
    "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3",
    "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6"
];

impl CPU {
    pub fn new(buffer: Vec<u8>) -> CPU {
        let mem_size = 1024*1024;
        let mut regs = [0; 32];
        regs[2] = mem_size as u64;
        CPU {
            regs,
            pc: 0,
            running: true,
            mmu: mmu::MMU::new(mem_size, buffer),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn print_all(&self) {
        println!("{}", self);
    }

    pub fn print_regs(&mut self) {
        for i in 0..self.regs.len() {
            let mut data = format!("{} = ", REG_NAMES[i]);
            data = format!("{:>7}{}", data, self.regs[i] as i64);
            print!("{:15}", data);
            if (i+1)%8 == 0 {
                println!();
            }
        }
    }

    fn fetch(&self) -> Result<u32, ()> {
        self.mmu.read_32(self.pc as usize)
    }

    fn write_reg(&mut self, reg: usize, val: u64) {
        if reg == 0 || reg > self.regs.len() {
            return;
        }
        self.regs[reg] = val;
    }

    fn read_reg(&self, reg: usize) -> u64 {
        if reg == 0 || reg > self.regs.len() {
            return 0;
        }
        self.regs[reg]
    }

    fn execute(&mut self, inst: Instructions) -> Result<(), String> {
        let result = match inst {
            Instructions::Add { rd, rs1, rs2 } => {
                self.write_reg(rd, self.read_reg(rs1).wrapping_add(self.read_reg(rs2)));
                Ok(())
            }
            Instructions::Addi { rd, rs1, imm } => {
                self.write_reg(rd, self.read_reg(rs1).wrapping_add(imm as u64));
                Ok(())
            }
            Instructions::Lb { rd, rs1, imm } => {
                let val = self.mmu.read_8(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap()as i8 as i64 as u64);
                    Ok(())
                }
            }
            Instructions::Lh { rd, rs1, imm } => {
                let val = self.mmu.read_16(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap()as i16 as i64 as u64);
                    Ok(())
                }
            }
            Instructions::Lw { rd, rs1, imm } => {
                let val = self.mmu.read_32(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap() as i32 as i64 as u64);
                    Ok(())
                }
            }
            Instructions::Ld { rd, rs1, imm } => {
                let val = self.mmu.read_64(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap());
                    Ok(())
                }
            }

            Instructions::Lbu { rd, rs1, imm } => {
                let val = self.mmu.read_8(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap() as u64);
                    Ok(())
                }
            }
            Instructions::Lhu { rd, rs1, imm } => {
                let val = self.mmu.read_16(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap() as u64);
                    Ok(())
                }
            }
            Instructions::Lwu { rd, rs1, imm } => {
                let val = self.mmu.read_32(self.read_reg(rs1).wrapping_add(imm as u64) as usize);
                if val.is_err() {
                    Err("Read error!".to_string())
                } else {
                    self.write_reg(rd, val.unwrap() as u64);
                    Ok(())
                }
            }

            Instructions::Sb { rs1, rs2, imm } => {
                if self.mmu.write_8(self.read_reg(rs1).wrapping_add(imm as u64) as usize, self.read_reg(rs2) as u8).is_err() {
                    Err("Write error!".to_string())
                } else {
                    Ok(())
                }
            }
            Instructions::Sh { rs1, rs2, imm } => {
                if self.mmu.write_16(self.read_reg(rs1).wrapping_add(imm as u64) as usize, self.read_reg(rs2) as u16).is_err() {
                    Err("Write error!".to_string())
                } else {
                    Ok(())
                }
            }
            Instructions::Sw { rs1, rs2, imm } => {
                if self.mmu.write_32(self.read_reg(rs1).wrapping_add(imm as u64) as usize, self.read_reg(rs2) as u32).is_err() {
                    Err("Write error!".to_string())
                } else {
                    Ok(())
                }
            }
            Instructions::Sd { rs1, rs2, imm } => {
                if self.mmu.write_64(self.read_reg(rs1).wrapping_add(imm as u64) as usize, self.read_reg(rs2)).is_err() {
                    Err("Write error!".to_string())
                } else {
                    Ok(())
                }
            }

            Instructions::Auipc { rd, imm } => {
                println!("Auipc imm: 0x{:X}", imm);
                self.write_reg(rd, self.pc + (imm as u64));
                Ok(())
            }

            Instructions::Jalr { rd, rs1, imm } => {
                let rs1_val = self.read_reg(rs1);
                self.write_reg(rd, self.pc + 4);
                println!("self.pc: 0x{:02X}", self.pc);
                self.pc = imm as u64 + rs1_val;
                self.pc &= !1;
                println!("self.pc: 0x{:02X}", self.pc);
                self.pc = self.pc.wrapping_sub(4); // To negate the +4 after
                Ok(())
            }

            Instructions::Jal { rd, imm } => {
                self.set(rd, self.pc + 4);
                self.pc += imm.wrapping_sub(4) as u64;
                Ok(())
            }

            Instructions::Beq { rs1, rs2, imm } => {
                if self.read_reg(rs1) == self.read_reg(rs2) {
                    self.pc = imm as u64;
                }
                Ok(())
            }

            Instructions::Bne { rs1, rs2, imm } => {
                if self.read_reg(rs1) != self.read_reg(rs2) {
                    self.pc = imm as u64;
                }
                Ok(())
            }

            _ => {
                Err("Instruction not implemented!".to_string())
            }
        };
        self.pc = self.pc.wrapping_add(4);
        result
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }
        // Fetch, decode, execute:
        let raw_u32 = self.fetch();
        if raw_u32.is_err() {
            self.running = false; return;
        }
        let inst = decode::Instructions::from(raw_u32.unwrap());
        println!("{:02X} inst: {:?}", self.pc, inst);
        let status = self.execute(inst);
        if status.is_err() {
            self.running = false;
            println!("Error: {}", status.err().unwrap());
        }
    }

    fn set(&mut self, reg: usize, value: u64) {
        if reg > 0  && reg < self.regs.len() {
            self.regs[reg-1] = value;
        }
    }

    fn get(&self, reg: usize) -> u64 {
        if reg > 0  && reg < self.regs.len() {
            self.regs[reg-1]
        } else {
            0
        }
    }
}