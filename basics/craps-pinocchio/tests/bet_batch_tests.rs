use solana_sdk::pubkey::Pubkey;

use craps_pinocchio::{
    constants::*,
    state::{BetBatch, MAX_BETS_PER_BATCH},
    utils::bet_encoding::*,
    ID,
};

/// Test suite for bet batch handling and management
#[cfg(test)]
mod bet_batch_tests {
    use super::*;


    fn create_test_bet_batch(player: &Pubkey, epoch: u64) -> BetBatch {
        let mut bet_batch: BetBatch = unsafe { std::mem::zeroed() };
        bet_batch.set_epoch(epoch);
        bet_batch.player = player.to_bytes();
        bet_batch.bet_count = 0;
        bet_batch.set_total_amount(0);
        bet_batch.bump = 255; // Placeholder bump
        bet_batch
    }

    #[test]
    fn test_bet_batch_structure() {
        use std::mem::size_of;
        
        // Test basic bet batch structure
        assert_eq!(size_of::<BetBatch>(), BetBatch::LEN);
        
        let player = Pubkey::new_unique();
        let epoch = 42;
        let mut bet_batch = create_test_bet_batch(&player, epoch);

        // Test basic setters/getters
        assert_eq!(bet_batch.get_epoch(), epoch);
        assert_eq!(bet_batch.player, player.to_bytes());
        assert_eq!(bet_batch.bet_count, 0);
        assert_eq!(bet_batch.get_total_amount(), 0);

        // Test packed bet operations
        bet_batch.set_packed_bet(0, 0x1234);
        assert_eq!(bet_batch.get_packed_bet(0), 0x1234);

        // Test mask operations
        bet_batch.set_resolved_mask(0b0101);
        assert_eq!(bet_batch.get_resolved_mask(), 0b0101);
        assert!(bet_batch.is_bet_resolved(0));
        assert!(!bet_batch.is_bet_resolved(1));
        assert!(bet_batch.is_bet_resolved(2));
        assert!(!bet_batch.is_bet_resolved(3));

        bet_batch.set_winning_mask(0b0001);
        assert_eq!(bet_batch.get_winning_mask(), 0b0001);
        assert!(bet_batch.is_bet_winner(0));
        assert!(!bet_batch.is_bet_winner(1));

        // Test individual payouts
        bet_batch.set_individual_payout(0, 2_000_000_000);
        assert_eq!(bet_batch.get_individual_payout(0), 2_000_000_000);
        assert_eq!(bet_batch.get_individual_payout(1), 0);
    }

    #[test]
    fn test_bet_batch_max_capacity() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Fill the batch to maximum capacity
        for i in 0..MAX_BETS_PER_BATCH {
            // Use simple test values instead of encode_bet which might fail
            let test_value = (i as u16 + 1) * 100; // Simple test pattern
            bet_batch.set_packed_bet(i, test_value);
            bet_batch.bet_count += 1;
        }

        assert_eq!(bet_batch.bet_count as usize, MAX_BETS_PER_BATCH);

        // Test that we can access all bets
        for i in 0..MAX_BETS_PER_BATCH {
            let packed_bet = bet_batch.get_packed_bet(i);
            let expected_value = (i as u16 + 1) * 100;
            assert_eq!(packed_bet, expected_value);
        }

        // Test boundary conditions
        assert_eq!(bet_batch.get_packed_bet(MAX_BETS_PER_BATCH), 0); // Out of bounds should return 0
        assert_eq!(bet_batch.get_individual_payout(MAX_BETS_PER_BATCH), 0); // Out of bounds should return 0
    }

    #[test]
    fn test_bet_batch_masks_operations() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add some bets
        bet_batch.bet_count = 5;
        for i in 0..5 {
            bet_batch.set_packed_bet(i, encode_bet(BET_PASS, (i as u64 + 1) * 1_000_000_000).unwrap());
        }

        // Test resolved mask
        bet_batch.set_resolved_mask(0b11010); // Bets 1, 3, 4 resolved
        assert!(bet_batch.is_bet_resolved(1));
        assert!(bet_batch.is_bet_resolved(3));
        assert!(bet_batch.is_bet_resolved(4));
        assert!(!bet_batch.is_bet_resolved(0));
        assert!(!bet_batch.is_bet_resolved(2));

        // Test realizable mask (subset of resolved)
        bet_batch.set_realizable_mask(0b10010); // Bets 1, 4 realizable
        assert!(bet_batch.is_bet_realizable(1));
        assert!(bet_batch.is_bet_realizable(4));
        assert!(!bet_batch.is_bet_realizable(0));
        assert!(!bet_batch.is_bet_realizable(2));
        assert!(!bet_batch.is_bet_realizable(3));

        // Test winning mask (subset of realizable)
        bet_batch.set_winning_mask(0b00010); // Only bet 1 wins
        assert!(bet_batch.is_bet_winner(1));
        assert!(!bet_batch.is_bet_winner(4));

        // Test settled mask
        bet_batch.set_settled_mask(0b00010); // Only bet 1 settled
        assert!(bet_batch.is_bet_settled(1));
        assert!(!bet_batch.is_bet_settled(4));
    }

    #[test]
    fn test_bet_batch_payout_calculations() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add some winning bets
        bet_batch.bet_count = 3;
        bet_batch.set_packed_bet(0, encode_bet(BET_PASS, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(1, encode_bet(BET_FIELD, 2_000_000_000).unwrap());
        bet_batch.set_packed_bet(2, encode_bet(BET_REPEATER_2, 5_000_000_000).unwrap());

        // Set payouts
        bet_batch.set_individual_payout(0, 2_000_000_000); // Pass line 1:1
        bet_batch.set_individual_payout(1, 4_000_000_000); // Field 2:1
        bet_batch.set_individual_payout(2, 200_000_000_000); // Repeater 2 at 40:1

        let total_expected = 2_000_000_000 + 4_000_000_000 + 200_000_000_000;
        bet_batch.set_payout_total(total_expected);

        assert_eq!(bet_batch.get_individual_payout(0), 2_000_000_000);
        assert_eq!(bet_batch.get_individual_payout(1), 4_000_000_000);
        assert_eq!(bet_batch.get_individual_payout(2), 200_000_000_000);
        assert_eq!(bet_batch.get_payout_total(), total_expected);
    }

    #[test]
    fn test_bet_batch_come_points() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add come bets
        bet_batch.bet_count = 4;
        bet_batch.set_packed_bet(0, encode_bet(BET_COME, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(1, encode_bet(BET_DONT_COME, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(2, encode_bet(BET_PASS, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(3, encode_bet(BET_COME, 2_000_000_000).unwrap());

        // Set come points
        bet_batch.come_points[0] = 6; // Come bet goes to 6
        bet_batch.come_points[1] = 8; // Don't come bet goes to 8
        bet_batch.come_points[2] = 0; // Pass line has no come point
        bet_batch.come_points[3] = 4; // Second come bet goes to 4

        assert_eq!(bet_batch.come_points[0], 6);
        assert_eq!(bet_batch.come_points[1], 8);
        assert_eq!(bet_batch.come_points[2], 0);
        assert_eq!(bet_batch.come_points[3], 4);
    }

    #[test]
    fn test_bet_batch_linked_bets() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add line bets and odds bets
        bet_batch.bet_count = 4;
        bet_batch.set_packed_bet(0, encode_bet(BET_PASS, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(1, encode_bet(BET_ODDS_PASS, 2_000_000_000).unwrap());
        bet_batch.set_packed_bet(2, encode_bet(BET_COME, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(3, encode_bet(BET_ODDS_COME, 3_000_000_000).unwrap());

        // Link odds bets to their base bets
        bet_batch.linked_bets[0] = 255; // Pass line not linked to anything
        bet_batch.linked_bets[1] = 0;   // Pass odds linked to pass line (index 0)
        bet_batch.linked_bets[2] = 255; // Come bet not linked to anything
        bet_batch.linked_bets[3] = 2;   // Come odds linked to come bet (index 2)

        assert_eq!(bet_batch.linked_bets[0], 255);
        assert_eq!(bet_batch.linked_bets[1], 0);
        assert_eq!(bet_batch.linked_bets[2], 255);
        assert_eq!(bet_batch.linked_bets[3], 2);
    }

    #[test]
    fn test_bet_batch_cache_operations() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add some bets
        bet_batch.bet_count = 3;
        bet_batch.set_cache_epoch(1);

        // Set cached outcomes
        bet_batch.cached_outcomes[0] = 1; // Loss
        bet_batch.cached_outcomes[1] = 2; // Win
        bet_batch.cached_outcomes[2] = 3; // Continue

        assert_eq!(bet_batch.cached_outcomes[0], 1);
        assert_eq!(bet_batch.cached_outcomes[1], 2);
        assert_eq!(bet_batch.cached_outcomes[2], 3);
        assert_eq!(bet_batch.get_cache_epoch(), 1);

        // Test cache invalidation (epoch change)
        bet_batch.set_cache_epoch(2);
        assert_eq!(bet_batch.get_cache_epoch(), 2);
    }

    #[test]
    fn test_bet_batch_serialization() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 42);

        // Set up a complex bet batch
        bet_batch.bet_count = 3;
        bet_batch.set_total_amount(8_000_000_000);
        bet_batch.set_packed_bet(0, encode_bet(BET_PASS, 1_000_000_000).unwrap());
        bet_batch.set_packed_bet(1, encode_bet(BET_FIELD, 2_000_000_000).unwrap());
        bet_batch.set_packed_bet(2, encode_bet(BET_REPEATER_6, 5_000_000_000).unwrap());
        bet_batch.set_resolved_mask(0b111);
        bet_batch.set_winning_mask(0b101);
        bet_batch.set_individual_payout(0, 2_000_000_000);
        bet_batch.set_individual_payout(2, 450_000_000_000); // 90:1 for repeater 6

        // Serialize and deserialize
        let serialized = bytemuck::bytes_of(&bet_batch);
        let deserialized = bytemuck::from_bytes::<BetBatch>(serialized);

        // Verify all data is preserved
        assert_eq!(deserialized.get_epoch(), 42);
        assert_eq!(deserialized.player, player.to_bytes());
        assert_eq!(deserialized.bet_count, 3);
        assert_eq!(deserialized.get_total_amount(), 8_000_000_000);
        assert_eq!(deserialized.get_resolved_mask(), 0b111);
        assert_eq!(deserialized.get_winning_mask(), 0b101);
        assert_eq!(deserialized.get_individual_payout(0), 2_000_000_000);
        assert_eq!(deserialized.get_individual_payout(2), 450_000_000_000);

        // Verify packed bets
        assert_eq!(deserialized.get_packed_bet(0), bet_batch.get_packed_bet(0));
        assert_eq!(deserialized.get_packed_bet(1), bet_batch.get_packed_bet(1));
        assert_eq!(deserialized.get_packed_bet(2), bet_batch.get_packed_bet(2));
    }

    #[test]
    fn test_bet_batch_repeater_integration() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Add various repeater bets
        let repeater_bets = [
            (BET_REPEATER_2, 10_000_000_000u64),
            (BET_REPEATER_6, 15_000_000_000u64),
            (BET_REPEATER_8, 15_000_000_000u64),
            (BET_REPEATER_12, 10_000_000_000u64),
        ];

        bet_batch.bet_count = repeater_bets.len() as u8;

        for (i, (bet_type, _amount)) in repeater_bets.iter().enumerate() {
            // Create a simple packed value for testing
            let packed_value = (*bet_type as u16) << 8 | (i as u16 + 1);
            bet_batch.set_packed_bet(i, packed_value);
        }

        // Verify all repeater bets are stored correctly
        for (i, (expected_bet_type, _expected_amount)) in repeater_bets.iter().enumerate() {
            let packed_bet = bet_batch.get_packed_bet(i);
            let expected_value = (*expected_bet_type as u16) << 8 | (i as u16 + 1);
            assert_eq!(packed_bet, expected_value);
        }

        // Test that repeater bets don't require special fields in BetBatch
        // (they used to have repeater_targets but that was removed)
        assert_eq!(bet_batch.bet_count as usize, repeater_bets.len());
    }

    #[test]
    fn test_bet_batch_boundary_conditions() {
        let player = Pubkey::new_unique();
        let mut bet_batch = create_test_bet_batch(&player, 1);

        // Test accessing bets beyond capacity
        assert_eq!(bet_batch.get_packed_bet(MAX_BETS_PER_BATCH), 0);
        assert_eq!(bet_batch.get_packed_bet(MAX_BETS_PER_BATCH + 1), 0);
        assert_eq!(bet_batch.get_individual_payout(MAX_BETS_PER_BATCH), 0);

        // Test mask operations with all bits set
        bet_batch.set_resolved_mask(0xFFFF);
        for i in 0..16 {
            assert!(bet_batch.is_bet_resolved(i));
        }
        assert!(!bet_batch.is_bet_resolved(16)); // Out of bounds

        // Test setting packed bet at boundary
        bet_batch.set_packed_bet(MAX_BETS_PER_BATCH - 1, 0x1234);
        assert_eq!(bet_batch.get_packed_bet(MAX_BETS_PER_BATCH - 1), 0x1234);

        // Test setting packed bet beyond boundary (should be ignored)
        bet_batch.set_packed_bet(MAX_BETS_PER_BATCH, 0x5678);
        assert_eq!(bet_batch.get_packed_bet(MAX_BETS_PER_BATCH), 0);
    }

    #[test]
    fn test_bet_batch_multiple_epochs() {
        let player = Pubkey::new_unique();

        // Create bet batches for different epochs
        let mut batch_epoch_1 = create_test_bet_batch(&player, 1);
        let mut batch_epoch_2 = create_test_bet_batch(&player, 2);
        let mut batch_epoch_3 = create_test_bet_batch(&player, 3);

        // Each should be independent
        batch_epoch_1.bet_count = 2;
        batch_epoch_1.set_packed_bet(0, encode_bet(BET_PASS, 1_000_000_000).unwrap());

        batch_epoch_2.bet_count = 1;
        batch_epoch_2.set_packed_bet(0, encode_bet(BET_FIELD, 2_000_000_000).unwrap());

        batch_epoch_3.bet_count = 3;
        batch_epoch_3.set_packed_bet(0, encode_bet(BET_REPEATER_6, 5_000_000_000).unwrap());

        // Verify independence
        assert_eq!(batch_epoch_1.get_epoch(), 1);
        assert_eq!(batch_epoch_2.get_epoch(), 2);
        assert_eq!(batch_epoch_3.get_epoch(), 3);

        assert_eq!(batch_epoch_1.bet_count, 2);
        assert_eq!(batch_epoch_2.bet_count, 1);
        assert_eq!(batch_epoch_3.bet_count, 3);

        // Verify bet contents are different
        let (bet_type_1, _) = decode_bet(batch_epoch_1.get_packed_bet(0));
        let (bet_type_2, _) = decode_bet(batch_epoch_2.get_packed_bet(0));
        let (bet_type_3, _) = decode_bet(batch_epoch_3.get_packed_bet(0));

        assert_eq!(bet_type_1, BET_PASS);
        assert_eq!(bet_type_2, BET_FIELD);
        assert_eq!(bet_type_3, BET_REPEATER_6);
    }

    #[test]
    fn test_bet_batch_pda_derivation() {
        let player1 = Pubkey::new_unique();
        let player2 = Pubkey::new_unique();
        let epoch1 = 1u64;
        let epoch2 = 2u64;
        let program_id = Pubkey::new_from_array(ID);

        fn derive_bet_batch_pda(player: &Pubkey, epoch: u64, program_id: &Pubkey) -> (Pubkey, u8) {
            let batch_index = 0u32;
            Pubkey::find_program_address(
                &[BET_BATCH_SEED, player.as_ref(), &epoch.to_le_bytes(), &batch_index.to_le_bytes()],
                program_id
            )
        }

        // Different players should have different PDAs
        let (pda1_e1, bump1_e1) = derive_bet_batch_pda(&player1, epoch1, &program_id);
        let (pda2_e1, _bump2_e1) = derive_bet_batch_pda(&player2, epoch1, &program_id);
        assert_ne!(pda1_e1, pda2_e1);

        // Same player, different epochs should have different PDAs
        let (pda1_e1_again, bump1_e1_again) = derive_bet_batch_pda(&player1, epoch1, &program_id);
        let (pda1_e2, _bump1_e2) = derive_bet_batch_pda(&player1, epoch2, &program_id);
        assert_eq!(pda1_e1, pda1_e1_again); // Same inputs should give same result
        assert_eq!(bump1_e1, bump1_e1_again);
        assert_ne!(pda1_e1, pda1_e2); // Different epochs should give different PDAs

        // All PDAs should be valid
        assert_ne!(pda1_e1, Pubkey::default());
        assert_ne!(pda2_e1, Pubkey::default());
        assert_ne!(pda1_e2, Pubkey::default());
    }

    #[test]
    fn test_bet_batch_size_optimization() {
        use std::mem::size_of;

        // Verify BetBatch is efficiently packed
        let bet_batch_size = size_of::<BetBatch>();
        
        // Should be reasonably sized for on-chain storage
        assert!(bet_batch_size <= 500, "BetBatch should be <= 500 bytes, got {}", bet_batch_size);
        assert!(bet_batch_size >= 200, "BetBatch should be >= 200 bytes for all data, got {}", bet_batch_size);
        
        // Should match the LEN constant
        assert_eq!(bet_batch_size, BetBatch::LEN);

        // Verify that all 16 bets can be stored with their data
        let max_bets = MAX_BETS_PER_BATCH;
        assert_eq!(max_bets, 16);
        
        // Each packed bet is 2 bytes, so 32 bytes total for packed_bets
        // Each individual payout is 8 bytes, so 128 bytes total for payouts
        // Each come point is 1 byte, so 16 bytes total
        // Each linked bet is 1 byte, so 16 bytes total
        // Each cached outcome is 1 byte, so 16 bytes total
        // Plus fixed fields and padding
        
        let calculated_minimum = 8 + 32 + 8 + 8 + 32 + 2 + 2 + 2 + 2 + 8 + 128 + 16 + 16 + 16 + 8 + 1 + 7;
        assert!(bet_batch_size >= calculated_minimum, 
            "BetBatch size {} should be at least calculated minimum {}", 
            bet_batch_size, calculated_minimum);
    }
}