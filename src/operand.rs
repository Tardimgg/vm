use crate::register::RegisterName;
use enum_tags::Tag;
use enum_tags_traits::TaggedEnum;

#[derive(Debug, Copy, Clone, Tag)]
pub enum Operand {
    Literal(u16),
    Register(RegisterName)
}

impl OperandTag {
     pub fn get_id(&self) -> u16 {
         match self {
             OperandTag::Literal => 1,
             OperandTag::Register => 2
         }
    }
}