use pinocchio::{program_error::ProgramError, log};
use shank::ShankInstruction;

#[derive(ShankInstruction)]
#[rustfmt::skip]
pub enum CounterInstruction {
    /// Initialize a new counter
    #[account(0, name = "counter", desc = "The counter account to initialize")]
    #[account(1, name = "payer", desc = "The account paying for initialization", signer)]
    #[account(2, name = "system_program", desc = "The system program")]
    Initialize,

    /// Increment the counter
    #[account(0, name = "counter", desc = "The counter account to increment", writable)]
    #[account(1, name = "authority", desc = "The counter authority", signer)]
    Increment,

    /// Decrement the counter
    #[account(0, name = "counter", desc = "The counter account to decrement", writable)]
    #[account(1, name = "authority", desc = "The counter authority", signer)]
    Decrement,
}

impl CounterInstruction {
    pub fn try_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }

        match data[0] {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Increment),
            2 => Ok(Self::Decrement),
            _ => {
                log!("Invalid instruction discriminator: {}", data[0]);
                Err(ProgramError::InvalidInstructionData)
            }
        }
    }
}
