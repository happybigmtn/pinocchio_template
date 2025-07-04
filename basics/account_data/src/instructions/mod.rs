pub mod create;
pub use create::*;
use pinocchio::program_error::ProgramError;
use shank::ShankInstruction;
use crate::state::CreateAddressInfoInstructionData;

#[derive(ShankInstruction)]
#[repr(u8)]
pub enum Instruction {
    /// Create a new address info account with the provided address information
    #[account(0, writable, signer, name="payer", desc="The account that will pay for the transaction and rent")]
    #[account(1, writable, signer, name="address_info", desc="The address info account to create (must be a new keypair)")]
    #[account(2, name="system_program", desc="System Program for account creation")]
    Create(CreateAddressInfoInstructionData),
}

impl TryFrom<&u8> for Instruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            0 => {
                // Return a default instruction for now
                // In a real implementation, this would be handled differently
                let default_data = crate::state::CreateAddressInfoInstructionData {
                    name: [0u8; 50],
                    house_number: 0,
                    street: [0u8; 50],
                    city: [0u8; 50],
                };
                Ok(Instruction::Create(default_data))
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
