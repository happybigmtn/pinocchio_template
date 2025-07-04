use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    log,
};

use crate::instructions::CounterInstruction;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    log!("Processing counter instruction");
    
    let instruction = CounterInstruction::try_from_bytes(instruction_data)?;
    
    match instruction {
        CounterInstruction::Initialize => {
            log!("Instruction: Initialize");
            // TODO: Implement initialize logic
            Ok(())
        }
        CounterInstruction::Increment => {
            log!("Instruction: Increment");
            // TODO: Implement increment logic
            Ok(())
        }
        CounterInstruction::Decrement => {
            log!("Instruction: Decrement");
            // TODO: Implement decrement logic
            Ok(())
        }
    }
}
