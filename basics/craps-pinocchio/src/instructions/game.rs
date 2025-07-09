use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self, Pubkey},
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;

use crate::{
    constants::*,
    error::CrapsError,
    state::{GlobalGameState, BonusState, RngState, RngPhase},
    utils::{*, dice},
};

/// Handler for SecureAutoRoll instruction
pub fn secure_auto_roll_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 5 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [_global_game_state, bonus_state, rng_state, treasury, rng_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate RNG authority is signer
    if !rng_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate PDAs
    let (game_state_pda, _) = pubkey::find_program_address(
        &[GLOBAL_GAME_STATE_SEED],
        &crate::ID,
    );
    if _global_game_state.key() != &game_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (bonus_state_pda, _) = pubkey::find_program_address(
        &[BONUS_STATE_SEED],
        &crate::ID,
    );
    if bonus_state.key() != &bonus_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (rng_state_pda, _) = pubkey::find_program_address(
        &[RNG_STATE_SEED],
        &crate::ID,
    );
    if rng_state.key() != &rng_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (treasury_pda, _) = pubkey::find_program_address(
        &[TREASURY_SEED],
        &crate::ID,
    );
    if treasury.key() != &treasury_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Load and validate game state
    let mut game_state_data = _global_game_state.try_borrow_mut_data()?;
    let game_state = bytemuck::from_bytes_mut::<GlobalGameState>(&mut game_state_data[..]);

    // Verify RNG authority
    let expected_rng_authority = Pubkey::from(game_state.rng_authority);
    if rng_authority.key() != &expected_rng_authority {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Check game is not paused
    if game_state.paused != 0 {
        return Err(CrapsError::GamePaused.into());
    }

    // Load RNG state
    let rng_state_data = rng_state.try_borrow_data()?;
    let rng_data = bytemuck::from_bytes::<RngState>(&rng_state_data[..]);

    // Verify RNG is finalized for current epoch
    let current_epoch = game_state.get_game_epoch();
    validate_rng_phase(rng_data, RngPhase::Finalized, current_epoch)?;

    // Generate dice roll from finalized RNG value
    let final_value = rng_data.get_final_value();
    let (dice1, dice2) = generate_dice_from_final_value(final_value);
    
    // Validate dice values
    validate_dice_values(dice1, dice2)?;
    let dice_sum = calculate_roll_total(dice1, dice2);

    let roll_type = get_roll_type(dice1, dice2);
    log!("Auto roll executed: {} + {} = {} ({})", dice1, dice2, dice_sum, roll_type);

    // Load and update bonus state
    let mut bonus_state_data = bonus_state.try_borrow_mut_data()?;
    let bonus_data = bytemuck::from_bytes_mut::<BonusState>(&mut bonus_state_data[..]);
    
    // Update bonus state based on dice roll
    let current_phase = game_state.game_phase;
    let current_point = game_state.current_point;
    
    bonus_data.update_for_roll(dice1, dice2, dice_sum, current_point);

    match current_phase {
        PHASE_COME_OUT => {
            // Come out roll logic
            if dice::is_natural(dice1, dice2) {
                // Natural - Pass line wins
                log!("Natural! Pass line wins");
                // Stay in come out phase
            } else if dice::is_craps(dice1, dice2) {
                // Craps - Don't pass wins (except 12 is a push)
                log!("Craps! Don't pass wins");
                // Stay in come out phase  
            } else if is_valid_point_roll(dice1, dice2) {
                // Point established
                log!("Point established: {}", dice_sum);
                game_state.game_phase = PHASE_POINT;
                game_state.current_point = dice_sum;
            } else {
                return Err(CrapsError::InvalidDiceTotal.into());
            }
        }
        PHASE_POINT => {
            // Point phase logic
            if is_point_made(dice1, dice2, current_point) {
                // Point made - Pass line wins
                log!("Point made! Pass line wins");
                game_state.game_phase = PHASE_COME_OUT;
                game_state.current_point = 0;
            } else if is_seven_out(dice1, dice2) {
                // Seven out - Don't pass wins, advance epoch
                log!("Seven out! Don't pass wins");
                game_state.game_phase = PHASE_COME_OUT;
                game_state.current_point = 0;
                
                // Reset bonus state on seven-out
                bonus_data.reset_on_seven_out();
                
                // Advance epoch on seven-out
                let new_epoch = current_epoch + 1;
                game_state.set_game_epoch(new_epoch);
                log!("Epoch advanced to {} after seven-out", new_epoch);
            }
            // Otherwise continue in point phase
        }
        _ => return Err(CrapsError::InvalidPhase.into()),
    }

    // Update last roll information
    game_state.current_die1 = dice1;
    game_state.current_die2 = dice2;
    game_state.current_dice = dice_sum;
    game_state.set_next_roll_slot(Clock::get()?.slot + AUTO_ROLL_INTERVAL);

    // Increment roll count
    let roll_count = game_state.get_epoch_roll_count();
    game_state.set_epoch_roll_count(roll_count + 1);

    // Note: Epoch is only advanced on seven-out in point phase
    log!("Secure auto roll completed for epoch {}", current_epoch);

    Ok(())
}

/// Handler for CollectBlockHash instruction
pub fn collect_block_hash_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [rng_state, _global_game_state, rng_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate RNG authority
    if !rng_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate PDAs
    let (rng_state_pda, _) = pubkey::find_program_address(
        &[RNG_STATE_SEED],
        &crate::ID,
    );
    if rng_state.key() != &rng_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Load game state to verify authority
    let game_state_data = _global_game_state.try_borrow_data()?;
    let game_state = bytemuck::from_bytes::<GlobalGameState>(&game_state_data[..]);
    
    let expected_rng_authority = Pubkey::from(game_state.rng_authority);
    if rng_authority.key() != &expected_rng_authority {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Load RNG state
    let mut rng_state_data = rng_state.try_borrow_mut_data()?;
    let rng_data = bytemuck::from_bytes_mut::<RngState>(&mut rng_state_data[..]);

    // Check RNG phase - must be in collection phase
    if rng_data.get_phase() != RngPhase::Collection {
        return Err(CrapsError::InvalidRngPhase.into());
    }

    // Collect block hash (simplified - would use recent blockhashes sysvar)
    let hash_count = rng_data.hash_count;
    if hash_count >= MAX_BLOCK_HASHES {
        return Err(CrapsError::MaxBlockHashesReached.into());
    }

    // Store a dummy hash for now
    let dummy_hash = [hash_count + 1; 32];
    let offset = (hash_count as usize) * 32;
    rng_data.block_hashes[offset..offset + 32].copy_from_slice(&dummy_hash);
    rng_data.hash_count = hash_count + 1;

    log!("Collected block hash {} of {}", hash_count + 1, REQUIRED_BLOCK_HASHES);

    Ok(())
}

/// Handler for FinalizeRng instruction
pub fn finalize_rng_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [rng_state, _global_game_state, rng_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate RNG authority
    if !rng_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load and validate RNG state
    let mut rng_state_data = rng_state.try_borrow_mut_data()?;
    let rng_data = bytemuck::from_bytes_mut::<RngState>(&mut rng_state_data[..]);

    // Check we have enough block hashes
    if rng_data.hash_count < REQUIRED_BLOCK_HASHES {
        return Err(CrapsError::InsufficientBlockHashes.into());
    }

    // Check RNG phase - must be in collection phase
    if rng_data.get_phase() != RngPhase::Collection {
        return Err(CrapsError::InvalidRngPhase.into());
    }

    // Mix entropy from collected block hashes
    // Create temporary array of hashes from flattened storage
    let mut hashes = [[0u8; 32]; 10];
    for i in 0..(rng_data.hash_count as usize).min(10) {
        let offset = i * 32;
        hashes[i].copy_from_slice(&rng_data.block_hashes[offset..offset + 32]);
    }
    let mixed_entropy = mix_entropy(&hashes[..rng_data.hash_count as usize], rng_data.hash_count)?;
    
    // Generate final value from mixed entropy
    let final_value = u64::from_le_bytes([
        mixed_entropy[0], mixed_entropy[1], mixed_entropy[2], mixed_entropy[3],
        mixed_entropy[4], mixed_entropy[5], mixed_entropy[6], mixed_entropy[7],
    ]);
    
    // Update RNG state
    rng_data.set_final_value(final_value);
    rng_data.set_phase(RngPhase::Finalized);
    rng_data.set_finalization_slot(Clock::get()?.slot);

    log!("RNG finalized for epoch {}", rng_data.get_epoch());

    Ok(())
}

/// Handler for StartBettingPhase instruction
pub fn start_betting_phase_handler(
    accounts: & [AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 3 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [_global_game_state, rng_state, rng_authority] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate RNG authority
    if !rng_authority.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Load game state
    let mut game_state_data = _global_game_state.try_borrow_mut_data()?;
    let game_state = bytemuck::from_bytes_mut::<GlobalGameState>(&mut game_state_data[..]);

    // Verify authority
    let expected_rng_authority = Pubkey::from(game_state.rng_authority);
    if rng_authority.key() != &expected_rng_authority {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Load RNG state
    let mut rng_state_data = rng_state.try_borrow_mut_data()?;
    let rng_data = bytemuck::from_bytes_mut::<RngState>(&mut rng_state_data[..]);

    // Check if previous RNG cycle is complete
    let current_epoch = game_state.get_game_epoch();
    let current_slot = Clock::get()?.slot;
    
    // If RNG is already finalized for current epoch, reset for new betting phase
    if rng_data.get_epoch() == current_epoch && rng_data.is_finalized() {
        // Reset RNG state for new betting phase
        rng_data.reset_for_epoch(current_epoch, current_slot);
        log!("Reset RNG for new betting phase, epoch {}", current_epoch);
    } else if rng_data.get_epoch() < current_epoch {
        // Start new RNG cycle for new epoch
        rng_data.reset_for_epoch(current_epoch, current_slot);
        log!("Started new RNG cycle for epoch {}", current_epoch);
    } else {
        return Err(CrapsError::InvalidRngPhase.into());
    }

    log!("Started betting phase for epoch {}", current_epoch);

    Ok(())
}