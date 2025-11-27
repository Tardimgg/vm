use strum_macros::{EnumIter, EnumString, IntoStaticStr};

#[derive(Debug)]
pub enum Register {
    R16(Register16)
}

impl Register {
    pub fn as_u16(&self) -> u16 {
        match self {
            Register::R16(r) => r.v
        }
    }

    pub fn put_u16(&mut self, new: u16) {
        match self {
            Register::R16(r) => r.v = new
        }
    }
}

#[derive(Debug)]
pub struct Register16 {
    pub v: u16
}

#[derive(Debug, EnumString, IntoStaticStr, EnumIter, Copy, Clone, Hash, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum RegisterName {
    Acc,
    Rg1,
    Rg2,
    Rg3,
    Rg4,
    Rg5,
}

impl RegisterName {
    pub fn get_id(&self) -> u16 {
        match self {
            RegisterName::Acc => 1,
            RegisterName::Rg1 => 2,
            RegisterName::Rg2 => 3,
            RegisterName::Rg3 => 4,
            RegisterName::Rg4 => 5,
            RegisterName::Rg5 => 6
        }
    }

    pub fn new_register(&self) -> Register {
        Register::R16(Register16 {
            v: 0
        })
    }
}

