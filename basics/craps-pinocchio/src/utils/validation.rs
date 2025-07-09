//! Validation utilities for bet types, dice rolls, and game state

use crate::constants::*;
use crate::error::CrapsError;
use pinocchio::program_error::ProgramError;

/// Validate a bet type is valid
pub fn validate_bet_type(bet_type: u8) -> Result<(), ProgramError> {
    if bet_type > BET_REPEATER_12 {
        return Err(CrapsError::InvalidBetKind.into());
    }
    Ok(())
}

/// Validate dice values are within valid range
pub fn validate_dice_roll(die1: u8, die2: u8) -> Result<u8, ProgramError> {
    if die1 < DICE_MIN_VALUE || die1 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceTotal.into());
    }
    if die2 < DICE_MIN_VALUE || die2 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceTotal.into());
    }
    
    let total = die1 + die2;
    if total < DICE_MIN_SUM || total > DICE_MAX_SUM {
        return Err(CrapsError::InvalidDiceTotal.into());
    }
    
    Ok(total)
}

/// Validate a bet can be placed in the current game phase
pub fn validate_bet_for_phase(bet_type: u8, game_phase: u8, current_point: u8) -> Result<(), ProgramError> {
    match bet_type {
        // Pass/Don't Pass only on come-out
        BET_PASS | BET_DONT_PASS => {
            if game_phase != PHASE_COME_OUT {
                return Err(CrapsError::InvalidBetForPhase.into());
            }
        }
        
        // Come/Don't Come only during point phase
        BET_COME | BET_DONT_COME => {
            if game_phase != PHASE_POINT || current_point == 0 {
                return Err(CrapsError::InvalidBetForPhase.into());
            }
        }
        
        // Odds bets require point phase
        BET_ODDS_PASS | BET_ODDS_DONT_PASS | BET_ODDS_COME | BET_ODDS_DONT_COME => {
            if game_phase != PHASE_POINT {
                return Err(CrapsError::OddsBetRequiresBase.into());
            }
        }
        
        // Bonus bets only on come-out
        BET_BONUS_SMALL | BET_BONUS_TALL | BET_BONUS_SMALL_TALL => {
            if game_phase != PHASE_COME_OUT {
                return Err(CrapsError::BonusOnlyComeOut.into());
            }
        }
        
        // Fire bet only on come-out
        BET_FIRE => {
            if game_phase != PHASE_COME_OUT {
                return Err(CrapsError::FireRequiresComeOut.into());
            }
        }
        
        // Hot Roller only on come-out
        BET_HOT_ROLLER => {
            if game_phase != PHASE_COME_OUT {
                return Err(CrapsError::HotRollerRequiresComeOut.into());
            }
        }
        
        // Repeater bets only on come-out
        BET_REPEATER_2..=BET_REPEATER_12 => {
            if game_phase != PHASE_COME_OUT {
                return Err(CrapsError::RepeaterRequiresComeOut.into());
            }
        }
        
        // All other bets can be placed anytime
        _ => {}
    }
    
    Ok(())
}

/// Check if a number is a valid point
pub fn is_valid_point(num: u8) -> bool {
    VALID_POINTS.contains(&num)
}

/// Check if a roll is a natural (7 or 11)
pub fn is_natural(total: u8) -> bool {
    total == NATURAL_SEVEN || total == NATURAL_ELEVEN
}

/// Check if a roll is craps (2, 3, or 12)
pub fn is_craps(total: u8) -> bool {
    total == CRAPS_TWO || total == CRAPS_THREE || total == CRAPS_TWELVE
}

/// Check if dice form a hard way (same value on both dice)
pub fn is_hard_way(die1: u8, die2: u8) -> bool {
    die1 == die2
}

/// Validate bet amount is within limits
pub fn validate_bet_amount(amount: u64) -> Result<(), ProgramError> {
    if amount < MIN_BET_AMOUNT || amount > MAX_BET_AMOUNT {
        return Err(CrapsError::InvalidBetAmount.into());
    }
    Ok(())
}

/// Validate deposit amount
pub fn validate_deposit_amount(amount: u64) -> Result<(), ProgramError> {
    if amount == 0 || amount > MAX_DEPOSIT_AMOUNT {
        return Err(CrapsError::InvalidDepositAmount.into());
    }
    Ok(())
}

/// Validate withdrawal amount
pub fn validate_withdrawal_amount(amount: u64, balance: u64) -> Result<(), ProgramError> {
    if amount == 0 {
        return Err(CrapsError::InvalidWithdrawalAmount.into());
    }
    if amount > MAX_WITHDRAWAL_AMOUNT {
        return Err(CrapsError::WithdrawalExceedsLimit.into());
    }
    if amount > balance {
        return Err(CrapsError::InsufficientBalance.into());
    }
    Ok(())
}

/// Get target number for YES bets
pub fn get_yes_bet_target(bet_type: u8) -> Option<u8> {
    match bet_type {
        BET_YES_2 => Some(2),
        BET_YES_3 => Some(3),
        BET_YES_4 => Some(4),
        BET_YES_5 => Some(5),
        BET_YES_6 => Some(6),
        BET_YES_8 => Some(8),
        BET_YES_9 => Some(9),
        BET_YES_10 => Some(10),
        BET_YES_11 => Some(11),
        BET_YES_12 => Some(12),
        _ => None,
    }
}

/// Get target number for NO bets
pub fn get_no_bet_target(bet_type: u8) -> Option<u8> {
    match bet_type {
        BET_NO_2 => Some(2),
        BET_NO_3 => Some(3),
        BET_NO_4 => Some(4),
        BET_NO_5 => Some(5),
        BET_NO_6 => Some(6),
        BET_NO_8 => Some(8),
        BET_NO_9 => Some(9),
        BET_NO_10 => Some(10),
        BET_NO_11 => Some(11),
        BET_NO_12 => Some(12),
        _ => None,
    }
}

/// Get target number for NEXT bets
pub fn get_next_bet_target(bet_type: u8) -> Option<u8> {
    match bet_type {
        BET_NEXT_2 => Some(2),
        BET_NEXT_3 => Some(3),
        BET_NEXT_4 => Some(4),
        BET_NEXT_5 => Some(5),
        BET_NEXT_6 => Some(6),
        BET_NEXT_7 => Some(7),
        BET_NEXT_8 => Some(8),
        BET_NEXT_9 => Some(9),
        BET_NEXT_10 => Some(10),
        BET_NEXT_11 => Some(11),
        BET_NEXT_12 => Some(12),
        _ => None,
    }
}

/// Get target number for REPEATER bets
pub fn get_repeater_bet_target(bet_type: u8) -> Option<u8> {
    match bet_type {
        BET_REPEATER_2 => Some(2),
        BET_REPEATER_3 => Some(3),
        BET_REPEATER_4 => Some(4),
        BET_REPEATER_5 => Some(5),
        BET_REPEATER_6 => Some(6),
        BET_REPEATER_8 => Some(8),
        BET_REPEATER_9 => Some(9),
        BET_REPEATER_10 => Some(10),
        BET_REPEATER_11 => Some(11),
        BET_REPEATER_12 => Some(12),
        _ => None,
    }
}

/// Check if bet is a YES bet
pub fn is_yes_bet(bet_type: u8) -> bool {
    bet_type >= BET_YES_2 && bet_type <= BET_YES_12
}

/// Check if bet is a NO bet
pub fn is_no_bet(bet_type: u8) -> bool {
    bet_type >= BET_NO_2 && bet_type <= BET_NO_12
}

/// Check if bet is a NEXT bet
pub fn is_next_bet(bet_type: u8) -> bool {
    bet_type >= BET_NEXT_2 && bet_type <= BET_NEXT_12
}

/// Check if bet is a REPEATER bet
pub fn is_repeater_bet(bet_type: u8) -> bool {
    bet_type >= BET_REPEATER_2 && bet_type <= BET_REPEATER_12
}

/// Check if bet is an odds bet
pub fn is_odds_bet(bet_type: u8) -> bool {
    matches!(bet_type, BET_ODDS_PASS | BET_ODDS_DONT_PASS | BET_ODDS_COME | BET_ODDS_DONT_COME)
}

/// Check if bet is a bonus bet
pub fn is_bonus_bet(bet_type: u8) -> bool {
    matches!(bet_type, BET_BONUS_SMALL | BET_BONUS_TALL | BET_BONUS_SMALL_TALL)
}

/// Check if bet is a multi-roll bet
pub fn is_multi_roll_bet(bet_type: u8) -> bool {
    match bet_type {
        // Single roll bets
        BET_FIELD | BET_NEXT_2..=BET_NEXT_12 => false,
        // All others are multi-roll
        _ => true,
    }
}