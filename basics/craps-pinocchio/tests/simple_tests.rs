use solana_sdk::pubkey::Pubkey;

use craps_pinocchio::{
    constants::*,
    instructions::CrapsInstruction,
    state::*,
    ID,
};

/// Test suite for basic craps-pinocchio functionality
#[cfg(test)]
mod simple_tests {
    use super::*;

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_program_id() {
        // Basic test that verifies program ID is set correctly
        assert_ne!(PROGRAM_ID, Pubkey::default());
    }

    #[test]
    fn test_pda_derivation() {
        let test_player = Pubkey::new_unique();
        let test_epoch = 42u64;
        
        // Test player PDA derivation
        let (player_pda, _) = Pubkey::find_program_address(
            &[SCALABLE_PLAYER_SEED, test_player.as_ref()],
            &PROGRAM_ID
        );
        assert_ne!(player_pda, Pubkey::default());
        
        // Test bet batch PDA derivation
        let batch_index = 0u32;
        let (bet_batch_pda, _) = Pubkey::find_program_address(
            &[BET_BATCH_SEED, test_player.as_ref(), &test_epoch.to_le_bytes(), &batch_index.to_le_bytes()],
            &PROGRAM_ID
        );
        assert_ne!(bet_batch_pda, Pubkey::default());
        
        // Test that different inputs give different PDAs
        let different_player = Pubkey::new_unique();
        let (different_pda, _) = Pubkey::find_program_address(
            &[SCALABLE_PLAYER_SEED, different_player.as_ref()],
            &PROGRAM_ID
        );
        assert_ne!(player_pda, different_pda);
        
        let different_epoch = 43u64;
        let (different_batch_pda, _) = Pubkey::find_program_address(
            &[BET_BATCH_SEED, test_player.as_ref(), &different_epoch.to_le_bytes(), &batch_index.to_le_bytes()],
            &PROGRAM_ID
        );
        assert_ne!(bet_batch_pda, different_batch_pda);
    }

    #[test]
    fn test_bet_type_constants() {
        // Test that all bet types are within valid range
        assert!(BET_PASS <= 63);
        assert!(BET_DONT_PASS <= 63);
        assert!(BET_FIELD <= 63);
        assert!(BET_COME <= 63);
        assert!(BET_DONT_COME <= 63);
        
        // Test repeater bet constants
        assert!(BET_REPEATER_2 <= 63);
        assert!(BET_REPEATER_3 <= 63);
        assert!(BET_REPEATER_4 <= 63);
        assert!(BET_REPEATER_5 <= 63);
        assert!(BET_REPEATER_6 <= 63);
        assert!(BET_REPEATER_8 <= 63);
        assert!(BET_REPEATER_9 <= 63);
        assert!(BET_REPEATER_10 <= 63);
        assert!(BET_REPEATER_11 <= 63);
        assert!(BET_REPEATER_12 <= 63);
        
        // Test that all repeater bets are unique
        let repeater_bets = [
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6,
            BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];
        
        for i in 0..repeater_bets.len() {
            for j in (i + 1)..repeater_bets.len() {
                assert_ne!(repeater_bets[i], repeater_bets[j], 
                    "Repeater bet constants should be unique");
            }
        }
    }

    #[test]
    fn test_dice_utilities() {
        use craps_pinocchio::utils::dice::*;
        
        // Test dice validation
        assert!(are_dice_valid(1, 1));
        assert!(are_dice_valid(6, 6));
        assert!(!are_dice_valid(0, 1));
        assert!(!are_dice_valid(1, 7));
        
        // Test roll calculations
        assert_eq!(calculate_roll_total(3, 4), 7);
        assert_eq!(calculate_roll_total(6, 6), 12);
        
        // Test craps detection
        assert!(is_craps(1, 1));  // 2
        assert!(is_craps(1, 2));  // 3
        assert!(is_craps(6, 6));  // 12
        assert!(!is_craps(3, 4)); // 7
        
        // Test seven out
        assert!(is_seven_out(3, 4));
        assert!(is_seven_out(1, 6));
        assert!(!is_seven_out(2, 2));
        
        // Test natural
        assert!(is_natural(3, 4));   // 7
        assert!(is_natural(5, 6));   // 11
        assert!(!is_natural(2, 2));  // 4
        
        // Test hard ways
        assert!(is_hard_way(2, 2));  // Hard 4
        assert!(is_hard_way(3, 3));  // Hard 6
        assert!(!is_hard_way(1, 3)); // Soft 4
        
        // Test field winners
        assert!(is_field_winner(1, 1));  // 2
        assert!(is_field_winner(1, 2));  // 3
        assert!(is_field_winner(2, 2));  // 4
        assert!(!is_field_winner(2, 3)); // 5
        
        // Test field multipliers
        assert_eq!(get_field_multiplier(1, 1), 2);  // 2 pays 2:1
        assert_eq!(get_field_multiplier(6, 6), 3);  // 12 pays 3:1
        assert_eq!(get_field_multiplier(1, 2), 1);  // 3 pays 1:1
    }

    #[test]
    fn test_bet_encoding() {
        use craps_pinocchio::utils::bet_encoding::*;
        
        // Test minimum amounts
        assert!(encode_bet(BET_PASS, 1_000_000_000).is_ok()); // 1 CRAP
        
        // Test maximum amounts
        assert!(encode_bet(BET_PASS, 100_000_000_000_000).is_ok()); // 100k CRAP
        
        // Test invalid amounts
        assert!(encode_bet(BET_PASS, 0).is_err());
        assert!(encode_bet(BET_PASS, 500_000_000).is_err()); // 0.5 CRAP (not whole)
        
        // Test invalid bet types
        assert!(encode_bet(64, 1_000_000_000).is_err());
        
        // Test round-trip encoding/decoding
        let original_bet = (BET_FIELD, 5_000_000_000u64); // 5 CRAP
        let encoded = encode_bet(original_bet.0, original_bet.1).unwrap();
        let decoded = decode_bet(encoded);
        assert_eq!(original_bet, decoded);
    }

    #[test]
    fn test_account_size_constraints() {
        use std::mem::size_of;
        
        // Verify account sizes match expected values
        assert_eq!(size_of::<GlobalGameState>(), GlobalGameState::LEN);
        assert_eq!(size_of::<ScalablePlayerState>(), ScalablePlayerState::LEN);
        assert_eq!(size_of::<BetBatch>(), BetBatch::LEN);
        assert_eq!(size_of::<BonusState>(), BonusState::LEN);
        assert_eq!(size_of::<Treasury>(), Treasury::LEN);
        assert_eq!(size_of::<RngState>(), RngState::LEN);
        
        // Verify all structures are reasonably sized (not too large)
        assert!(GlobalGameState::LEN < 1000);
        assert!(ScalablePlayerState::LEN < 1000);
        assert!(BetBatch::LEN < 1000);
        assert!(Treasury::LEN < 1000);
        assert!(RngState::LEN < 1000);
    }

    #[test]
    fn test_instruction_discriminants() {
        // Test that instruction discriminants are correct
        assert_eq!(CrapsInstruction::InitializeSystem as u8, 0);
        assert_eq!(CrapsInstruction::InitializePlayer as u8, 2);
        assert_eq!(CrapsInstruction::PlaceBet as u8, 8);
        assert_eq!(CrapsInstruction::SecureAutoRoll as u8, 9);
        assert_eq!(CrapsInstruction::CollectBlockHash as u8, 10);
        assert_eq!(CrapsInstruction::FinalizeRng as u8, 11);
        
        // Test TryFrom implementation
        assert_eq!(CrapsInstruction::try_from(&0u8).unwrap(), CrapsInstruction::InitializeSystem);
        assert_eq!(CrapsInstruction::try_from(&2u8).unwrap(), CrapsInstruction::InitializePlayer);
        assert_eq!(CrapsInstruction::try_from(&8u8).unwrap(), CrapsInstruction::PlaceBet);
        
        // Test invalid discriminant
        assert!(CrapsInstruction::try_from(&255u8).is_err());
    }

    #[test]
    fn test_repeater_bet_types() {
        // Test that each repeater bet type is correctly defined
        let repeater_bets = [
            (BET_REPEATER_2, "Repeater 2"),
            (BET_REPEATER_3, "Repeater 3"),
            (BET_REPEATER_4, "Repeater 4"),
            (BET_REPEATER_5, "Repeater 5"),
            (BET_REPEATER_6, "Repeater 6"),
            (BET_REPEATER_8, "Repeater 8"),
            (BET_REPEATER_9, "Repeater 9"),
            (BET_REPEATER_10, "Repeater 10"),
            (BET_REPEATER_11, "Repeater 11"),
            (BET_REPEATER_12, "Repeater 12"),
        ];
        
        for (bet_type, description) in repeater_bets {
            // Each repeater bet should be a valid bet type
            assert!(bet_type <= 63, "{} should be <= 63", description);
            
            // Should be able to encode/decode the bet
            use craps_pinocchio::utils::bet_encoding::*;
            let encoded = encode_bet(bet_type, 10_000_000_000).unwrap(); // 10 CRAP
            let (decoded_type, decoded_amount) = decode_bet(encoded);
            assert_eq!(decoded_type, bet_type);
            assert_eq!(decoded_amount, 10_000_000_000);
        }
    }

    #[test]
    fn test_place_bet_data_structure() {
        use craps_pinocchio::instructions::betting::PlaceBetData;
        use std::mem::size_of;
        
        // Test that PlaceBetData has the expected size after removing repeater_target
        let expected_size = 8 + 1 + 7 + 8 + 8; // epoch + bet_kind + padding1 + bet_amount + padding2
        assert_eq!(size_of::<PlaceBetData>(), expected_size);
        
        // Test that PlaceBetData is Pod and Zeroable
        let mut data = [0u8; size_of::<PlaceBetData>()];
        let place_bet_data = bytemuck::from_bytes_mut::<PlaceBetData>(&mut data);
        
        // Should be able to set values
        place_bet_data.epoch = 42u64.to_le_bytes();
        place_bet_data.bet_kind = BET_PASS;
        place_bet_data.bet_amount = 1_000_000_000u64.to_le_bytes();
        
        // Should be able to read them back
        assert_eq!(u64::from_le_bytes(place_bet_data.epoch), 42);
        assert_eq!(place_bet_data.bet_kind, BET_PASS);
        assert_eq!(u64::from_le_bytes(place_bet_data.bet_amount), 1_000_000_000);
    }

    #[test] 
    fn test_bet_batch_structure() {
        use std::mem::size_of;
        
        // Test that BetBatch no longer has repeater_targets field
        let mut data = [0u8; size_of::<BetBatch>()];
        let bet_batch = bytemuck::from_bytes_mut::<BetBatch>(&mut data);
        
        // Should be able to use basic functionality
        bet_batch.set_epoch(1);
        bet_batch.bet_count = 0;
        bet_batch.set_total_amount(0);
        
        // Test packed bet functionality
        bet_batch.set_packed_bet(0, 0x1234);
        assert_eq!(bet_batch.get_packed_bet(0), 0x1234);
        
        // Test mask functionality
        bet_batch.set_resolved_mask(0b1010);
        assert_eq!(bet_batch.get_resolved_mask(), 0b1010);
        assert!(bet_batch.is_bet_resolved(1));
        assert!(!bet_batch.is_bet_resolved(0));
    }
}