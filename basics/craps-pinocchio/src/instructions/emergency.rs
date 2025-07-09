//! Emergency instruction handlers for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    constants::*,
    error::CrapsError,
    state::GlobalGameState,
};

/// Handler for EmergencyShutdown instruction
pub fn emergency_shutdown_handler(
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("EmergencyShutdown: Initiating emergency shutdown");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let emergency_authority_account = &accounts[1];

    // Validate signer
    if !emergency_authority_account.is_signer() {
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

    // Verify emergency authority (for now, using main authority)
    if &game_state.authority != emergency_authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Check if already paused
    if game_state.paused != 0 {
        log!("EmergencyShutdown: Game is already paused");
        return Ok(());
    }

    // Set pause flag
    game_state.paused = 1;

    // Record current slot for tracking
    let current_slot = Clock::get()?.slot;
    game_state.set_epoch_start_slot(current_slot); // Reuse this field to track pause time

    log!("EmergencyShutdown: Emergency shutdown activated at slot {}", current_slot);

    // Emit emergency action event
    let clock = Clock::get()?;
    let current_epoch = game_state.get_game_epoch();
    crate::events::emit_emergency_action(
        emergency_authority_account.key(),
        0, // Shutdown action
        0, // Previous state (not paused)
        1, // New state (paused)
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}

/// Handler for ResumeOperations instruction
pub fn resume_operations_handler(
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("ResumeOperations: Resuming normal operations");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let emergency_authority_account = &accounts[1];

    // Validate signer
    if !emergency_authority_account.is_signer() {
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

    // Verify emergency authority
    if &game_state.authority != emergency_authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Check if not paused
    if game_state.paused == 0 {
        log!("ResumeOperations: Game is not paused");
        return Ok(());
    }

    // Clear pause flag
    game_state.paused = 0;

    // Update next roll slot to resume game flow
    let current_slot = Clock::get()?.slot;
    game_state.set_next_roll_slot(current_slot + SLOTS_PER_ROLL);

    log!("ResumeOperations: Normal operations resumed at slot {}", current_slot);

    // Emit emergency action event
    let clock = Clock::get()?;
    let current_epoch = game_state.get_game_epoch();
    crate::events::emit_emergency_action(
        emergency_authority_account.key(),
        1, // Resume operations action
        1, // Previous state (paused)
        0, // New state (not paused)
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}

/// Handler for EmergencyPause instruction
pub fn emergency_pause_handler(
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    log!("EmergencyPause: Processing emergency pause");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let emergency_authority_account = &accounts[1];

    // Validate signer
    if !emergency_authority_account.is_signer() {
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

    // Verify emergency authority
    if &game_state.authority != emergency_authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Parse pause duration from instruction data (optional)
    let pause_duration_slots = if data.len() >= 8 {
        u64::from_le_bytes(data[0..8].try_into().unwrap())
    } else {
        0 // Indefinite pause
    };

    // Set pause flag
    game_state.paused = 1;

    // Record pause details
    let current_slot = Clock::get()?.slot;
    game_state.set_epoch_start_slot(current_slot); // Track when paused
    
    if pause_duration_slots > 0 {
        // Set when to automatically resume
        game_state.set_next_roll_slot(current_slot + pause_duration_slots);
        log!("EmergencyPause: Game paused for {} slots", pause_duration_slots);
    } else {
        log!("EmergencyPause: Game paused indefinitely");
    }

    // Emit emergency action event
    let clock = Clock::get()?;
    let current_epoch = game_state.get_game_epoch();
    crate::events::emit_emergency_action(
        emergency_authority_account.key(),
        2, // Emergency pause action
        0, // Previous state (not paused)
        1, // New state (paused)
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}

/// Handler for ResumeGame instruction
pub fn resume_game_handler(
    accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("ResumeGame: Processing game resume");

    // Account validation
    if accounts.len() != 2 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let global_game_state_account = &accounts[0];
    let authority_account = &accounts[1];

    // Validate signer
    if !authority_account.is_signer() {
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

    // Verify authority
    if &game_state.authority != authority_account.key().as_ref() {
        return Err(CrapsError::UnauthorizedAuthority.into());
    }

    // Check if game is paused
    if game_state.paused == 0 {
        log!("ResumeGame: Game is already running");
        return Ok(());
    }

    // Clear pause flag
    game_state.paused = 0;

    // Calculate next roll slot
    let current_slot = Clock::get()?.slot;
    game_state.set_next_roll_slot(current_slot + SLOTS_PER_ROLL);

    log!("ResumeGame: Game resumed at slot {}", current_slot);

    // Emit emergency action event
    let clock = Clock::get()?;
    let current_epoch = game_state.get_game_epoch();
    crate::events::emit_emergency_action(
        authority_account.key(),
        3, // Resume game action
        1, // Previous state (paused)
        0, // New state (not paused)
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}