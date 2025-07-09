//! Cleanup instruction handlers for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    error::CrapsError,
    state::{GlobalGameState, ScalablePlayerState, BetBatch},
    instructions::claim::EpochOutcome,
};

/// Handler for CleanupBetBatch instruction
/// Allows players to clean up their own bet batches after claiming payouts
pub fn cleanup_bet_batch_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("CleanupBetBatch: Processing cleanup");

    // Account validation
    if accounts.len() != 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let bet_batch_account = &accounts[0];
    let player_state_account = &accounts[1];
    let player_account = &accounts[2];

    // Validate signer
    if !player_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { bet_batch_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }
    if unsafe { player_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load player state
    let player_state_data = player_state_account.try_borrow_data()?;
    let player_state = bytemuck::try_from_bytes::<ScalablePlayerState>(&player_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify player matches
    if &player_state.player != player_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedPlayer.into());
    }

    // Load bet batch
    let mut bet_batch_data = bet_batch_account.try_borrow_mut_data()?;
    let bet_batch = bytemuck::try_from_bytes_mut::<BetBatch>(&mut bet_batch_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify bet batch belongs to player
    if &bet_batch.player != player_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedPlayer.into());
    }

    // Check if all bets have been settled
    let all_settled_mask = (1u16 << bet_batch.bet_count) - 1;
    if bet_batch.get_settled_mask() != all_settled_mask {
        return Err(CrapsError::BetsNotSettled.into());
    }

    // Parse epoch from instruction data
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let epoch = u64::from_le_bytes(data[0..8].try_into().unwrap());

    // Verify epoch matches
    if bet_batch.get_epoch() != epoch {
        return Err(CrapsError::EpochMismatch.into());
    }

    // Close the account and return rent to player
    let bet_batch_lamports = bet_batch_account.lamports();
    *bet_batch_account.try_borrow_mut_lamports()? = 0;
    *player_account.try_borrow_mut_lamports()? += bet_batch_lamports;

    // Zero out the account data
    bet_batch_account.try_borrow_mut_data()?.fill(0);

    log!("CleanupBetBatch: Successfully cleaned up bet batch for epoch {}", epoch);

    Ok(())
}

/// Handler for CleanupOldEpochOutcome instruction
/// Allows admin to clean up old epoch outcome accounts
pub fn cleanup_old_epoch_outcome_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("CleanupOldEpochOutcome: Processing cleanup");

    // Account validation
    if accounts.len() != 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let epoch_outcome_account = &accounts[0];
    let global_game_state_account = &accounts[1];
    let admin_account = &accounts[2];

    // Validate signer
    if !admin_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { epoch_outcome_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load global game state
    let game_state_data = global_game_state_account.try_borrow_data()?;
    let game_state = bytemuck::try_from_bytes::<GlobalGameState>(&game_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify admin authority
    if &game_state.authority != admin_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Load epoch outcome
    let epoch_outcome_data = epoch_outcome_account.try_borrow_data()?;
    let epoch_outcome = bytemuck::try_from_bytes::<EpochOutcome>(&epoch_outcome_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Parse minimum age from instruction data
    if data.len() < 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let minimum_age_slots = u64::from_le_bytes(data[0..8].try_into().unwrap());

    // Check if epoch outcome is old enough
    let current_slot = Clock::get()?.slot;
    let outcome_age = current_slot.saturating_sub(epoch_outcome.get_finalized_slot());
    
    if outcome_age < minimum_age_slots {
        return Err(CrapsError::EpochTooRecent.into());
    }

    // Ensure epoch is fully settled (at least 100 epochs old)
    let current_epoch = game_state.get_game_epoch();
    let outcome_epoch = epoch_outcome.get_epoch();
    
    const MIN_EPOCH_AGE: u64 = 100;
    if current_epoch.saturating_sub(outcome_epoch) < MIN_EPOCH_AGE {
        return Err(CrapsError::EpochTooRecent.into());
    }

    // Close the account and return rent to admin
    let outcome_lamports = epoch_outcome_account.lamports();
    *epoch_outcome_account.try_borrow_mut_lamports()? = 0;
    *admin_account.try_borrow_mut_lamports()? += outcome_lamports;

    // Zero out the account data
    epoch_outcome_account.try_borrow_mut_data()?.fill(0);

    log!("CleanupOldEpochOutcome: Successfully cleaned up epoch outcome for epoch {}", outcome_epoch);

    Ok(())
}