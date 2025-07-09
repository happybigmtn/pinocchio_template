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
    state::{Treasury, ScalablePlayerState, BetBatch},
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
        _bonus_state,
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

    // Calculate total payout for all bets in the batch
    let mut total_payout = 0u64;
    let dice_total = outcome.get_dice_total();
    let phase = outcome.phase;
    let point = outcome.point;
    
    // Process each bet in the batch
    for i in 0..batch.bet_count as usize {
        let bet_metadata = decode_bet_from_batch(batch, i)?;
        let payout = calculate_bet_payout(&bet_metadata, dice_total, phase, point)?;
        
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
fn calculate_bet_payout(
    bet: &BetMetadata,
    dice_total: u8,
    phase: u8,
    point: u8,
) -> Result<u64, ProgramError> {
    // This is a simplified version - in production, you would use the full
    // bet resolution logic from craps_core_lib or implement all 64 bet types
    
    let amount = bet.amount;
    
    match bet.bet_type {
        BET_PASS => {
            if phase == PHASE_COME_OUT {
                if dice_total == 7 || dice_total == 11 {
                    // Win on natural
                    Ok(amount * 2) // 1:1 payout
                } else if dice_total == 2 || dice_total == 3 || dice_total == 12 {
                    // Lose on craps
                    Ok(0)
                } else {
                    // Point established, bet continues
                    Ok(0)
                }
            } else {
                // Point phase
                if dice_total == point {
                    // Win on point
                    Ok(amount * 2) // 1:1 payout
                } else if dice_total == 7 {
                    // Seven out
                    Ok(0)
                } else {
                    // Bet continues
                    Ok(0)
                }
            }
        },
        BET_DONT_PASS => {
            if phase == PHASE_COME_OUT {
                if dice_total == 7 || dice_total == 11 {
                    // Lose on natural
                    Ok(0)
                } else if dice_total == 2 || dice_total == 3 {
                    // Win on craps
                    Ok(amount * 2) // 1:1 payout
                } else if dice_total == 12 {
                    // Push on 12
                    Ok(amount) // Return bet
                } else {
                    // Point established, bet continues
                    Ok(0)
                }
            } else {
                // Point phase
                if dice_total == 7 {
                    // Win on seven out
                    Ok(amount * 2) // 1:1 payout
                } else if dice_total == point {
                    // Lose on point
                    Ok(0)
                } else {
                    // Bet continues
                    Ok(0)
                }
            }
        },
        BET_FIELD => {
            // Field bet is a one-roll bet
            match dice_total {
                2 => Ok(amount * 3), // 2:1 payout
                3 | 4 | 9 | 10 | 11 => Ok(amount * 2), // 1:1 payout
                12 => Ok(amount * 3), // 2:1 payout
                _ => Ok(0), // Lose on 5, 6, 7, 8
            }
        },
        // Add more bet types as needed
        // For now, return 0 for unimplemented bet types
        _ => Ok(0),
    }
}