use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    Rbp,
    Rsp,
    Eax,
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Register::Rbp => "rbp",
                Register::Rsp => "rsp",
                Register::Eax => "eax",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Address {
    Register(Register),
    Immediate(i32),
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Address::Immediate(val) => Display::fmt(val, f),
            Address::Register(reg) => Display::fmt(reg, f),
        }
    }
}

impl From<Register> for Address {
    fn from(register: Register) -> Address {
        Address::Register(register)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Label(String),
    Push(Register),
    /// dest, src
    Mov(Address, Address),

    Pop(Register),
    Ret,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Push(reg) => write!(f, "push {}", reg),
            Instruction::Mov(src, dest) => write!(f, "mov {}, {}", src, dest),
            Instruction::Pop(reg) => write!(f, "pop {}", reg),
            Instruction::Ret => write!(f, "ret"),
        }
    }
}
