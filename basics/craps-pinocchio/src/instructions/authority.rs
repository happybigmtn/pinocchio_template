//! Authority management instruction handlers for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    error::CrapsError,
    state::GlobalGameState,
};

/// Pending authority transfer state (stored in instruction data)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AuthorityTransferData {
    /// The new authority pubkey
    pub new_authority: [u8; 32],
    /// The type of authority being transferred (0=main, 1=rng, 2=admin, 3=emergency)
    pub authority_type: u8,
    /// Padding for alignment
    pub _padding: [u8; 7],
}

/// Handler for UpdateAuthority instruction
pub fn update_authority_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateAuthority: Processing authority update");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let current_authority_account = &accounts[1];

    // Validate signer
    if !current_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load game state
    let mut game_state_data = global_game_state_account.try_borrow_mut_data()?;
    let game_state = bytemuck::try_from_bytes_mut::<GlobalGameState>(&mut game_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify current authority
    if &game_state.authority != current_authority_account.key().as_ref() {
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

    // Update authority
    game_state.authority.copy_from_slice(new_authority.as_ref());

    log!("UpdateAuthority: Authority updated from {} to {}", 
        current_authority_account.key(), &new_authority);

    Ok(())
}

/// Handler for UpdateRngAuthority instruction
pub fn update_rng_authority_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateRngAuthority: Processing RNG authority update");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let current_authority_account = &accounts[1];

    // Validate signer
    if !current_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load game state
    let mut game_state_data = global_game_state_account.try_borrow_mut_data()?;
    let game_state = bytemuck::try_from_bytes_mut::<GlobalGameState>(&mut game_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Verify current authority (main authority can update RNG authority)
    if &game_state.authority != current_authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Parse new RNG authority from instruction data
    if data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    
    let mut new_rng_authority_bytes = [0u8; 32];
    new_rng_authority_bytes.copy_from_slice(&data[0..32]);
    let new_rng_authority = Pubkey::from(new_rng_authority_bytes);
    
    // Validate new authority is not zero
    if new_rng_authority == Pubkey::default() {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Update RNG authority
    game_state.rng_authority.copy_from_slice(new_rng_authority.as_ref());

    log!("UpdateRngAuthority: RNG authority updated to {}", &new_rng_authority);

    Ok(())
}

/// Handler for UpdateAdminAuthority instruction
pub fn update_admin_authority_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateAdminAuthority: Processing admin authority update");

    // For now, admin authority is the same as main authority
    // This could be extended to have a separate admin role
    update_authority_handler(accounts, data)
}

/// Handler for UpdateEmergencyAuthority instruction
pub fn update_emergency_authority_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdateEmergencyAuthority: Processing emergency authority update");

    // For now, emergency authority is the same as main authority
    // This could be extended to have a separate emergency role
    update_authority_handler(accounts, data)
}

/// Handler for ExecuteAuthorityTransfer instruction
pub fn execute_authority_transfer_handler(
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("ExecuteAuthorityTransfer: Processing authority transfer execution");

    // In a full implementation, this would:
    // 1. Check for a pending authority transfer
    // 2. Verify the new authority is signing
    // 3. Complete the transfer
    // 4. Clear the pending transfer
    
    // For now, we'll implement a simple direct transfer
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let new_authority_account = &accounts[1];

    // Validate signer
    if !new_authority_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { global_game_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load game state
    let mut game_state_data = global_game_state_account.try_borrow_mut_data()?;
    let _game_state = bytemuck::try_from_bytes_mut::<GlobalGameState>(&mut game_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // In a real implementation, we would check for a pending transfer here
    // For now, we'll just log that this instruction was called
    log!("ExecuteAuthorityTransfer: Called by {}", new_authority_account.key());
    
    // Return an error indicating this needs full implementation
    Err(CrapsError::NotYetImplemented.into())
}