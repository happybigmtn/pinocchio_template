pub mod create;
pub use create::*;
pub mod mutate;
pub use mutate::*;

use pinocchio::program_error::ProgramError;
use shank::ShankInstruction;

#[derive(ShankInstruction)]
#[repr(u8)]
pub enum CounterInstruction {
    #[account(0, writable, signer, name = "maker", desc = "The payer of the counter")]
    #[account(1, writable, name = "counter", desc = "The counter account")]
    #[account(2, name = "system_program", desc = "The system program")]
    Create,

    #[account(0, writable, signer, name = "authority", desc = "Counter authority")]
    #[account(1, writable, name = "counter", desc = "The counter account")]
    Increase,

    #[account(0, writable, signer, name = "authority", desc = "Counter authority")]
    #[account(1, writable, name = "counter", desc = "The counter account")]
    Decrease,
}

impl TryFrom<&u8> for CounterInstruction {
    type Error = ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CounterInstruction::Create),
            1 => Ok(CounterInstruction::Increase),
            2 => Ok(CounterInstruction::Decrease),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
