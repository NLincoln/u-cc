use crate::asm::Address::Indirect;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Rbp,
    Rsp,
    Eax,
    Ebp,
    Edi,
    Esi,
    Esp,
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Register::*;
        write!(
            f,
            "{}",
            match self {
                Rbp => "rbp",
                Rsp => "rsp",
                Eax => "eax",
                Ebp => "ebp",
                Esp => "esp",
                Edi => "edi",
                Esi => "esi",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    Register(Register),
    Immediate(i32),
    Indirect(IndirectAddress),
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Address::Immediate(val) => Display::fmt(val, f),
            Address::Register(reg) => Display::fmt(reg, f),
            Address::Indirect(indirect) => Display::fmt(indirect, f),
        }
    }
}

impl From<Register> for Address {
    fn from(register: Register) -> Address {
        Address::Register(register)
    }
}

impl From<IndirectAddress> for Address {
    fn from(addr: IndirectAddress) -> Address {
        Address::Indirect(addr)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndirectAddress {
    name: Box<Address>,
    offset: Option<i32>,
}
impl IndirectAddress {
    pub fn offset(name: Box<Address>, offset: i32) -> IndirectAddress {
        IndirectAddress {
            name,
            offset: Some(offset),
        }
    }
    pub fn indirect(name: Box<Address>) -> IndirectAddress {
        IndirectAddress { name, offset: None }
    }
}

impl Display for IndirectAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.offset {
            Some(offset) => write!(f, "[{} {}]", self.name, offset),
            None => write!(f, "[{}]", self.name),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Label(String),
    Push(Register),
    /// dest, src
    Mov(Address, Address),
    // dest, adder
    Add(Address, Address),
    /// label
    Call(String),

    Pop(Register),
    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Push(reg) => write!(f, "push {}", reg),
            Instruction::Mov(src, dest) => write!(f, "mov {}, {}", src, dest),
            Instruction::Add(src, dest) => write!(f, "add {}, {}", src, dest),
            Instruction::Call(label) => write!(f, "call {}", label),
            Instruction::Pop(reg) => write!(f, "pop {}", reg),
            Instruction::Ret => write!(f, "ret"),
        }
    }
}
