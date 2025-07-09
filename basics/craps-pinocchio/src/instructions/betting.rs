use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self},
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use pinocchio_system::{instructions::CreateAccount};
use bytemuck::{Pod, Zeroable};

use crate::{
    constants::*,
    error::CrapsError,
    state::{GlobalGameState, ScalablePlayerState, BetBatch},
    utils::validation::{validate_bet_amount, validate_bet_type},
};

/// Instruction data for PlaceBet
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct PlaceBetData {
    /// The epoch to place the bet for
    pub epoch: [u8; 8],
    /// The type of bet (0-63)
    pub bet_kind: u8,
    /// Padding for alignment
    pub _padding1: [u8; 7],
    /// The amount to bet
    pub bet_amount: [u8; 8],
    /// Padding for alignment
    pub _padding2: [u8; 8],
}

/// Handler for PlaceBet instruction
pub fn place_bet_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [bet_batch, player_state, global_game_state, player, system_program_account] = accounts else {
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

    // Parse instruction data
    if data.len() < core::mem::size_of::<PlaceBetData>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let bet_data = bytemuck::from_bytes::<PlaceBetData>(
        &data[..core::mem::size_of::<PlaceBetData>()]
    );
    
    let epoch = u64::from_le_bytes(bet_data.epoch);
    let bet_kind = bet_data.bet_kind;
    let bet_amount = u64::from_le_bytes(bet_data.bet_amount);

    // Validate bet type
    validate_bet_type(bet_kind)?;

    // Validate bet amount
    validate_bet_amount(bet_amount)?;


    // Validate PDAs
    let game_state_pda = pubkey::find_program_address(
        &[GLOBAL_GAME_STATE_SEED],
        &crate::ID,
    ).0;
    if global_game_state.key() != &game_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let player_state_pda = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    ).0;
    if player_state.key() != &player_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Load global game state
    let game_state_data = global_game_state.try_borrow_data()?;
    let game_state = bytemuck::from_bytes::<GlobalGameState>(&game_state_data[..]);

    // Check game is not paused or shutdown
    if game_state.get_is_paused() {
        return Err(CrapsError::GamePaused.into());
    }
    if game_state.get_is_emergency_shutdown() {
        return Err(CrapsError::EmergencyShutdown.into());
    }

    // Validate epoch
    let current_epoch = game_state.get_current_epoch();
    if epoch != current_epoch {
        log!("Invalid epoch: {} vs current {}", epoch, current_epoch);
        return Err(CrapsError::InvalidEpoch.into());
    }

    // Check betting window is open (simplified check)
    // In full implementation, would check RNG state and betting phase
    
    // Load and update player state
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    let player_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..]);
    
    // Verify player ownership
    if player_data.player != player.key().as_ref() {
        return Err(CrapsError::InvalidPlayer.into());
    }

    // Check player balance
    let current_balance = player_data.get_balance();
    if current_balance < bet_amount {
        log!("Insufficient balance: {} < {}", current_balance, bet_amount);
        return Err(CrapsError::InsufficientFunds.into());
    }

    // Derive bet batch PDA
    let batch_index = 0u32; // Simplified - would find next available batch
    let (bet_batch_pda, bet_batch_bump) = pubkey::find_program_address(
        &[
            BET_BATCH_SEED,
            player.key().as_ref(),
            &epoch.to_le_bytes(),
            &batch_index.to_le_bytes(),
        ],
        &crate::ID,
    );
    if bet_batch.key() != &bet_batch_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Create or load bet batch
    if bet_batch.data_is_empty() {
        // Create new bet batch
        let bet_batch_space = BetBatch::LEN;
        let rent = pinocchio::sysvars::rent::Rent::get()?;
        let bet_batch_rent = rent.minimum_balance(bet_batch_space);

        // Create longer-lived bindings for seed references
        let epoch_bytes = epoch.to_le_bytes();
        let batch_index_bytes = batch_index.to_le_bytes();
        let bump_bytes = [bet_batch_bump];
        
        let bet_batch_seeds = &[
            Seed::from(BET_BATCH_SEED),
            Seed::from(player.key().as_ref()),
            Seed::from(&epoch_bytes),
            Seed::from(&batch_index_bytes),
            Seed::from(&bump_bytes),
        ];
        let signer_seeds = Signer::from(bet_batch_seeds);

        CreateAccount {
            from: player,
            to: bet_batch,
            lamports: bet_batch_rent,
            space: bet_batch_space as u64,
            owner: &crate::ID,
        }.invoke_signed(&[signer_seeds])?;

        // Initialize bet batch
        let mut bet_batch_data = bet_batch.try_borrow_mut_data()?;
        let batch = bytemuck::from_bytes_mut::<BetBatch>(&mut bet_batch_data[..]);
        
        batch.player = player.key().as_ref().try_into().unwrap();
        batch.set_epoch(epoch);
        batch.bet_count = 0;
        batch.set_total_amount(0);
        batch.bump = bet_batch_bump;
        // Initialize masks to 0
        batch.set_resolved_mask(0);
        batch.set_realizable_mask(0);
        batch.set_settled_mask(0);
        batch.set_winning_mask(0);
    }

    // Load bet batch
    let mut bet_batch_data = bet_batch.try_borrow_mut_data()?;
    let batch = bytemuck::from_bytes_mut::<BetBatch>(&mut bet_batch_data[..]);

    // Check batch has space
    let bet_count = batch.bet_count as usize;
    if bet_count >= MAX_BETS_PER_BATCH {
        return Err(CrapsError::BatchFull.into());
    }

    // Add bet to batch using packed format
    // Pack bet data: kind (6 bits) + amount_index (10 bits) = 16 bits
    let packed_bet = ((bet_kind as u16) & 0x3F) | (((bet_amount as u16) & 0x3FF) << 6);
    batch.set_packed_bet(bet_count, packed_bet);
    
    // Update bet count
    batch.bet_count += 1;
    
    // Update total amount
    let current_total = batch.get_total_amount();
    batch.set_total_amount(current_total + bet_amount);
    

    // Update player balance and stats
    player_data.set_balance(current_balance - bet_amount);
    let total_wagered = player_data.get_total_wagered();
    player_data.set_total_wagered(total_wagered + bet_amount);
    // Update last bet slot
    player_data.last_bet_slot = Clock::get()?.slot.to_le_bytes();

    log!("Bet placed successfully");
    log!("Player: {}", player.key());
    log!("Epoch: {}", epoch);
    log!("Bet type: {}", bet_kind);
    log!("Amount: {}", bet_amount);

    Ok(())
}