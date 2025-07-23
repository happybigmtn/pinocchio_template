use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_log::log;

use crate::processor::{create_token, Instruction};

entrypoint!(process_instruction);

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

    match Instruction::try_from(discriminator)? {
        Instruction::CreateToken => {
            log!("Instruction: CreateToken");
            CreateToken::try_from((accounts, data))?.handler()
        }
    }
}
