pub mod create_token;
pub use create_token::*;
use shank::ShankInstruction;

use pinocchio::program_error::ProgramError;

#[repr(u8)]
#[derive(ShankInstruction)]
pub enum Instruction {
    #[account(0, writable, signer, name = "payer", desc = "Pays for mint")]
    #[account(
        1,
        writable,
        signer,
        name = "mint",
        desc = "The mint account to create"
    )]
    #[account(2, name = "token_program", desc = "The token program to use")]
    #[account(3, name = "system_program", desc = "The system program")]
    CreateToken,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::CreateToken),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
