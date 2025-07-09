//! Claim instruction handlers for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self},
    ProgramResult,
};
use pinocchio_log::log;
use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

use crate::{
    constants::*,
    error::CrapsError,
    state::{Treasury, ScalablePlayerState, BetBatch, BonusState},
    utils::{
        token::{validate_token_account, get_token_balance, transfer_tokens},
        bet_encoding::{decode_bet_from_batch, BetMetadata},
    },
};

/// Instruction data for claim operations
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ClaimData {
    /// The epoch to claim payouts for
    pub epoch: [u8; 8],
    /// Reserved for future use
    pub reserved: [u8; 8],
}

/// EpochOutcome state for tracking resolved bets
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct EpochOutcome {
    /// The epoch number
    pub epoch: [u8; 8],
    /// The dice roll result (die1, die2)
    pub dice: [u8; 2],
    /// The game phase at time of roll
    pub phase: u8,
    /// The point number (0 if come-out phase)
    pub point: u8,
    /// Whether this outcome has been resolved (0=false, 1=true)
    pub resolved: u8,
    /// Padding for alignment
    pub _padding: [u8; 3],
    /// The total amount of payouts for this epoch
    pub total_payouts: [u8; 8],
    /// The slot when this outcome was finalized
    pub finalized_slot: [u8; 8],
    /// Reserved for future use
    pub _reserved: [u8; 32],
}

impl EpochOutcome {
    pub const LEN: usize = 72;
    
    pub fn get_epoch(&self) -> u64 {
        u64::from_le_bytes(self.epoch)
    }
    
    pub fn get_total_payouts(&self) -> u64 {
        u64::from_le_bytes(self.total_payouts)
    }
    
    pub fn get_finalized_slot(&self) -> u64 {
        u64::from_le_bytes(self.finalized_slot)
    }
    
    pub fn get_dice_total(&self) -> u8 {
        self.dice[0] + self.dice[1]
    }
}

/// Handler for ClaimEpochPayoutsUnified instruction
pub fn claim_epoch_payouts_unified_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 10 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [
        player,
        player_state,
        treasury,
        treasury_token_account,
        player_token_account,
        epoch_outcome,
        bet_batch,
        bonus_state,
        token_program,
        mint,
    ] = &accounts[..10] else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate player is signer
    if !player.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate token program
    if token_program.key() != &pinocchio_token::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse instruction data
    if data.len() < core::mem::size_of::<ClaimData>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let claim_data = bytemuck::from_bytes::<ClaimData>(
        &data[..core::mem::size_of::<ClaimData>()]
    );
    let epoch = u64::from_le_bytes(claim_data.epoch);

    // Validate PDAs
    let (treasury_pda, treasury_bump) = pubkey::find_program_address(
        &[TREASURY_SEED],
        &crate::ID,
    );
    if treasury.key() != &treasury_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (player_state_pda, _) = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    );
    if player_state.key() != &player_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (epoch_outcome_pda, _) = pubkey::find_program_address(
        &[EPOCH_OUTCOME_SEED, &epoch.to_le_bytes()],
        &crate::ID,
    );
    if epoch_outcome.key() != &epoch_outcome_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (bet_batch_pda, _) = pubkey::find_program_address(
        &[BET_BATCH_SEED, player.key().as_ref(), &epoch.to_le_bytes()],
        &crate::ID,
    );
    if bet_batch.key() != &bet_batch_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Validate token accounts
    validate_token_account(treasury_token_account, &treasury_pda, mint.key())?;
    validate_token_account(player_token_account, player.key(), mint.key())?;

    // Load epoch outcome
    let epoch_data = epoch_outcome.try_borrow_data()?;
    if epoch_data.len() < EpochOutcome::LEN {
        return Err(CrapsError::InvalidAccount.into());
    }
    let outcome = bytemuck::from_bytes::<EpochOutcome>(&epoch_data[..EpochOutcome::LEN]);
    
    // Verify epoch matches
    if outcome.get_epoch() != epoch {
        return Err(CrapsError::InvalidEpoch.into());
    }
    
    // Verify outcome is resolved
    if outcome.resolved == 0 {
        return Err(CrapsError::RngNotFinalized.into());
    }

    // Load bet batch
    let mut bet_batch_data = bet_batch.try_borrow_mut_data()?;
    let batch = bytemuck::from_bytes_mut::<BetBatch>(&mut bet_batch_data[..]);
    
    // Check if already claimed
    // Check if batch is already fully settled
    if batch.get_settled_mask() == 0xFFFF {
        return Err(CrapsError::AlreadyClaimed.into());
    }
    
    // Verify player ownership
    if batch.player != player.key().as_ref() {
        return Err(CrapsError::InvalidPlayer.into());
    }
    
    // Verify epoch matches
    if batch.get_epoch() != epoch {
        return Err(CrapsError::InvalidEpoch.into());
    }

    // Load bonus state
    let bonus_data = bonus_state.try_borrow_data()?;
    if bonus_data.len() < BonusState::LEN {
        return Err(CrapsError::InvalidAccount.into());
    }
    let bonus = bytemuck::from_bytes::<BonusState>(&bonus_data[..BonusState::LEN]);
    
    // Calculate total payout for all bets in the batch
    let mut total_payout = 0u64;
    let dice_total = outcome.get_dice_total();
    let phase = outcome.phase;
    let point = outcome.point;
    
    // Process each bet in the batch
    for i in 0..batch.bet_count as usize {
        let bet_metadata = decode_bet_from_batch(batch, i)?;
        let payout = calculate_bet_payout(&bet_metadata, dice_total, phase, point, bonus)?;
        
        total_payout = total_payout
            .checked_add(payout)
            .ok_or(ProgramError::ArithmeticOverflow)?;
    }

    // If there's a payout, transfer tokens
    if total_payout > 0 {
        // Check treasury has sufficient balance
        let treasury_token_balance = get_token_balance(treasury_token_account)?;
        if treasury_token_balance < total_payout {
            return Err(CrapsError::InsufficientTreasuryFunds.into());
        }

        // Perform the token transfer from treasury to player
        let treasury_bump_bytes = [treasury_bump];
        let treasury_seeds = &[TREASURY_SEED, &treasury_bump_bytes];
        transfer_tokens(
            treasury_token_account,
            player_token_account,
            treasury,
            token_program,
            mint,
            total_payout,
            9, // TOKEN_DECIMALS for $CRAP tokens
            treasury_seeds,
        )?;
    }

    // Mark all bets as settled
    let all_bets_mask = (1u16 << batch.bet_count) - 1;
    batch.set_settled_mask(all_bets_mask);

    // Update player state
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    let player_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..]);
    
    // Update player balance
    let current_balance = player_data.get_balance();
    let new_balance = current_balance
        .checked_add(total_payout)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    player_data.set_balance(new_balance);
    
    // Update total won
    let total_won = player_data.get_total_won();
    player_data.set_total_won(
        total_won.checked_add(total_payout)
            .ok_or(ProgramError::ArithmeticOverflow)?
    );

    // Update treasury
    let mut treasury_data = treasury.try_borrow_mut_data()?;
    let treasury_state = bytemuck::from_bytes_mut::<Treasury>(&mut treasury_data[..]);
    
    let _total_deposits = treasury_state.get_total_deposits();
    let total_payouts = treasury_state.get_total_payouts();
    treasury_state.set_total_payouts(
        total_payouts.checked_add(total_payout)
            .ok_or(ProgramError::ArithmeticOverflow)?
    );
    
    // Update treasury stats - remove locked amount since bets are settled
    let total_settled = treasury_state.get_total_bets_settled();
    treasury_state.set_total_bets_settled(
        total_settled.checked_add(batch.bet_count as u64)
            .ok_or(ProgramError::ArithmeticOverflow)?
    );

    log!("Claim successful");
    log!("Player: {}", player.key());
    log!("Epoch: {}", epoch);
    log!("Total payout: {}", total_payout);
    log!("New balance: {}", new_balance);

    Ok(())
}

/// Calculate the payout for a single bet based on the outcome
/// Full implementation for all 64 bet types (0-63)
fn calculate_bet_payout(
    bet: &BetMetadata,
    dice_total: u8,
    phase: u8,
    point: u8,
    bonus: &BonusState,
) -> Result<u64, ProgramError> {
    let amount = bet.amount;
    let dice1 = if dice_total >= 2 && dice_total <= 12 { dice_total / 2 } else { 1 };
    let dice2 = dice_total - dice1;
    let is_hard = dice1 == dice2;
    
    match bet.bet_type {
        // Core game bets (0-4)
        BET_PASS => evaluate_pass_line(amount, dice_total, phase, point),
        BET_DONT_PASS => evaluate_dont_pass(amount, dice_total, phase, point),
        BET_COME => evaluate_come(amount, dice_total, phase, point),
        BET_DONT_COME => evaluate_dont_come(amount, dice_total, phase, point),
        BET_FIELD => evaluate_field(amount, dice_total),
        
        // YES bets (5-14) - number will be rolled before 7
        5 => evaluate_yes_bet(amount, dice_total, 2), // Yes2
        6 => evaluate_yes_bet(amount, dice_total, 3), // Yes3
        7 => evaluate_yes_bet(amount, dice_total, 4), // Yes4
        8 => evaluate_yes_bet(amount, dice_total, 5), // Yes5
        9 => evaluate_yes_bet(amount, dice_total, 6), // Yes6
        10 => evaluate_yes_bet(amount, dice_total, 8), // Yes8
        11 => evaluate_yes_bet(amount, dice_total, 9), // Yes9
        12 => evaluate_yes_bet(amount, dice_total, 10), // Yes10
        13 => evaluate_yes_bet(amount, dice_total, 11), // Yes11
        14 => evaluate_yes_bet(amount, dice_total, 12), // Yes12
        
        // NO bets (15-24) - 7 will be rolled before the number
        15 => evaluate_no_bet(amount, dice_total, 2), // No2
        16 => evaluate_no_bet(amount, dice_total, 3), // No3
        17 => evaluate_no_bet(amount, dice_total, 4), // No4
        18 => evaluate_no_bet(amount, dice_total, 5), // No5
        19 => evaluate_no_bet(amount, dice_total, 6), // No6
        20 => evaluate_no_bet(amount, dice_total, 8), // No8
        21 => evaluate_no_bet(amount, dice_total, 9), // No9
        22 => evaluate_no_bet(amount, dice_total, 10), // No10
        23 => evaluate_no_bet(amount, dice_total, 11), // No11
        24 => evaluate_no_bet(amount, dice_total, 12), // No12
        
        // Hard ways (25-28)
        25 => evaluate_hard_way(amount, dice_total, 4, is_hard), // Hard4
        26 => evaluate_hard_way(amount, dice_total, 6, is_hard), // Hard6
        27 => evaluate_hard_way(amount, dice_total, 8, is_hard), // Hard8
        28 => evaluate_hard_way(amount, dice_total, 10, is_hard), // Hard10
        
        // Odds bets (29-32)
        29 => evaluate_odds_pass(amount, dice_total, phase, point), // OddsPass
        30 => evaluate_odds_dont_pass(amount, dice_total, phase, point), // OddsDontPass
        31 => evaluate_odds_come(amount, dice_total, phase, point), // OddsCome
        32 => evaluate_odds_dont_come(amount, dice_total, phase, point), // OddsDontCome
        
        // Complex/special bets (33-42)
        33 => evaluate_hot_roller(amount, dice_total, phase, bonus), // HotRoller
        34 => evaluate_fire_bet(amount, dice_total, phase, bonus), // Fire
        35 => evaluate_twice_hard(amount, dice_total, is_hard, bonus), // TwiceHard
        36 => evaluate_ride_line(amount, dice_total, phase, point, bonus), // RideLine
        37 => evaluate_muggsy(amount, dice_total, phase, point), // Muggsy
        38 => evaluate_bonus_small(amount, dice_total, phase, bonus), // BonusSmall
        39 => evaluate_bonus_tall(amount, dice_total, phase, bonus), // BonusTall
        40 => evaluate_bonus_small_tall(amount, dice_total, phase, bonus), // BonusSmallTall
        41 => evaluate_replay(amount, dice_total, phase, point, bonus), // Replay
        42 => evaluate_different_doubles(amount, dice_total, is_hard, phase, bonus), // DifferentDoubles
        
        // NEXT bets (43-53) - one-roll bets
        43 => evaluate_next_bet(amount, dice_total, 2), // Next2
        44 => evaluate_next_bet(amount, dice_total, 3), // Next3
        45 => evaluate_next_bet(amount, dice_total, 4), // Next4
        46 => evaluate_next_bet(amount, dice_total, 5), // Next5
        47 => evaluate_next_bet(amount, dice_total, 6), // Next6
        48 => evaluate_next_bet(amount, dice_total, 7), // Next7
        49 => evaluate_next_bet(amount, dice_total, 8), // Next8
        50 => evaluate_next_bet(amount, dice_total, 9), // Next9
        51 => evaluate_next_bet(amount, dice_total, 10), // Next10
        52 => evaluate_next_bet(amount, dice_total, 11), // Next11
        53 => evaluate_next_bet(amount, dice_total, 12), // Next12
        
        // Individual Repeater bets (54-63)
        54 => evaluate_repeater_bet(amount, dice_total, 2, phase, bonus), // Repeater2
        55 => evaluate_repeater_bet(amount, dice_total, 3, phase, bonus), // Repeater3
        56 => evaluate_repeater_bet(amount, dice_total, 4, phase, bonus), // Repeater4
        57 => evaluate_repeater_bet(amount, dice_total, 5, phase, bonus), // Repeater5
        58 => evaluate_repeater_bet(amount, dice_total, 6, phase, bonus), // Repeater6
        59 => evaluate_repeater_bet(amount, dice_total, 8, phase, bonus), // Repeater8
        60 => evaluate_repeater_bet(amount, dice_total, 9, phase, bonus), // Repeater9
        61 => evaluate_repeater_bet(amount, dice_total, 10, phase, bonus), // Repeater10
        62 => evaluate_repeater_bet(amount, dice_total, 11, phase, bonus), // Repeater11
        63 => evaluate_repeater_bet(amount, dice_total, 12, phase, bonus), // Repeater12
        
        // Invalid bet type
        _ => Ok(0),
    }
}

/// Evaluate Pass Line bet
fn evaluate_pass_line(amount: u64, dice_total: u8, phase: u8, point: u8) -> Result<u64, ProgramError> {
    if phase == PHASE_COME_OUT {
        match dice_total {
            7 | 11 => Ok(amount * 2), // Win on natural
            2 | 3 | 12 => Ok(0), // Lose on craps
            _ => Ok(0), // Point established, bet continues
        }
    } else {
        // Point phase
        if dice_total == point {
            Ok(amount * 2) // Win on point
        } else if dice_total == 7 {
            Ok(0) // Seven out
        } else {
            Ok(0) // Bet continues
        }
    }
}

/// Evaluate Don't Pass bet
fn evaluate_dont_pass(amount: u64, dice_total: u8, phase: u8, point: u8) -> Result<u64, ProgramError> {
    if phase == PHASE_COME_OUT {
        match dice_total {
            7 | 11 => Ok(0), // Lose on natural
            2 | 3 => Ok(amount * 2), // Win on craps
            12 => Ok(amount), // Push on 12
            _ => Ok(0), // Point established, bet continues
        }
    } else {
        // Point phase
        if dice_total == 7 {
            Ok(amount * 2) // Win on seven out
        } else if dice_total == point {
            Ok(0) // Lose on point
        } else {
            Ok(0) // Bet continues
        }
    }
}

/// Evaluate Come bet
fn evaluate_come(amount: u64, dice_total: u8, _phase: u8, _point: u8) -> Result<u64, ProgramError> {
    // Come bet acts like pass line on every roll
    match dice_total {
        7 | 11 => Ok(amount * 2), // Win on natural
        2 | 3 | 12 => Ok(0), // Lose on craps
        _ => Ok(0), // Come point established, bet continues
    }
}

/// Evaluate Don't Come bet
fn evaluate_dont_come(amount: u64, dice_total: u8, _phase: u8, _point: u8) -> Result<u64, ProgramError> {
    // Don't Come bet acts like don't pass on every roll
    match dice_total {
        7 | 11 => Ok(0), // Lose on natural
        2 | 3 => Ok(amount * 2), // Win on craps
        12 => Ok(amount), // Push on 12
        _ => Ok(0), // Don't come point established, bet continues
    }
}

/// Evaluate Field bet
fn evaluate_field(amount: u64, dice_total: u8) -> Result<u64, ProgramError> {
    match dice_total {
        2 => Ok(amount * 3), // 2:1 payout
        3 | 4 | 9 | 10 | 11 => Ok(amount * 2), // 1:1 payout
        12 => Ok(amount * 4), // 3:1 payout
        _ => Ok(0), // Lose on 5, 6, 7, 8
    }
}

/// Evaluate YES bet (number before 7)
fn evaluate_yes_bet(amount: u64, dice_total: u8, target: u8) -> Result<u64, ProgramError> {
    if dice_total == target {
        // Target hit, calculate payout based on odds
        let payout_multiplier = match target {
            2 => 7, // 6:1 payout
            3 => 4, // 3:1 payout
            4 => 3, // 2:1 payout
            5 => amount + (amount * 3) / 2, // 3:2 payout
            6 => amount + (amount * 6) / 5, // 6:5 payout
            8 => amount + (amount * 6) / 5, // 6:5 payout
            9 => amount + (amount * 3) / 2, // 3:2 payout
            10 => 3, // 2:1 payout
            11 => 4, // 3:1 payout
            12 => 7, // 6:1 payout
            _ => 1,
        };
        Ok(if target == 5 || target == 6 || target == 8 || target == 9 {
            payout_multiplier
        } else {
            amount * payout_multiplier
        })
    } else if dice_total == 7 {
        Ok(0) // Seven out, bet loses
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate NO bet (7 before number)
fn evaluate_no_bet(amount: u64, dice_total: u8, target: u8) -> Result<u64, ProgramError> {
    if dice_total == 7 {
        // Seven hit, calculate payout based on odds
        let payout_multiplier = match target {
            2 => amount + amount / 6, // 1:6 payout
            3 => amount + amount / 3, // 1:3 payout
            4 => amount + amount / 2, // 1:2 payout
            5 => amount + (amount * 2) / 3, // 2:3 payout
            6 => amount + (amount * 5) / 6, // 5:6 payout
            8 => amount + (amount * 5) / 6, // 5:6 payout
            9 => amount + (amount * 2) / 3, // 2:3 payout
            10 => amount + amount / 2, // 1:2 payout
            11 => amount + amount / 3, // 1:3 payout
            12 => amount + amount / 6, // 1:6 payout
            _ => amount,
        };
        Ok(payout_multiplier)
    } else if dice_total == target {
        Ok(0) // Target hit, bet loses
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Hard Way bet
fn evaluate_hard_way(amount: u64, dice_total: u8, target: u8, is_hard: bool) -> Result<u64, ProgramError> {
    if dice_total == target && is_hard {
        // Hard way hit
        let payout_multiplier = match target {
            4 => 8, // 7:1 payout
            6 => 10, // 9:1 payout
            8 => 10, // 9:1 payout
            10 => 8, // 7:1 payout
            _ => 1,
        };
        Ok(amount * payout_multiplier)
    } else if dice_total == target || dice_total == 7 {
        Ok(0) // Easy way or seven out, bet loses
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Odds Pass bet
fn evaluate_odds_pass(amount: u64, dice_total: u8, phase: u8, point: u8) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && point != 0 {
        if dice_total == point {
            // Point hit, calculate true odds payout
            let payout = match point {
                4 | 10 => amount * 3, // 2:1 payout
                5 | 9 => amount + (amount * 3) / 2, // 3:2 payout
                6 | 8 => amount + (amount * 6) / 5, // 6:5 payout
                _ => amount,
            };
            Ok(payout)
        } else if dice_total == 7 {
            Ok(0) // Seven out, bet loses
        } else {
            Ok(0) // Bet continues
        }
    } else {
        Ok(0) // No point established
    }
}

/// Evaluate Odds Don't Pass bet
fn evaluate_odds_dont_pass(amount: u64, dice_total: u8, phase: u8, point: u8) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && point != 0 {
        if dice_total == 7 {
            // Seven out, calculate true odds payout
            let payout = match point {
                4 | 10 => amount + amount / 2, // 1:2 payout
                5 | 9 => amount + (amount * 2) / 3, // 2:3 payout
                6 | 8 => amount + (amount * 5) / 6, // 5:6 payout
                _ => amount,
            };
            Ok(payout)
        } else if dice_total == point {
            Ok(0) // Point hit, bet loses
        } else {
            Ok(0) // Bet continues
        }
    } else {
        Ok(0) // No point established
    }
}

/// Evaluate Odds Come bet
fn evaluate_odds_come(amount: u64, dice_total: u8, _phase: u8, _point: u8) -> Result<u64, ProgramError> {
    // Simplified - would need come point tracking
    Ok(0) // Bet continues
}

/// Evaluate Odds Don't Come bet
fn evaluate_odds_dont_come(amount: u64, dice_total: u8, _phase: u8, _point: u8) -> Result<u64, ProgramError> {
    // Simplified - would need come point tracking
    Ok(0) // Bet continues
}

/// Evaluate Hot Roller bet
fn evaluate_hot_roller(amount: u64, dice_total: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check hot roller count
        let payout = match bonus.hot_roller_count {
            6 => amount * 200, // 199:1 payout
            5 => amount * 50,  // 49:1 payout
            4 => amount * 20,  // 19:1 payout
            3 => amount * 10,  // 9:1 payout
            2 => amount * 5,   // 4:1 payout
            _ => 0,
        };
        Ok(payout)
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Fire bet
fn evaluate_fire_bet(amount: u64, dice_total: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check fire points
        let payout = match bonus.fire_points {
            4 => amount * 25,   // 24:1 payout
            5 => amount * 250,  // 249:1 payout
            6 => amount * 1000, // 999:1 payout
            _ => 0,
        };
        Ok(payout)
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Twice Hard bet
fn evaluate_twice_hard(amount: u64, dice_total: u8, is_hard: bool, bonus: &BonusState) -> Result<u64, ProgramError> {
    if dice_total == 7 {
        // Seven out - check if any double has been rolled at least twice
        if bonus.has_twice_hard() {
            Ok(amount * 9) // 8:1 payout
        } else {
            Ok(0) // Bet loses
        }
    } else if is_hard {
        Ok(0) // Continue tracking
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Ride Line bet
fn evaluate_ride_line(amount: u64, dice_total: u8, phase: u8, point: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check pass line wins
        let payout = match bonus.ride_line_streak {
            3 => amount * 2,   // 1:1 payout
            4 => amount * 3,   // 2:1 payout
            5 => amount * 4,   // 3:1 payout
            6 => amount * 5,   // 4:1 payout
            7 => amount * 11,  // 10:1 payout
            8 => amount * 16,  // 15:1 payout
            9 => amount * 21,  // 20:1 payout
            10 => amount * 31, // 30:1 payout
            11.. => amount * 151, // 150:1 payout for 11+
            _ => 0,
        };
        Ok(payout)
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Muggsy's Corner bet
fn evaluate_muggsy(amount: u64, dice_total: u8, phase: u8, point: u8) -> Result<u64, ProgramError> {
    if phase == PHASE_COME_OUT && dice_total == 7 {
        Ok(amount * 3) // 2:1 payout on come-out 7
    } else if phase != PHASE_COME_OUT && dice_total == 7 {
        Ok(amount * 4) // 3:1 payout on seven after point
    } else {
        Ok(0) // Bet continues or loses
    }
}

/// Evaluate Bonus Small bet
fn evaluate_bonus_small(amount: u64, dice_total: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check if all small numbers (2-6) have been rolled
        if bonus.all_small_rolled() {
            Ok(amount * 37) // 36:1 payout
        } else {
            Ok(0) // Bet loses
        }
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Bonus Tall bet
fn evaluate_bonus_tall(amount: u64, dice_total: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check if all tall numbers (8-12) have been rolled
        if bonus.all_tall_rolled() {
            Ok(amount * 37) // 36:1 payout
        } else {
            Ok(0) // Bet loses
        }
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Bonus Small & Tall bet
fn evaluate_bonus_small_tall(amount: u64, dice_total: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check if both small and tall numbers have been rolled
        if bonus.all_small_rolled() && bonus.all_tall_rolled() {
            Ok(amount * 181) // 180:1 payout
        } else {
            Ok(0) // Bet loses
        }
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Replay bet
fn evaluate_replay(amount: u64, dice_total: u8, phase: u8, point: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check pass wins on same point
        let wins_for_point = bonus.get_pass_win_count(point);
        
        let payout = match (point, wins_for_point) {
            // 4/10 points
            (4 | 10, 3) => amount * 121,   // 120:1 payout
            (4 | 10, 4..) => amount * 1001, // 1000:1 payout
            // 5/9 points  
            (5 | 9, 3) => amount * 96,     // 95:1 payout
            (5 | 9, 4..) => amount * 501,  // 500:1 payout
            // 6/8 points
            (6 | 8, 3) => amount * 71,     // 70:1 payout
            (6 | 8, 4..) => amount * 101,  // 100:1 payout
            _ => 0,
        };
        Ok(payout)
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate Different Doubles bet
fn evaluate_different_doubles(amount: u64, dice_total: u8, is_hard: bool, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check how many different doubles have been rolled
        let unique_count = bonus.count_doubles_rolled();
        let payout = match unique_count {
            3 => amount * 5,   // 4:1 payout
            4 => amount * 9,   // 8:1 payout
            5 => amount * 16,  // 15:1 payout
            6 => amount * 101, // 100:1 payout
            _ => 0,
        };
        Ok(payout)
    } else if is_hard {
        Ok(0) // Continue tracking
    } else {
        Ok(0) // Bet continues
    }
}

/// Evaluate NEXT bet (one-roll)
fn evaluate_next_bet(amount: u64, dice_total: u8, target: u8) -> Result<u64, ProgramError> {
    if dice_total == target {
        // Calculate payout based on true odds
        let payout_multiplier = match target {
            2 => 36, // 35:1 payout
            3 => 18, // 17:1 payout
            4 => 12, // 11:1 payout
            5 => 9, // 8:1 payout
            6 => 8, // 7:1 payout
            7 => 5, // 4:1 payout
            8 => 8, // 7:1 payout
            9 => 9, // 8:1 payout
            10 => 12, // 11:1 payout
            11 => 18, // 17:1 payout
            12 => 36, // 35:1 payout
            _ => 1,
        };
        Ok(amount * payout_multiplier)
    } else {
        Ok(0) // Bet loses
    }
}

/// Evaluate Repeater bet
fn evaluate_repeater_bet(amount: u64, dice_total: u8, target: u8, phase: u8, bonus: &BonusState) -> Result<u64, ProgramError> {
    if phase != PHASE_COME_OUT && dice_total == 7 {
        // Seven out - check if target has been hit the required number of times
        let hit_count = bonus.get_hit_count(target);
        let required_hits = match target {
            2 => 2,
            3 => 3,
            4 => 4,
            5 => 5,
            6 => 6,
            8 => 6,
            9 => 5,
            10 => 4,
            11 => 3,
            12 => 2,
            _ => 0,
        };
        
        if hit_count >= required_hits {
            let payout_multiplier = match target {
                2 => 41, // 40:1 payout
                3 => 51, // 50:1 payout
                4 => 66, // 65:1 payout
                5 => 81, // 80:1 payout
                6 => 91, // 90:1 payout
                8 => 91, // 90:1 payout
                9 => 81, // 80:1 payout
                10 => 66, // 65:1 payout
                11 => 51, // 50:1 payout
                12 => 41, // 40:1 payout
                _ => 1,
            };
            Ok(amount * payout_multiplier)
        } else {
            Ok(0) // Bet loses
        }
    } else {
        Ok(0) // Bet continues
    }
}