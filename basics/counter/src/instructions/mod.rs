pub mod create;
pub use create::*;
use pinocchio::program_error::ProgramError;
use shank::ShankInstruction;

#[derive(ShankInstruction)]
#[repr(u8)]
pub enum Instruction {
    /// Create a new address info account with the provided address information
    #[account(
        0,
        writable,
        signer,
        name = "payer",
        desc = "The account that will pay for the transaction and rent"
    )]
    #[account(
        1,
        writable,
        signer,
        name = "address_info",
        desc = "The address info account to create (must be a new keypair)"
    )]
    #[account(
        2,
        name = "system_program",
        desc = "System Program for account creation"
    )]
    Create,
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(Instruction::Create),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
