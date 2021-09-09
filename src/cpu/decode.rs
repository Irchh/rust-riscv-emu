use crate::cpu::decode::Instructions::*;
use std::fmt::{Debug, Formatter};
use crate::cpu::REG_NAMES;
use std::convert::TryFrom;
use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter;

pub enum Instructions {
    Add{rd: usize, rs1: usize, rs2: usize},
    Addw{rd: usize, rs1: usize, rs2: usize},
    Addi{rd: usize, rs1: usize, imm: i64},
    Addiw{rd: usize, rs1: usize, imm: i64},
    And{rd: usize, rs1: usize, rs2: usize},
    Andi{rd: usize, rs1: usize, imm: i64},
    Auipc{rd: usize, imm: i64},
    Beq{rs1: usize, rs2: usize, imm: i64},
    Bge{rs1: usize, rs2: usize, imm: i64},
    Bgeu{rs1: usize, rs2: usize, imm: i64},
    Blt{rs1: usize, rs2: usize, imm: i64},
    Bltu{rs1: usize, rs2: usize, imm: i64},
    Bne{rs1: usize, rs2: usize, imm: i64},

    Csrrc{rd: usize, rs1: usize, csr: i64},
    Csrrci{rd: usize, rs1: usize, csr: i64},
    Csrrs{rd: usize, rs1: usize, csr: i64},
    Csrrsi{rd: usize, rs1: usize, csr: i64},
    Csrrw{rd: usize, rs1: usize, csr: i64},
    Csrrwi{rd: usize, rs1: usize, csr: i64},

    Ebreak,
    Ecall,
    Fence{rd: usize, rs1: usize, succ: i64, pred: i64, fm: i64},
    FenceI,
    Jal{rd: usize, imm: i64},
    Jalr{rd: usize, rs1: usize, imm: i64},
    Lb{rd: usize, rs1: usize, imm: i64},
    Lbu{rd: usize, rs1: usize, imm: i64},
    Lh{rd: usize, rs1: usize, imm: i64},
    Lhu{rd: usize, rs1: usize, imm: i64},
    Lw{rd: usize, rs1: usize, imm: i64},
    Lwu{rd: usize, rs1: usize, imm: i64},
    Ld{rd: usize, rs1: usize, imm: i64},
    Ldu{rd: usize, rs1: usize, imm: i64},
    Lui{rd: usize, imm: i64},
    Mul,
    Mulh,
    Mulhsu,
    Mulhu,
    Div,
    Divu,
    Rem,
    Remu,
    Or,
    Ori,
    RdCycle,
    RdCycleH,
    RdTime,
    RdTimeH,
    RdInstRet,
    RdInstRetH,
    Sb{rs1: usize, rs2: usize, imm: i64},
    Sh{rs1: usize, rs2: usize, imm: i64},
    Sw{rs1: usize, rs2: usize, imm: i64},
    Sd{rs1: usize, rs2: usize, imm: i64},
    Sll,
    Sllw,
    Slli,
    Slliw,
    Slt,
    Slti,
    Sltu,
    Sltiu,
    Sra,
    Sraw,
    Srai,
    Sraiw,
    Srl,
    Srlw,
    Srli,
    Srliw,
    Sub,
    Subw,
    Xor,
    Xori,
    Unknown,
}

#[allow(non_camel_case_types)]
#[derive(Debug, EnumIter, Clone)]
pub enum CsrNames {
    sstatus = 0x100,
    sedeleg,

    mstatus = 0x300,
    misa,
    medeleg,

    mhartid = 0xf14,
}

impl TryFrom<i64> for CsrNames {
    type Error = i64;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        for t in CsrNames::iter() {
            if value == t.clone() as i64 {
                return Ok(t.clone());
            }
        }
        return Err(value);
    }
}

impl Debug for Instructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = match *self {
            Instructions::Add { rd, rs1, rs2 } => {
                f.write_str(format!("Add {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], REG_NAMES[rs2]).as_str())
            }
            Instructions::Addw { rd, rs1, rs2 } => {
                f.write_str(format!("Addw {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], REG_NAMES[rs2]).as_str())
            }
            Instructions::Addi { rd, rs1, imm } => {
                f.write_str(format!("Addi {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Addiw { rd, rs1, imm } => {
                f.write_str(format!("Addiw {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::And { rd, rs1, rs2 } => {
                f.write_str(format!("And {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], REG_NAMES[rs2]).as_str())
            }
            Instructions::Andi { rd, rs1, imm } => {
                f.write_str(format!("Andi {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Auipc { rd, imm } => {
                f.write_str(format!("Auipc {}, {}", REG_NAMES[rd], imm).as_str())
            }
            Instructions::Beq { rs1, rs2, imm } => {
                f.write_str(format!("Beq {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Instructions::Bge { rs1, rs2, imm } => {
                f.write_str(format!("Bge {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Instructions::Bgeu { rs1, rs2, imm } => {
                f.write_str(format!("Bgeu {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Instructions::Blt { rs1, rs2, imm } => {
                f.write_str(format!("Blt {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Instructions::Bltu { rs1, rs2, imm } => {
                f.write_str(format!("Bltu {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Instructions::Bne { rs1, rs2, imm } => {
                f.write_str(format!("Bne {}, {}, {}", REG_NAMES[rs1], REG_NAMES[rs2], imm).as_str())
            }
            Csrrc{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrc {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Csrrci{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrci {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Csrrs{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrs {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Csrrsi{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrsi {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Csrrw{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrw {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Csrrwi{rd, rs1, csr: imm } => {
                f.write_str(format!("Csrrwi {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Ebreak => { f.write_str(format!("Ebreak").as_str()) }
            Ecall => { f.write_str(format!("Ecall").as_str()) }
            Instructions::Fence { rd, rs1, succ, pred, fm } => {
                f.write_str(format!("Fence {}, {}, succ: {}, pred: {}, fm: {}", REG_NAMES[rd], REG_NAMES[rs1], succ, pred, fm).as_str())
            }
            FenceI => { f.write_str(format!("FenceI").as_str()) }
            Instructions::Jal { rd, imm } => {
                f.write_str(format!("Jal {}, {}", REG_NAMES[rd], imm).as_str())
            }
            Instructions::Jalr { rd, rs1, imm } => {
                f.write_str(format!("Jalr {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lb { rd, rs1, imm } => {
                f.write_str(format!("Lb {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lbu { rd, rs1, imm } => {
                f.write_str(format!("Lbu {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lh { rd, rs1, imm } => {
                f.write_str(format!("Lh {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lhu { rd, rs1, imm } => {
                f.write_str(format!("Lhu {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lw { rd, rs1, imm } => {
                f.write_str(format!("Lw {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lwu { rd, rs1, imm } => {
                f.write_str(format!("Lwu {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Ld { rd, rs1, imm } => {
                f.write_str(format!("Ld {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Ldu { rd, rs1, imm } => {
                f.write_str(format!("Ldu {}, {}, {}", REG_NAMES[rd], REG_NAMES[rs1], imm).as_str())
            }
            Instructions::Lui { rd, imm } => {
                f.write_str(format!("Lui {}, {}", REG_NAMES[rd], imm).as_str())
            }
            Mul => { f.write_str(format!("Mul").as_str()) }
            Mulh => { f.write_str(format!("Mulh").as_str()) }
            Mulhsu => { f.write_str(format!("Mulhsu").as_str()) }
            Mulhu => { f.write_str(format!("Mulhs").as_str()) }
            Div => { f.write_str(format!("Div").as_str()) }
            Divu => { f.write_str(format!("Divu").as_str()) }
            Rem => { f.write_str(format!("Rem").as_str()) }
            Remu => { f.write_str(format!("Remu").as_str()) }
            Or => { f.write_str(format!("Or").as_str()) }
            Ori => { f.write_str(format!("Ori").as_str()) }
            RdCycle => { f.write_str(format!("RdCycle").as_str()) }
            RdCycleH => { f.write_str(format!("RdCycleH").as_str()) }
            RdTime => { f.write_str(format!("RdTime").as_str()) }
            RdTimeH => { f.write_str(format!("RdTimeH").as_str()) }
            RdInstRet => { f.write_str(format!("RdInstRet").as_str()) }
            RdInstRetH => { f.write_str(format!("RdInstRetH").as_str()) }
            Instructions::Sb { rs1, rs2, imm } => {
                f.write_str(format!("Sb {}, {}({})", REG_NAMES[rs2], imm, REG_NAMES[rs1]).as_str())
            }
            Instructions::Sh { rs1, rs2, imm } => {
                f.write_str(format!("Sh {}, {}({})", REG_NAMES[rs2], imm, REG_NAMES[rs1]).as_str())
            }
            Instructions::Sw { rs1, rs2, imm } => {
                f.write_str(format!("Sw {}, {}({})", REG_NAMES[rs2], imm, REG_NAMES[rs1]).as_str())
            }
            Instructions::Sd { rs1, rs2, imm } => {
                f.write_str(format!("Sd {}, {}({})", REG_NAMES[rs2], imm, REG_NAMES[rs1]).as_str())
            }
            Sll => { f.write_str(format!("Sll").as_str()) }
            Sllw => { f.write_str(format!("Sllw").as_str()) }
            Slli => { f.write_str(format!("Slli").as_str()) }
            Slliw => { f.write_str(format!("Slliw").as_str()) }
            Slt => { f.write_str(format!("Slt").as_str()) }
            Slti => { f.write_str(format!("Slti").as_str()) }
            Sltu => { f.write_str(format!("Slti").as_str()) }
            Sltiu => { f.write_str(format!("Sltiu").as_str()) }
            Sra => { f.write_str(format!("Sra").as_str()) }
            Sraw => { f.write_str(format!("Sraw").as_str()) }
            Srai => { f.write_str(format!("Srai").as_str()) }
            Sraiw => { f.write_str(format!("Sraiw").as_str()) }
            Srl => { f.write_str(format!("Srl").as_str()) }
            Srlw => { f.write_str(format!("Srlw").as_str()) }
            Srli => { f.write_str(format!("Srli").as_str()) }
            Srliw => { f.write_str(format!("Srliw").as_str()) }
            Sub => { f.write_str(format!("Sub").as_str()) }
            Subw => { f.write_str(format!("Subw").as_str()) }
            Xor => { f.write_str(format!("Xor").as_str()) }
            Xori => { f.write_str(format!("Xori").as_str()) }
            Unknown => { f.write_str(format!("Unknown").as_str()) }
        };
        Ok(())
    }
}

impl Instructions {
    pub fn from(inst: u32) -> Instructions {
        let opcode = (inst&0x7F) as u8;
        let rd = ((inst>>7)&0x1F) as usize;
        let rs1 = ((inst>>15)&0x1F) as usize;
        let rs2 = ((inst>>20)&0x1F) as usize;
        let funct3 = ((inst>>12)&0x7) as usize;
        #[allow(unused_variables)]
        let funct7 = ((inst>>25)&0x7F) as usize;
        let itype_imm =  (((inst&0xFFF00000) as i32)>>20) as i32 as i64; // Sign extension logic (??)
        let stype_imm = (((((inst&0xFE000000) as i32)>>20) as u32) | rd as u32) as i32 as i64;
        let utype_imm =  (inst&0xFFFFF000) as i32 as i64; // TODO: Check for accuracy

        let __btype_imm = stype_imm & !1;
        let _btype_imm = __btype_imm & !(1<<11);
        let btype_imm = _btype_imm | (stype_imm & 1)<<11; // TODO: Check for accuracy

        let _jtype_imm_1_10 = (((inst >> 21) & 0x3FF) << 1) as i64;
        let _jtype_imm_11 = (((inst >> 20) & 0x1) << 11) as i64;
        let _jtype_imm_12_19 = (((inst >> 12) & 0xFF) << 12) as i64;
        let _jtype_imm_20 = (((inst&(1<<31)) as i32) >> 11) as i64;
        let jtype_imm = _jtype_imm_1_10 | _jtype_imm_11 | _jtype_imm_12_19 | _jtype_imm_20;

        match opcode {
            0x13 => /* OP-IMM */ {
                match funct3 {
                    0b000 => { Addi { rd, rs1, imm: itype_imm } }
                    _ => {
                        println!("Unknown funct3 OP-IMM: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }
            0x33 => {
                match funct3 {
                    0b000 => { Add { rd, rs1, rs2 } }
                    _ => {
                        println!("Unknown funct3 0x33: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }
            0x1B => /* OP-IMM-32 */ {
                match funct3 {
                    0b000 => { Addiw { rd, rs1, imm: itype_imm } }
                    _ => {
                        println!("Unknown funct3 OP-IMM-32: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }
            0x3B => {
                match funct3 {
                    0b000 => { Addw { rd, rs1, rs2 } }
                    _ => {
                        println!("Unknown funct3 0x33: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }

            0x03 => /* Loads */ {
                match funct3 {
                    0b000 => { Lb { rd, rs1, imm: itype_imm } }
                    0b001 => { Lh { rd, rs1, imm: itype_imm } }
                    0b010 => { Lw { rd, rs1, imm: itype_imm } }
                    0b011 => { Ld { rd, rs1, imm: itype_imm } }
                    0b100 => { Lbu { rd, rs1, imm: itype_imm } }
                    0b101 => { Lhu { rd, rs1, imm: itype_imm } }
                    0b110 => { Lwu { rd, rs1, imm: itype_imm } }
                    _ => {
                        println!("Unknown funct3 load: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }

            0x23 => /* Stores */ {
                match funct3 {
                    0b000 => { Sb { rs1, rs2, imm: stype_imm } }
                    0b001 => { Sh { rs1, rs2, imm: stype_imm } }
                    0b010 => { Sw { rs1, rs2, imm: stype_imm } }
                    0b011 => { Sd { rs1, rs2, imm: stype_imm } }
                    _ => {
                        println!("Unknown funct3 store: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }

            0x17 => { Auipc { rd, imm: utype_imm } }
            0x63 => /* Conditional jumps */ {
                println!("\n\nfunct3 cond jmp: 0b{:03b}", funct3);
                println!("inst: 0b{:032b}", inst);
                match funct3  {
                    0b000 => { Beq { rs1, rs2, imm: btype_imm } }
                    0b001 => { Bne { rs1, rs2, imm: btype_imm } }
                    0b100 => { Blt { rs1, rs2, imm: btype_imm } }
                    0b101 => { Bge { rs1, rs2, imm: btype_imm } }
                    0b110 => { Bltu { rs1, rs2, imm: btype_imm } }
                    0b111 => { Bgeu { rs1, rs2, imm: btype_imm } }
                    _ => {
                        println!("Unknown funct3 cond jmp: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }

            0x67 => { Jalr { rd, rs1, imm: itype_imm } }

            0x6F => { Jal { rd, imm: jtype_imm } }

            0x73 => /* SYSTEM */ {
                match funct3 {
                    0x1 => /* CSRRW */ {
                        Csrrw { rd, rs1, csr: itype_imm }
                    }
                    0x2 => /* CSRRS */ {
                        Csrrs { rd, rs1, csr: itype_imm }
                    }
                    0x3 => /* CSRRC */ {
                        Csrrc { rd, rs1, csr: itype_imm }
                    }
                    0x5 => /* CSRRWI */ {
                        Csrrwi { rd, rs1, csr: itype_imm }
                    }
                    0x6 => /* CSRRSI */ {
                        Csrrsi { rd, rs1, csr: itype_imm }
                    }
                    0x7 => /* CSRRCI */ {
                        Csrrci { rd, rs1, csr: itype_imm }
                    }
                    _ => {
                        println!("Unknown funct3 system: 0x{:X}", funct3);
                        Unknown
                    }
                }
            }

            _ => {
                println!("Unknown instruction: 0x{:02X}", opcode);
                Instructions::Unknown
            }
        }
    }
}