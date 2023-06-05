use super::Instruction;
use crate::{
    micheline::{self, Micheline},
    michelson::{data::Data, Michelson},
    Error, Result,
};
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Sequence(Vec<Instruction>);

impl Sequence {
    pub fn instructions(&self) -> &[Instruction] {
        &self.0
    }

    pub fn into_instructions(self) -> Vec<Instruction> {
        self.0
    }

    pub fn form(values: Vec<Instruction>) -> Self {
        Self(values)
    }
}

impl From<Vec<Instruction>> for Sequence {
    fn from(values: Vec<Instruction>) -> Self {
        Self::form(values)
    }
}

impl From<Sequence> for Vec<Instruction> {
    fn from(value: Sequence) -> Self {
        value.0
    }
}

impl TryFrom<Instruction> for Sequence {
    type Error = Error;

    fn try_from(value: Instruction) -> Result<Self> {
        if let Instruction::Sequence(value) = value {
            return Ok(value);
        }
        Err(Error::InvalidMichelsonInstruction)
    }
}

impl From<Sequence> for Instruction {
    fn from(value: Sequence) -> Self {
        Instruction::Sequence(value)
    }
}

impl From<Sequence> for Micheline {
    fn from(value: Sequence) -> Self {
        value
            .0
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<&Sequence> for Micheline {
    fn from(value: &Sequence) -> Self {
        value
            .0
            .iter()
            .map(|value| value.into())
            .collect::<Vec<Micheline>>()
            .into()
    }
}

impl From<Sequence> for Michelson {
    fn from(value: Sequence) -> Self {
        let instruction: Instruction = value.into();
        let data: Data = instruction.into();
        data.into()
    }
}

impl TryFrom<Micheline> for Sequence {
    type Error = Error;

    fn try_from(value: Micheline) -> Result<Self> {
        let sequence: micheline::sequence::Sequence = value.try_into()?;

        Ok(sequence
            .into_values()
            .into_iter()
            .map(|item| item.try_into())
            .collect::<Result<Vec<Instruction>>>()?
            .into())
    }
}

pub fn sequence(instructions: Vec<Instruction>) -> Michelson {
    Sequence::from(instructions).into()
}
