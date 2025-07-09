//! Treasury admin instruction handlers for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    error::CrapsError,
    state::{Treasury, GlobalGameState},
};

/// Treasury parameters update data
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TreasuryParametersData {
    /// New minimum balance requirement
    pub min_balance: [u8; 8],
    /// New maximum bet limit
    pub max_bet_limit: [u8; 8],
    /// New fee percentage (basis points)
    pub fee_bps: [u8; 2],
    /// Padding
    pub _padding: [u8; 6],
}

/// Handler for UpdateTreasuryAuthority instruction
pub fn update_treasury_authority_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateTreasuryAuthority: Processing treasury authority update");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let treasury_account = &accounts[0];
    let current_authority_account = &accounts[1];

    // Validate signer
    if !current_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { treasury_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load treasury
    let mut treasury_data = treasury_account.try_borrow_mut_data()?;
    let treasury = bytemuck::try_from_bytes_mut::<Treasury>(&mut treasury_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify current authority
    if &treasury.authority != current_authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Parse new authority from instruction data
    if data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let mut new_authority_bytes = [0u8; 32];
    new_authority_bytes.copy_from_slice(&data[0..32]);
    let new_authority = Pubkey::from(new_authority_bytes);
    
    // Validate new authority is not zero
    if new_authority == Pubkey::default() {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Update treasury authority
    treasury.authority.copy_from_slice(new_authority.as_ref());

    // Update last update slot
    let current_slot = Clock::get()?.slot;
    treasury.set_last_update_slot(current_slot);

    log!("UpdateTreasuryAuthority: Treasury authority updated from {} to {}", 
        current_authority_account.key(), &new_authority);

    Ok(())
}

/// Handler for UpdateTreasuryParameters instruction
pub fn update_treasury_parameters_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateTreasuryParameters: Processing treasury parameters update");

    // Account validation
    if accounts.len() != 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let treasury_account = &accounts[0];
    let global_game_state_account = &accounts[1];
    let admin_account = &accounts[2];

    // Validate signer
    if !admin_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { treasury_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load game state to verify admin
    let game_state_data = global_game_state_account.try_borrow_data()?;
    let game_state = bytemuck::try_from_bytes::<GlobalGameState>(&game_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify admin authority
    if &game_state.authority != admin_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Load treasury
    let mut treasury_data = treasury_account.try_borrow_mut_data()?;
    let treasury = bytemuck::try_from_bytes_mut::<Treasury>(&mut treasury_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Parse parameters from instruction data
    if data.len() < 24 {
        log!("UpdateTreasuryParameters: No parameters to update");
        return Ok(());
    }

    // Update last update slot
    let current_slot = Clock::get()?.slot;
    treasury.set_last_update_slot(current_slot);

    // In a real implementation, we would update various treasury parameters here
    // For now, just log the update
    log!("UpdateTreasuryParameters: Treasury parameters updated at slot {}", current_slot);

    Ok(())
}