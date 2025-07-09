//! Tournament instruction handlers for craps-pinocchio

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
    state::ScalablePlayerState,
};

/// Tournament update data
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TournamentUpdateData {
    /// The tournament public key
    pub tournament_pubkey: [u8; 32],
    /// Reserved for future use
    pub _reserved: [u8; 32],
}

/// Handler for UpdatePlayerTournament instruction
pub fn update_player_tournament_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("UpdatePlayerTournament: Processing tournament update");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let player_state_account = &accounts[0];
    let tournament_program_account = &accounts[1];

    // Validate signer (tournament program must sign)
    if !tournament_program_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { player_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load player state
    let mut player_state_data = player_state_account.try_borrow_mut_data()?;
    let player_state = bytemuck::try_from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Parse tournament data
    if data.len() < 32 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut tournament_pubkey_bytes = [0u8; 32];
    tournament_pubkey_bytes.copy_from_slice(&data[0..32]);
    let tournament_pubkey = Pubkey::from(tournament_pubkey_bytes);

    // Validate tournament pubkey is not zero
    if tournament_pubkey == Pubkey::default() {
        return Err(CrapsError::InvalidTournament.into());
    }

    // Check if player already has an active tournament
    let current_tournament = Pubkey::from(player_state.active_tournament);
    if current_tournament != Pubkey::default() && current_tournament != tournament_pubkey {
        log!("UpdatePlayerTournament: Player already in tournament {}", &current_tournament);
        return Err(CrapsError::AlreadyInTournament.into());
    }

    // Update tournament
    player_state.set_active_tournament(tournament_pubkey.as_ref().try_into().unwrap());
    
    // Update tournament slot
    let current_slot = Clock::get()?.slot;
    player_state.set_last_tournament_update_slot(Some(current_slot));

    log!("UpdatePlayerTournament: Player enrolled in tournament {}", &tournament_pubkey);

    Ok(())
}

/// Handler for ClearPlayerTournament instruction
pub fn clear_player_tournament_handler(
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("ClearPlayerTournament: Processing tournament clear");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let player_state_account = &accounts[0];
    let tournament_program_account = &accounts[1];

    // Validate signer (tournament program must sign)
    if !tournament_program_account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate account ownership
    if unsafe { player_state_account.owner() } != &crate::ID {
        return Err(ProgramError::IllegalOwner);
    }

    // Load player state
    let mut player_state_data = player_state_account.try_borrow_mut_data()?;
    let player_state = bytemuck::try_from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..])
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Get current tournament
    let current_tournament = Pubkey::from(player_state.active_tournament);
    
    // Check if player is in a tournament
    if current_tournament == Pubkey::default() {
        log!("ClearPlayerTournament: Player not in any tournament");
        return Ok(());
    }

    // Clear tournament
    player_state.set_active_tournament(Pubkey::default().as_ref().try_into().unwrap());
    
    // Update tournament slot
    let current_slot = Clock::get()?.slot;
    player_state.set_last_tournament_update_slot(Some(current_slot));

    log!("ClearPlayerTournament: Cleared tournament {} from player", &current_tournament);

    Ok(())
}