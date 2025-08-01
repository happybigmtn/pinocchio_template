pub mod create_pda;
pub mod get_pda;
pub mod shared;

pub use create_pda::*;
pub use get_pda::*;
use shank::ShankInstruction;
pub use shared::*;

use pinocchio::program_error::ProgramError;

#[repr(u8)]
#[derive(ShankInstruction)]
pub enum Instruction {
    CreatePda,
    GetPda,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::CreatePda),
            1 => Ok(Instruction::GetPda),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
