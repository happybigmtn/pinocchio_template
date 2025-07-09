use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::{instructions::CreateAccount};
use pinocchio_log::log;

use crate::{
    constants::*,
    error::CrapsError,
    state::ScalablePlayerState,
};

/// Handler for InitializePlayer instruction
pub fn initialize_player_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [player_state, player, global_game_state, system_program_account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate system program
    if system_program_account.key() != &pinocchio_system::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate player is signer
    if !player.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive and validate player state PDA
    let (player_state_pda, bump) = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    );
    if player_state.key() != &player_state_pda {
        log!("Invalid player state PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    // Check if already initialized
    if !player_state.data_is_empty() {
        return Err(CrapsError::AlreadyInitialized.into());
    }

    // Validate global game state
    let (game_state_pda, _) = pubkey::find_program_address(
        &[GLOBAL_GAME_STATE_SEED],
        &crate::ID,
    );
    if global_game_state.key() != &game_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Load global game state to verify it's initialized
    let game_state_data = global_game_state.try_borrow_data()?;
    if game_state_data.is_empty() {
        return Err(CrapsError::NotInitialized.into());
    }

    // Calculate space and rent
    let player_state_space = ScalablePlayerState::LEN;
    let rent = Rent::get()?;
    let player_state_rent = rent.minimum_balance(player_state_space);

    // Create player state account
    let bump_bytes = [bump];
    let player_seeds = &[
        pinocchio::instruction::Seed::from(SCALABLE_PLAYER_SEED),
        pinocchio::instruction::Seed::from(player.key().as_ref()),
        pinocchio::instruction::Seed::from(&bump_bytes),
    ];
    let signer = pinocchio::instruction::Signer::from(player_seeds);
    CreateAccount {
        from: player,
        to: player_state,
        lamports: player_state_rent,
        space: player_state_space as u64,
        owner: &crate::ID,
    }.invoke_signed(&[signer])?;

    // Initialize player state
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    let player_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..]);
    
    player_data.player = player.key().as_ref().try_into().unwrap();
    player_data.bump = bump;
    player_data.set_balance(0);
    player_data.set_total_deposited(0);
    player_data.set_total_withdrawn(0);
    player_data.set_total_wagered(0);
    player_data.set_total_won(0);
    player_data.set_active_tournament([0u8; 32]); // No active tournament
    player_data.set_last_claim_slot(0);
    player_data.set_games_played(0);
    player_data.set_bets_placed(0);
    player_data.set_bets_won(0);
    player_data.set_initialized_slot(pinocchio::sysvars::clock::Clock::get()?.slot);

    // Note: Player count tracking could be added as a separate field in GlobalGameState
    // For now, players are tracked individually through their ScalablePlayerState accounts

    log!("Player initialized successfully");
    log!("Player: {}", player.key());
    log!("Player state PDA: {}", player_state.key());

    Ok(())
}

/// Handler for ClosePlayerAccount instruction
pub fn close_player_account_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [player_state, player, receiver] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate player is signer
    if !player.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Derive and validate player state PDA
    let (player_state_pda, _) = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    );
    if player_state.key() != &player_state_pda {
        log!("Invalid player state PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    // Load player state to verify ownership and balance
    let player_state_data = player_state.try_borrow_data()?;
    let player_data = bytemuck::from_bytes::<ScalablePlayerState>(&player_state_data[..]);
    
    // Verify player ownership
    if player_data.player != player.key().as_ref() {
        return Err(CrapsError::InvalidPlayer.into());
    }

    // Check balance is zero
    if player_data.get_balance() != 0 {
        log!("Player has non-zero balance: {}", player_data.get_balance());
        return Err(CrapsError::InsufficientFunds.into());
    }

    // Check no active tournament
    let active_tournament = Pubkey::from(player_data.get_active_tournament());
    if active_tournament != Pubkey::default() {
        log!("Player has active tournament");
        return Err(CrapsError::ActiveTournament.into());
    }

    // Close the account and return lamports to receiver
    let player_state_lamports = player_state.lamports();
    *player_state.try_borrow_mut_lamports()? = 0;
    *receiver.try_borrow_mut_lamports()? = receiver
        .lamports()
        .checked_add(player_state_lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Clear the data
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    player_state_data.fill(0);

    log!("Player account closed successfully");
    log!("Returned {} lamports to {}", player_state_lamports, receiver.key());

    Ok(())
}