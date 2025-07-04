use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError,
    pubkey::Pubkey, ProgramResult,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    crate::processor::process_instruction(program_id, accounts, instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)
}
