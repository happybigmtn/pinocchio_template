use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_log::log;

use crate::processor::{create_pda::CreatePda, get_pda::GetPda, Instruction};

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
        Instruction::CreatePda => {
            log!("Instrucction: CreatePda");
            CreatePda::try_from((accounts, data))?.handler()
        }
        Instruction::GetPda => {
            log!("Instrucction: GetPda");
            GetPda::try_from(accounts)?.handler()
        }
    }
}
