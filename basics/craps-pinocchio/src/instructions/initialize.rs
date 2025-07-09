use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::{instructions::CreateAccount};
use pinocchio_log::log;
use bytemuck::{Pod, Zeroable};

use crate::{
    constants::*,
    error::CrapsError,
    state::{GlobalGameState, Treasury, RngState, BonusState},
};

/// Instruction data for InitializeSystem
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct InitializeSystemData {
    /// The public key of the RNG authority
    pub rng_authority: [u8; 32],
}

/// Handler for InitializeSystem instruction
pub fn initialize_system_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 6 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [global_game_state, treasury, bonus_state, rng_state, authority, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate system program
    if system_program.key() != &pinocchio_system::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Validate authority is signer
    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Parse instruction data
    if data.len() < core::mem::size_of::<InitializeSystemData>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let init_data = bytemuck::from_bytes::<InitializeSystemData>(
        &data[..core::mem::size_of::<InitializeSystemData>()]
    );
    let rng_authority = Pubkey::from(init_data.rng_authority);

    // Derive and validate PDAs
    let (game_state_pda, game_state_bump) = pubkey::find_program_address(
        &[GLOBAL_GAME_STATE_SEED],
        &crate::ID,
    );
    if global_game_state.key() != &game_state_pda {
        log!("Invalid global game state PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    let (treasury_pda, treasury_bump) = pubkey::find_program_address(
        &[TREASURY_SEED],
        &crate::ID,
    );
    if treasury.key() != &treasury_pda {
        log!("Invalid treasury PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    let (bonus_state_pda, bonus_state_bump) = pubkey::find_program_address(
        &[BONUS_STATE_SEED],
        &crate::ID,
    );
    if bonus_state.key() != &bonus_state_pda {
        log!("Invalid bonus state PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    let (rng_state_pda, rng_state_bump) = pubkey::find_program_address(
        &[RNG_STATE_SEED],
        &crate::ID,
    );
    if rng_state.key() != &rng_state_pda {
        log!("Invalid RNG state PDA");
        return Err(CrapsError::InvalidPDA.into());
    }

    // Check if already initialized
    if !global_game_state.data_is_empty() {
        return Err(CrapsError::AlreadyInitialized.into());
    }

    // Allocate space for accounts
    let game_state_space = GlobalGameState::LEN;
    let treasury_space = Treasury::LEN;
    let bonus_state_space = BonusState::LEN;
    let rng_state_space = RngState::LEN;

    // Calculate rent
    let rent = Rent::get()?;
    let game_state_rent = rent.minimum_balance(game_state_space);
    let treasury_rent = rent.minimum_balance(treasury_space);
    let bonus_state_rent = rent.minimum_balance(bonus_state_space);
    let rng_state_rent = rent.minimum_balance(rng_state_space);

    // Create global game state account
    let game_state_bump_bytes = [game_state_bump];
    let global_state_seeds = &[Seed::from(GLOBAL_GAME_STATE_SEED), Seed::from(&game_state_bump_bytes)];
    let global_state_signer = Signer::from(global_state_seeds);
    CreateAccount {
        from: authority,
        to: global_game_state,
        lamports: game_state_rent,
        space: game_state_space as u64,
        owner: &crate::ID,
    }.invoke_signed(&[global_state_signer])?;

    // Create treasury account
    let treasury_bump_bytes = [treasury_bump];
    let treasury_seeds = &[Seed::from(TREASURY_SEED), Seed::from(&treasury_bump_bytes)];
    let treasury_signer = Signer::from(treasury_seeds);
    CreateAccount {
        from: authority,
        to: treasury,
        lamports: treasury_rent,
        space: treasury_space as u64,
        owner: &crate::ID,
    }.invoke_signed(&[treasury_signer])?;

    // Create bonus state account
    let bonus_state_bump_bytes = [bonus_state_bump];
    let bonus_state_seeds = &[Seed::from(BONUS_STATE_SEED), Seed::from(&bonus_state_bump_bytes)];
    let bonus_state_signer = Signer::from(bonus_state_seeds);
    CreateAccount {
        from: authority,
        to: bonus_state,
        lamports: bonus_state_rent,
        space: bonus_state_space as u64,
        owner: &crate::ID,
    }.invoke_signed(&[bonus_state_signer])?;

    // Create RNG state account
    let rng_state_bump_bytes = [rng_state_bump];
    let rng_state_seeds = &[Seed::from(RNG_STATE_SEED), Seed::from(&rng_state_bump_bytes)];
    let rng_state_signer = Signer::from(rng_state_seeds);
    CreateAccount {
        from: authority,
        to: rng_state,
        lamports: rng_state_rent,
        space: rng_state_space as u64,
        owner: &crate::ID,
    }.invoke_signed(&[rng_state_signer])?;

    // Initialize global game state
    let mut game_state_data = global_game_state.try_borrow_mut_data()?;
    let game_state = bytemuck::from_bytes_mut::<GlobalGameState>(&mut game_state_data[..]);
    
    game_state.authority = authority.key().as_ref().try_into().unwrap();
    game_state.rng_authority = rng_authority;
    game_state.set_current_epoch(0);
    game_state.set_current_phase(PHASE_COME_OUT);
    game_state.set_total_players(0);
    game_state.set_total_games_played(0);
    game_state.set_total_deposited(0);
    game_state.set_total_wagered(0);
    game_state.set_total_paid_out(0);
    game_state.set_last_updated_slot(pinocchio::sysvars::clock::Clock::get()?.slot);
    game_state.set_secure_rng_enabled(false);
    game_state.set_is_paused(false);
    game_state.set_is_emergency_shutdown(false);
    game_state.set_auto_roll_enabled(true);

    // Initialize treasury
    let mut treasury_data = treasury.try_borrow_mut_data()?;
    let treasury_state = bytemuck::from_bytes_mut::<Treasury>(&mut treasury_data[..]);
    
    treasury_state.authority = authority.key().as_ref().try_into().unwrap();
    treasury_state.set_total_balance(0);
    treasury_state.set_locked_amount(0);
    treasury_state.set_insurance_pool(0);
    treasury_state.set_total_fees_collected(0);
    treasury_state.set_total_refunded(0);
    treasury_state.set_last_reconciliation_slot(pinocchio::sysvars::clock::Clock::get()?.slot);
    treasury_state.set_safety_multiplier(TREASURY_SAFETY_MULTIPLIER);
    treasury_state.set_reserve_percentage(TREASURY_RESERVE_PERCENTAGE as u64);

    // Initialize bonus state
    let mut bonus_state_data = bonus_state.try_borrow_mut_data()?;
    let bonus_data = bytemuck::from_bytes_mut::<BonusState>(&mut bonus_state_data[..]);
    
    // BonusState::new() sets all fields to 0, which is correct for initialization
    *bonus_data = BonusState::new();

    // Initialize RNG state
    let mut rng_state_data = rng_state.try_borrow_mut_data()?;
    let rng_data = bytemuck::from_bytes_mut::<RngState>(&mut rng_state_data[..]);
    
    rng_data.set_epoch(0);
    rng_data.set_rng_phase(0); // Idle
    rng_data.set_collection_start_slot(0);
    rng_data.set_hash_count(0);
    rng_data.set_last_finalized_epoch(0);
    rng_data.set_last_update_slot(pinocchio::sysvars::clock::Clock::get()?.slot);
    rng_data.set_total_collections(0);
    rng_data.set_successful_finalizations(0);
    rng_data.set_failed_finalizations(0);
    rng_data.set_is_active(true);

    log!("System initialized successfully");
    log!("Authority: {}", authority.key());
    log!("RNG Authority: {}", rng_authority.len());

    Ok(())
}

/// Handler for InitializeCriticalPDAs instruction
pub fn initialize_critical_pdas_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // This is a placeholder for initializing other critical PDAs
    // In the full implementation, this would create additional PDAs like:
    // - Authority config
    // - Rate limit accounts
    // - Auto roll timer
    // etc.
    
    if accounts.len() < 4 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [global_game_state, treasury, rng_state, authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate authority
    if !authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate that core PDAs exist
    if global_game_state.data_is_empty() {
        return Err(CrapsError::NotInitialized.into());
    }

    if treasury.data_is_empty() {
        return Err(CrapsError::NotInitialized.into());
    }

    if rng_state.data_is_empty() {
        return Err(CrapsError::NotInitialized.into());
    }

    log!("Critical PDAs verified");

    Ok(())
}