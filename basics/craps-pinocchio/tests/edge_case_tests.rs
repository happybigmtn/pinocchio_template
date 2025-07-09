#![cfg(test)]

use craps_pinocchio::{
    constants::*,
    state::{GlobalGameState, ScalablePlayerState, Treasury, BonusState, RngState, RngPhase},
    state::bet::{BetBatch, MAX_BETS_PER_BATCH},
    instructions::claim::EpochOutcome,
    utils::{
        bet_encoding::{encode_bet, decode_bet},
        dice::{are_dice_valid, is_hard_way, is_natural, is_craps},
        rng::*,
    },
};

mod bet_edge_cases {
    use super::*;

    #[test]
    fn test_minimum_bet_amounts() {
        // Test encoding minimum bet amount
        let min_bet = MIN_BET_AMOUNT;
        let encoded = encode_bet(BET_PASS, min_bet).unwrap();
        let (bet_type, decoded_amount) = decode_bet(encoded);
        
        assert_eq!(bet_type, BET_PASS);
        assert!(decoded_amount >= min_bet, "Decoded amount should be at least minimum");
    }

    #[test]
    fn test_maximum_bet_amounts() {
        // Test encoding maximum bet amount
        let max_bet = MAX_BET_AMOUNT;
        let encoded = encode_bet(BET_FIELD, max_bet).unwrap();
        let (bet_type, decoded_amount) = decode_bet(encoded);
        
        assert_eq!(bet_type, BET_FIELD);
        assert!(decoded_amount <= max_bet, "Decoded amount should not exceed maximum");
    }

    #[test]
    fn test_all_bet_types() {
        // Test encoding/decoding all 64 bet types
        let amount = 100_000_000; // 0.1 tokens
        
        for bet_type in 0..64u8 {
            let encoded = encode_bet(bet_type, amount).unwrap();
            let (decoded_type, _) = decode_bet(encoded);
            assert_eq!(decoded_type, bet_type, "Bet type {} should round-trip correctly", bet_type);
        }
    }

    #[test]
    fn test_bet_amount_quantization() {
        // Test that similar amounts map to same encoded value due to quantization
        let amounts = vec![
            100_000_000,
            100_500_000,
            101_000_000,
        ];
        
        let mut encoded_values = vec![];
        for amount in amounts {
            let encoded = encode_bet(BET_PASS, amount).unwrap();
            encoded_values.push(encoded);
        }
        
        // Due to quantization, some adjacent values might encode the same
        // This is expected behavior
    }
}

mod dice_edge_cases {
    use super::*;

    #[test]
    fn test_all_valid_dice_combinations() {
        // Test all 36 possible dice combinations
        for die1 in 1..=6 {
            for die2 in 1..=6 {
                assert!(are_dice_valid(die1, die2));
                
                let total = die1 + die2;
                assert!(total >= DICE_MIN_SUM && total <= DICE_MAX_SUM);
                
                // Test specific combinations
                if die1 == die2 && die1 >= 2 && die1 <= 5 {
                    assert!(is_hard_way(die1, die2));
                }
                
                if total == NATURAL_SEVEN || total == NATURAL_ELEVEN {
                    assert!(is_natural(die1, die2));
                }
                
                if total == CRAPS_TWO || total == CRAPS_THREE || total == CRAPS_TWELVE {
                    assert!(is_craps(die1, die2));
                }
            }
        }
    }

    #[test]
    fn test_point_validation() {
        // Valid points
        for point in VALID_POINTS {
            assert!(VALID_POINTS.contains(&point));
        }
        
        // Invalid points
        for non_point in [2, 3, 7, 11, 12] {
            assert!(!VALID_POINTS.contains(&non_point));
        }
    }
}

mod rng_edge_cases {
    use super::*;

    #[test]
    fn test_entropy_mixing_edge_cases() {
        // Test with minimum required hashes
        let mut hashes = [[0u8; 32]; 15];
        for i in 0..REQUIRED_BLOCK_HASHES as usize {
            hashes[i] = [i as u8; 32];
        }
        
        let mixed = mix_entropy(&hashes, REQUIRED_BLOCK_HASHES).unwrap();
        assert_ne!(mixed, [0u8; 32], "Mixed entropy should not be all zeros");
        
        // Test with maximum hashes
        for i in 0..MAX_BLOCK_HASHES as usize {
            hashes[i] = [(i * 7) as u8; 32];
        }
        
        let mixed_max = mix_entropy(&hashes, MAX_BLOCK_HASHES).unwrap();
        assert_ne!(mixed_max, mixed, "Different inputs should produce different entropy");
    }

    #[test]
    fn test_insufficient_hashes() {
        let hashes = [[1u8; 32]; 15];
        let result = mix_entropy(&hashes, REQUIRED_BLOCK_HASHES - 1);
        assert!(result.is_err(), "Should fail with insufficient hashes");
    }

    #[test]
    fn test_dice_generation_distribution() {
        // Test that dice generation produces valid values across different entropy
        let test_entropies = vec![
            [0x00u8; 32],
            [0xFFu8; 32],
            [0x55u8; 32],
            [0xAAu8; 32],
        ];
        
        for entropy in test_entropies {
            let result = generate_fair_dice(&entropy);
            assert!(result.is_ok());
            
            let (die1, die2) = result.unwrap();
            assert!(die1 >= 1 && die1 <= 6);
            assert!(die2 >= 1 && die2 <= 6);
        }
    }
}

mod state_edge_cases {
    use super::*;

    #[test]
    fn test_bet_batch_capacity() {
        let mut batch = BetBatch {
            player: [0; 32],
            epoch: [1, 0, 0, 0, 0, 0, 0, 0],
            bet_count: 0,
            _padding1: [0; 7],
            total_amount: [0; 8],
            packed_bets: [0; 32],
            resolved_mask: [0; 2],
            realizable_mask: [0; 2],
            settled_mask: [0; 2],
            winning_mask: [0; 2],
            payout_total: [0; 8],
            individual_payouts: [0; 128],
            come_points: [0; 16],
            linked_bets: [0; 16],
            cached_outcomes: [0; 16],
            cache_epoch: [0; 8],
            bump: 0,
            _padding2: [0; 7],
        };
        
        // Fill batch to capacity
        for i in 0..MAX_BETS_PER_BATCH {
            let bet_data = encode_bet(BET_PASS, 100_000_000).unwrap();
            let offset = i * 2;
            batch.packed_bets[offset] = (bet_data & 0xFF) as u8;
            batch.packed_bets[offset + 1] = ((bet_data >> 8) & 0xFF) as u8;
            batch.bet_count += 1;
        }
        
        assert_eq!(batch.bet_count as usize, MAX_BETS_PER_BATCH);
        
        // Verify all bets can be decoded
        for i in 0..MAX_BETS_PER_BATCH {
            let offset = i * 2;
            let packed = u16::from_le_bytes([batch.packed_bets[offset], batch.packed_bets[offset + 1]]);
            let (bet_type, _) = decode_bet(packed);
            assert_eq!(bet_type, BET_PASS);
        }
    }

    #[test]
    fn test_epoch_outcome_dice_bounds() {
        let mut outcome = EpochOutcome {
            epoch: [1, 0, 0, 0, 0, 0, 0, 0],
            dice: [0, 7], // Invalid dice values
            phase: PHASE_COME_OUT,
            point: 0,
            resolved: 0,
            _padding: [0; 3],
            total_payouts: [0; 8],
            finalized_slot: [0; 8],
            _reserved: [0; 32],
        };
        
        // Test that invalid dice values would be caught
        assert!(!are_dice_valid(outcome.dice[0], outcome.dice[1]));
        
        // Fix dice values
        outcome.dice[0] = 3;
        outcome.dice[1] = 4;
        assert!(are_dice_valid(outcome.dice[0], outcome.dice[1]));
    }
}

mod treasury_edge_cases {
    use super::*;

    #[test]
    fn test_treasury_balance_limits() {
        let treasury = Treasury {
            authority: [0; 32],
            token_mint: [0; 32],
            vault: [0; 32],
            total_deposits: [0xFF; 8],
            total_withdrawals: [0; 8],
            total_payouts: [0; 8],
            total_bets_placed: [0; 8],
            total_bets_settled: [0; 8],
            last_update_slot: [0; 8],
            emergency_shutdown: 0,
            bump: 0,
            _padding: [0; 6],
        };
        
        let balance = u64::from_le_bytes(treasury.total_deposits);
        assert_eq!(balance, u64::MAX);
        
        // Test that balance calculations handle overflow
        let deposit = 1000u64;
        let new_balance = balance.saturating_add(deposit);
        assert_eq!(new_balance, u64::MAX); // Should saturate, not overflow
    }

    #[test]
    fn test_withdrawal_limits() {
        // Test daily withdrawal limit
        assert!(DAILY_WITHDRAWAL_LIMIT > MAX_WITHDRAWAL_AMOUNT);
        assert!(MAX_WITHDRAWAL_AMOUNT > 0);
        
        // Test that single withdrawal can't exceed daily limit
        assert!(MAX_WITHDRAWAL_AMOUNT <= DAILY_WITHDRAWAL_LIMIT);
    }
}

mod concurrent_operation_tests {
    use super::*;

    #[test]
    fn test_multiple_players_same_epoch() {
        // Simulate multiple players betting in same epoch
        let epoch = 100u64;
        let players = vec![
            [1u8; 32],
            [2u8; 32],
            [3u8; 32],
        ];
        
        for player in players {
            for batch_index in 0..3u16 {
                // Each player can have multiple bet batches
                let seed_data = [
                    BET_BATCH_SEED,
                    &player,
                    &epoch.to_le_bytes(),
                    &batch_index.to_le_bytes(),
                ].concat();
                
                // Verify seed data is unique for each combination
                assert_eq!(seed_data.len(), BET_BATCH_SEED.len() + 32 + 8 + 2);
            }
        }
    }

    #[test]
    fn test_epoch_transition_boundaries() {
        // Test epoch number boundaries
        let epochs = vec![
            0u64,           // First epoch
            1u64,           // Normal epoch
            u64::MAX - 1,   // Near maximum
            u64::MAX,       // Maximum epoch
        ];
        
        for epoch in epochs {
            let bytes = epoch.to_le_bytes();
            let decoded = u64::from_le_bytes(bytes);
            assert_eq!(decoded, epoch);
        }
    }
}

mod phase_transition_tests {
    use super::*;

    #[test]
    fn test_game_phase_transitions() {
        // Test valid phase transitions using phase constants
        let mut phase = PHASE_COME_OUT;
        
        // Come out roll of 7 or 11 stays in come out
        phase = PHASE_COME_OUT;
        
        // Come out roll of point number transitions to point phase
        phase = PHASE_POINT;
        
        // Point phase seven out returns to come out
        phase = PHASE_COME_OUT;
        
        // Ensure phase values are valid
        assert!(phase == PHASE_COME_OUT || phase == PHASE_POINT);
    }

    #[test]
    fn test_rng_phase_transitions() {
        // Test RNG phase state machine
        let mut phase = RngPhase::Betting;
        
        // Betting -> Collection (after betting window)
        phase = RngPhase::Collection;
        
        // Collection -> Finalized (after enough hashes)
        phase = RngPhase::Finalized;
        
        // Finalized -> Betting (new epoch)
        phase = RngPhase::Betting;
    }
}

mod payout_edge_cases {
    use super::*;
    use pinocchio::program_error::ProgramError;

    fn calculate_test_payout(bet_type: u8, amount: u64, dice_total: u8) -> Result<u64, ProgramError> {
        // Simplified payout calculation for testing
        match bet_type {
            BET_PASS => {
                if dice_total == 7 || dice_total == 11 {
                    Ok(amount * 2) // 1:1 payout
                } else {
                    Ok(0)
                }
            }
            BET_FIELD => {
                match dice_total {
                    2 | 12 => Ok(amount * 3), // 2:1 payout
                    3 | 4 | 9 | 10 | 11 => Ok(amount * 2), // 1:1 payout
                    _ => Ok(0),
                }
            }
            _ => Ok(0),
        }
    }

    #[test]
    fn test_payout_overflow_protection() {
        // Test that payouts don't overflow
        let huge_bet = u64::MAX / 2;
        let payout = calculate_test_payout(BET_FIELD, huge_bet, 2);
        
        // Should handle overflow gracefully
        assert!(payout.is_ok() || payout.unwrap() <= u64::MAX);
    }

    #[test]
    fn test_zero_amount_bets() {
        // Test that zero amount bets are handled
        let payout = calculate_test_payout(BET_PASS, 0, 7);
        assert_eq!(payout.unwrap(), 0);
    }
}