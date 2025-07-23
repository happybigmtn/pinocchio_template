use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instructions::{CounterInstruction, Create, Mutate};
use crate::state::MutationType;
use pinocchio_log::log;

#[inline(always)]
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CounterInstruction::try_from(discriminator)? {
        CounterInstruction::Create => {
            log!("CounterInstruction::Create");
            Create::try_from((accounts, data))?.handler()
        }
        CounterInstruction::Increase => {
            log!("CounterInstruction::Increase");
            Mutate::try_from(accounts)?.handler(MutationType::INCREASE)
        }
        CounterInstruction::Decrease => {
            log!("CounterInstruction::Decrease");
            Mutate::try_from(accounts)?.handler(MutationType::DECREASE)
        }
    }
}
