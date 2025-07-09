//! RNG utilities for secure dice generation in craps-pinocchio

use pinocchio::{
    program_error::ProgramError,
};
use pinocchio_log::log;

use crate::{
    constants::*,
    error::CrapsError,
    state::{RngState, RngPhase},
};

/// Generate fair dice using rejection sampling from entropy
/// 
/// Uses rejection sampling to ensure uniform distribution across [1, 6] for each die
pub fn generate_fair_dice(entropy: &[u8; 32]) -> Result<(u8, u8), ProgramError> {
    // We need at least 2 bytes for two dice
    if entropy.is_empty() {
        return Err(CrapsError::InvalidBlockhashData.into());
    }
    
    // Use rejection sampling for each die to ensure uniform distribution
    let die1 = rejection_sample_die(&entropy[0..16])?;
    let die2 = rejection_sample_die(&entropy[16..32])?;
    
    // Validate the dice values
    if die1 < DICE_MIN_VALUE || die1 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceValues.into());
    }
    if die2 < DICE_MIN_VALUE || die2 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceValues.into());
    }
    
    Ok((die1, die2))
}

/// Rejection sampling to get a fair die value [1, 6]
fn rejection_sample_die(entropy_slice: &[u8]) -> Result<u8, ProgramError> {
    // We'll use multiple bytes if needed to find a valid sample
    for &byte in entropy_slice {
        // Map byte value to range [0, 255]
        // We want values [0, 251] which maps nicely to [0, 5] when modulo 6
        // This gives us 252 valid values (42 * 6), rejecting only 4 values [252, 255]
        if byte <= 251 {
            return Ok((byte % 6) + 1);
        }
        // Otherwise reject and try next byte
    }
    
    // If we exhausted all bytes without finding a valid sample, use fallback
    // This is extremely unlikely but ensures we always return a value
    Ok((entropy_slice[0] % 6) + 1)
}

/// Mix entropy from multiple block hashes to create final randomness
/// 
/// Combines multiple block hashes using XOR to produce a single 32-byte entropy value
pub fn mix_entropy(block_hashes: &[[u8; 32]], hash_count: u8) -> Result<[u8; 32], ProgramError> {
    if hash_count == 0 {
        return Err(CrapsError::NoHashesCollected.into());
    }
    
    if hash_count < REQUIRED_BLOCK_HASHES {
        return Err(CrapsError::InsufficientBlockHashes.into());
    }
    
    // Start with zero entropy
    let mut mixed_entropy = [0u8; 32];
    
    // XOR all collected hashes together
    for i in 0..hash_count as usize {
        for j in 0..32 {
            mixed_entropy[j] ^= block_hashes[i][j];
        }
    }
    
    // Additional mixing using a simple hash-like transformation
    // This helps ensure entropy is well-distributed even if block hashes have patterns
    for i in 0..32 {
        let rotated_index = (i + 13) % 32;
        let xor_index = (i + 17) % 32;
        mixed_entropy[i] = mixed_entropy[i]
            .wrapping_add(mixed_entropy[rotated_index])
            .wrapping_mul(0x9E) // Prime multiplier
            ^ mixed_entropy[xor_index];
    }
    
    Ok(mixed_entropy)
}

/// Validate that RNG is in the correct phase for the requested operation
pub fn validate_rng_phase(
    rng_state: &RngState,
    expected_phase: RngPhase,
    current_epoch: u64,
) -> Result<(), ProgramError> {
    // Check epoch matches
    if rng_state.get_epoch() != current_epoch {
        log!("RNG epoch mismatch: expected {}, got {}", current_epoch, rng_state.get_epoch());
        return Err(CrapsError::InvalidEpoch.into());
    }
    
    // Check phase matches
    let current_phase = rng_state.get_phase();
    if current_phase != expected_phase {
        log!("RNG phase mismatch: expected {}, got {}", expected_phase as u8, current_phase as u8);
        return Err(CrapsError::InvalidRngPhase.into());
    }
    
    Ok(())
}

/// Generate dice from RNG state's final value
pub fn generate_dice_from_final_value(final_value: u64) -> (u8, u8) {
    // Extract two dice values from different parts of the random number
    // Use different bit ranges to ensure independence
    let die1 = ((final_value & 0xFF) % 6) + 1;
    let die2 = (((final_value >> 8) & 0xFF) % 6) + 1;
    
    (die1 as u8, die2 as u8)
}

/// Validate dice roll values are within valid range
pub fn validate_dice_values(die1: u8, die2: u8) -> Result<(), ProgramError> {
    if die1 < DICE_MIN_VALUE || die1 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceValues.into());
    }
    if die2 < DICE_MIN_VALUE || die2 > DICE_SIDES {
        return Err(CrapsError::InvalidDiceValues.into());
    }
    Ok(())
}

/// Check if RNG collection window is open
pub fn is_collection_window_open(
    rng_state: &RngState,
    current_slot: u64,
) -> bool {
    rng_state.get_phase() == RngPhase::Collection &&
    current_slot >= rng_state.get_collection_start_slot()
}

/// Check if enough time has passed for betting window
pub fn is_betting_window_complete(
    betting_start_slot: u64,
    collection_start_slot: u64,
    current_slot: u64,
) -> bool {
    current_slot >= collection_start_slot && 
    current_slot >= betting_start_slot + BETTING_WINDOW_SLOTS
}

/// Calculate entropy mixing seed from epoch and slot
pub fn calculate_entropy_seed(epoch: u64, slot: u64) -> u64 {
    // Use a large prime multiplier for good distribution
    const PRIME_MULTIPLIER: u64 = 0x9E3779B97F4A7C15;
    
    epoch.wrapping_mul(PRIME_MULTIPLIER) ^ slot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rejection_sampling() {
        // Test various entropy values
        let test_cases = vec![
            [5u8; 16],   // Should give 6
            [0u8; 16],   // Should give 1
            [251u8; 16], // Should give 6 (251 % 6 + 1)
            [252u8; 16], // Should reject and use fallback
        ];
        
        for entropy in test_cases {
            let result = rejection_sample_die(&entropy);
            assert!(result.is_ok());
            let die = result.unwrap();
            assert!(die >= 1 && die <= 6);
        }
    }

    #[test]
    fn test_mix_entropy() {
        let mut hashes = [[0u8; 32]; 5];
        // Create some test hashes
        for i in 0..5 {
            hashes[i][0] = i as u8;
            hashes[i][1] = (i * 2) as u8;
        }
        
        let result = mix_entropy(&hashes, 5);
        assert!(result.is_ok());
        
        // Verify the mixed entropy is different from any single hash
        let mixed = result.unwrap();
        for hash in &hashes {
            assert_ne!(&mixed, hash);
        }
    }

    #[test]
    fn test_dice_generation() {
        let entropy = [0x42u8; 32];
        let result = generate_fair_dice(&entropy);
        assert!(result.is_ok());
        
        let (die1, die2) = result.unwrap();
        assert!(die1 >= 1 && die1 <= 6);
        assert!(die2 >= 1 && die2 <= 6);
    }
}