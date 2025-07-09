//! Bet encoding utilities for packing and unpacking bet data
//! 
//! This module provides functions to encode and decode bet information
//! into compact formats for efficient on-chain storage.

use crate::error::CrapsError;
use crate::state::BetBatch;
use pinocchio::program_error::ProgramError;

/// Metadata for a decoded bet
pub struct BetMetadata {
    pub bet_type: u8,
    pub amount: u64,
    pub base_bet_index: Option<u8>,
}

// Bit masks and shifts for bet packing
pub const BET_KIND_BITS: u8 = 6;
pub const AMOUNT_INDEX_BITS: u8 = 10;
pub const BET_KIND_MASK_U16: u16 = (1 << BET_KIND_BITS) - 1; // 0x3F
pub const MAX_BET_KIND: u8 = 63;

/// Encode a bet type and amount into a packed u16
/// 
/// # Arguments
/// * `bet_type` - The bet type (0-63)
/// * `amount` - The amount in lamports
/// 
/// # Returns
/// * `Ok(u16)` - The packed bet data
/// * `Err(ProgramError)` - If encoding fails
pub fn encode_bet(bet_type: u8, amount: u64) -> Result<u16, ProgramError> {
    // Validate bet type
    if bet_type > MAX_BET_KIND {
        return Err(CrapsError::InvalidBetKind.into());
    }
    
    // Encode amount to index
    let amount_index = encode_amount(amount)?;
    
    // Pack bet: amount_index in upper 10 bits, bet_type in lower 6 bits
    Ok(((amount_index & ((1 << AMOUNT_INDEX_BITS) - 1)) << BET_KIND_BITS) 
        | (bet_type as u16 & BET_KIND_MASK_U16))
}

/// Decode a packed u16 bet into bet type and amount
/// 
/// # Arguments
/// * `packed` - The packed bet data
/// 
/// # Returns
/// * `(bet_type, amount)` - The decoded bet type and amount in lamports
pub fn decode_bet(packed: u16) -> (u8, u64) {
    let bet_type = (packed & BET_KIND_MASK_U16) as u8;
    let amount_index = (packed >> BET_KIND_BITS) & ((1 << AMOUNT_INDEX_BITS) - 1);
    let amount = decode_amount(amount_index);
    (bet_type, amount)
}

/// Encode an amount to a 10-bit index using non-linear encoding
/// 
/// Encoding scheme:
/// - 1-100 CRAP: Direct mapping (indices 0-99)
/// - 101-500 CRAP by 5s: (indices 100-179)
/// - 501-1500 CRAP by 10s: (indices 180-279)
/// - 1501-5000 CRAP by 25s: (indices 280-419)
/// - 5001-10000 CRAP by 50s: (indices 420-519)
/// - 10001-20000 CRAP by 100s: (indices 520-619)
/// - 20001-40000 CRAP by 250s: (indices 620-699)
/// - 40001-60000 CRAP by 500s: (indices 700-739)
/// - 60001-80000 CRAP by 1000s: (indices 740-759)
/// - 80001-100000 CRAP by 2500s: (indices 760-767)
pub fn encode_amount(amount: u64) -> Result<u16, ProgramError> {
    const CRAP: u64 = 1_000_000_000; // 9 decimals
    
    // Validate amount
    if amount == 0 || amount > 100_000 * CRAP {
        return Err(CrapsError::InvalidBetAmount.into());
    }
    
    // Must be whole CRAP amounts
    if amount % CRAP != 0 {
        return Err(CrapsError::InvalidBetAmount.into());
    }
    
    let crap_amount = amount / CRAP;
    
    let index = match crap_amount {
        1..=100 => {
            crap_amount.checked_sub(1).ok_or(CrapsError::NumericalUnderflow)?
        }
        101..=500 => {
            let offset = crap_amount.checked_sub(100).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 5 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            99u64.checked_add(offset / 5).ok_or(CrapsError::NumericalOverflow)?
        }
        501..=1500 => {
            let offset = crap_amount.checked_sub(500).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 10 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            179u64.checked_add(offset / 10).ok_or(CrapsError::NumericalOverflow)?
        }
        1501..=5000 => {
            let offset = crap_amount.checked_sub(1500).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 25 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            279u64.checked_add(offset / 25).ok_or(CrapsError::NumericalOverflow)?
        }
        5001..=10000 => {
            let offset = crap_amount.checked_sub(5000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 50 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            419u64.checked_add(offset / 50).ok_or(CrapsError::NumericalOverflow)?
        }
        10001..=20000 => {
            let offset = crap_amount.checked_sub(10000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 100 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            519u64.checked_add(offset / 100).ok_or(CrapsError::NumericalOverflow)?
        }
        20001..=40000 => {
            let offset = crap_amount.checked_sub(20000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 250 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            619u64.checked_add(offset / 250).ok_or(CrapsError::NumericalOverflow)?
        }
        40001..=60000 => {
            let offset = crap_amount.checked_sub(40000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 500 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            699u64.checked_add(offset / 500).ok_or(CrapsError::NumericalOverflow)?
        }
        60001..=80000 => {
            let offset = crap_amount.checked_sub(60000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 1000 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            739u64.checked_add(offset / 1000).ok_or(CrapsError::NumericalOverflow)?
        }
        80001..=100000 => {
            let offset = crap_amount.checked_sub(80000).ok_or(CrapsError::NumericalUnderflow)?;
            if offset % 2500 != 0 {
                return Err(CrapsError::InvalidBetAmount.into());
            }
            759u64.checked_add(offset / 2500).ok_or(CrapsError::NumericalOverflow)?
        }
        _ => return Err(CrapsError::InvalidBetAmount.into()),
    };
    
    // Ensure index fits in 10 bits
    if index > 1023 {
        return Err(CrapsError::InvalidBetAmount.into());
    }
    
    Ok(index as u16)
}

/// Decode a 10-bit amount index to lamports
/// 
/// Returns 0 for invalid indices
pub fn decode_amount(index: u16) -> u64 {
    const CRAP: u64 = 1_000_000_000; // 9 decimals
    
    match index {
        // 1-100 CRAP: Direct mapping
        0..=99 => {
            (index as u64 + 1) * CRAP
        }
        
        // 101-500 CRAP by 5s
        100..=179 => {
            ((index as u64 - 99) * 5 + 100) * CRAP
        }
        
        // 501-1500 CRAP by 10s
        180..=279 => {
            ((index as u64 - 179) * 10 + 500) * CRAP
        }
        
        // 1501-5000 CRAP by 25s
        280..=419 => {
            ((index as u64 - 279) * 25 + 1500) * CRAP
        }
        
        // 5001-10000 CRAP by 50s
        420..=519 => {
            ((index as u64 - 419) * 50 + 5000) * CRAP
        }
        
        // 10001-20000 CRAP by 100s
        520..=619 => {
            ((index as u64 - 519) * 100 + 10000) * CRAP
        }
        
        // 20001-40000 CRAP by 250s
        620..=699 => {
            ((index as u64 - 619) * 250 + 20000) * CRAP
        }
        
        // 40001-60000 CRAP by 500s
        700..=739 => {
            ((index as u64 - 699) * 500 + 40000) * CRAP
        }
        
        // 60001-80000 CRAP by 1000s
        740..=759 => {
            ((index as u64 - 739) * 1000 + 60000) * CRAP
        }
        
        // 80001-100000 CRAP by 2500s
        760..=767 => {
            ((index as u64 - 759) * 2500 + 80000) * CRAP
        }
        
        // Invalid indices
        _ => 0,
    }
}

/// Decode amount with validation
pub fn decode_amount_safe(index: u16) -> Result<u64, ProgramError> {
    let amount = decode_amount(index);
    if amount == 0 && index != 0 {
        Err(CrapsError::InvalidBetAmount.into())
    } else {
        Ok(amount)
    }
}

/// Get the bet type from packed bet data
pub fn get_bet_type(packed: u16) -> u8 {
    (packed & BET_KIND_MASK_U16) as u8
}

/// Get the amount index from packed bet data
pub fn get_amount_index(packed: u16) -> u16 {
    (packed >> BET_KIND_BITS) & ((1 << AMOUNT_INDEX_BITS) - 1)
}

/// Decode a bet from a batch at the specified index
pub fn decode_bet_from_batch(batch: &BetBatch, index: usize) -> Result<BetMetadata, ProgramError> {
    if index >= batch.bet_count as usize {
        return Err(CrapsError::InvalidBetData.into());
    }
    
    let packed = batch.get_packed_bet(index);
    let (bet_type, amount) = decode_bet(packed);
    
    // Check if this is an odds bet that needs a base bet
    let base_bet_index = match bet_type {
        29..=32 => { // BET_ODDS_PASS through BET_ODDS_DONT_COME
            // For odds bets, find the corresponding base bet
            // This is a simplified version - in production you'd have more sophisticated tracking
            Some(0) // Placeholder - should find actual base bet
        }
        _ => None,
    };
    
    Ok(BetMetadata {
        bet_type,
        amount,
        base_bet_index,
    })
}