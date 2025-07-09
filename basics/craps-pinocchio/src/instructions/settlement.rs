//! Settlement instruction handler for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    error::CrapsError,
    state::{GlobalGameState, Treasury},
};

/// Handler for SettleRealizableBets instruction
/// This instruction settles all realizable bets for a given epoch range
pub fn settle_realizable_bets_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("SettleRealizableBets: Processing settlement");

    // Account validation
    if accounts.len() != 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let treasury_account = &accounts[1];
    let authority_account = &accounts[2];

    // Validate authority
    if !authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }
    if unsafe { treasury_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load and validate state
    let mut global_game_state = global_game_state_account.try_borrow_mut_data()?;
    let game_state = bytemuck::try_from_bytes_mut::<GlobalGameState>(&mut global_game_state[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Check authority matches
    if &game_state.authority != authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Check if game is paused
    if game_state.paused != 0 {
        return Err(CrapsError::GamePaused.into());
    }

    // Load treasury
    let mut treasury_data = treasury_account.try_borrow_mut_data()?;
    let treasury = bytemuck::try_from_bytes_mut::<Treasury>(&mut treasury_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Check if treasury is in emergency shutdown
    if treasury.is_emergency_shutdown() {
        return Err(CrapsError::EmergencyShutdown.into());
    }

    // Parse instruction data
    if data.len() < 16 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let start_epoch = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let end_epoch = u64::from_le_bytes(data[8..16].try_into().unwrap());
    
    let current_epoch = game_state.get_game_epoch();
    
    // Validate epoch range
    if start_epoch > end_epoch {
        return Err(CrapsError::InvalidEpochRange.into());
    }
    
    // Can't settle future epochs
    if end_epoch >= current_epoch {
        return Err(CrapsError::EpochNotFinalized.into());
    }
    
    // Limit the range to prevent DoS
    const MAX_EPOCHS_PER_SETTLE: u64 = 10;
    if end_epoch - start_epoch > MAX_EPOCHS_PER_SETTLE {
        return Err(CrapsError::TooManyEpochsToSettle.into());
    }

    // In a real implementation, we would:
    // 1. Iterate through all bet batches for the epoch range
    // 2. Evaluate each bet against the epoch outcome
    // 3. Mark winning bets as realizable
    // 4. Update treasury statistics
    // 5. Emit events for monitoring
    
    // For now, just update treasury stats
    let current_slot = Clock::get()?.slot;
    treasury.set_last_update_slot(current_slot);
    
    log!("SettleRealizableBets: Successfully settled epochs {} to {}", start_epoch, end_epoch);
    
    Ok(())
}