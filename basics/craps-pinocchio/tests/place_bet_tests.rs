use solana_sdk::pubkey::Pubkey;

use craps_pinocchio::{
    constants::*,
    instructions::betting::PlaceBetData,
    ID,
};

/// Test suite for place bet functionality with repeater bets
#[cfg(test)]
mod place_bet_tests {
    use super::*;

    const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    fn create_place_bet_instruction_data(epoch: u64, bet_kind: u8, bet_amount: u64) -> PlaceBetData {
        PlaceBetData {
            epoch: epoch.to_le_bytes(),
            bet_kind,
            _padding1: [0; 7],
            bet_amount: bet_amount.to_le_bytes(),
            _padding2: [0; 8],
        }
    }

    #[test]
    fn test_place_bet_data_structure_without_repeater_target() {
        // Test that PlaceBetData structure works correctly without repeater_target field
        use std::mem::size_of;
        
        let place_bet_data = PlaceBetData {
            epoch: 42u64.to_le_bytes(),
            bet_kind: BET_REPEATER_6,
            _padding1: [0; 7],
            bet_amount: 10_000_000_000u64.to_le_bytes(),
            _padding2: [0; 8], // This replaced repeater_target + padding
        };

        // Should be able to serialize/deserialize without issues
        let bytes = bytemuck::bytes_of(&place_bet_data);
        let deserialized = bytemuck::from_bytes::<PlaceBetData>(bytes);

        assert_eq!(u64::from_le_bytes(deserialized.epoch), 42);
        assert_eq!(deserialized.bet_kind, BET_REPEATER_6);
        assert_eq!(u64::from_le_bytes(deserialized.bet_amount), 10_000_000_000);

        // Verify size is as expected (32 bytes total)
        assert_eq!(size_of::<PlaceBetData>(), 32);
    }

    #[test]
    fn test_repeater_bet_no_target_parameter_required() {
        // Verify that repeater bets work as standalone bet types without additional parameters
        let repeater_bets = [
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6,
            BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];

        for bet_type in repeater_bets {
            // Each repeater bet should encode its target number implicitly
            let target_number = match bet_type {
                BET_REPEATER_2 => 2,
                BET_REPEATER_3 => 3,
                BET_REPEATER_4 => 4,
                BET_REPEATER_5 => 5,
                BET_REPEATER_6 => 6,
                BET_REPEATER_8 => 8,
                BET_REPEATER_9 => 9,
                BET_REPEATER_10 => 10,
                BET_REPEATER_11 => 11,
                BET_REPEATER_12 => 12,
                _ => 0,
            };

            assert!(target_number >= 2 && target_number <= 12 && target_number != 7,
                "Repeater bet {} should encode a valid target number {}", bet_type, target_number);

            // Create place bet data - no additional target parameter needed
            let place_bet_data = PlaceBetData {
                epoch: 1u64.to_le_bytes(),
                bet_kind: bet_type,
                _padding1: [0; 7],
                bet_amount: 5_000_000_000u64.to_le_bytes(),
                _padding2: [0; 8],
            };

            // Should be valid data structure
            let serialized = bytemuck::bytes_of(&place_bet_data);
            assert_eq!(serialized.len(), 32);

            let deserialized = bytemuck::from_bytes::<PlaceBetData>(serialized);
            assert_eq!(deserialized.bet_kind, bet_type);
        }
    }

    #[test]
    fn test_all_repeater_bet_types() {
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
            
            // Should be able to create instruction data
            let instruction_data = create_place_bet_instruction_data(1, bet_type, 5_000_000_000);
            assert_eq!(instruction_data.bet_kind, bet_type);
            assert_eq!(u64::from_le_bytes(instruction_data.bet_amount), 5_000_000_000);
        }
    }

    #[test]
    fn test_bet_validation_invalid_bet_types() {
        // Test invalid bet types
        let invalid_bet_types = [64, 65, 100, 255];

        for invalid_bet_type in invalid_bet_types {
            let instruction_data = create_place_bet_instruction_data(1, invalid_bet_type, 1_000_000_000);
            assert_eq!(instruction_data.bet_kind, invalid_bet_type);
            
            // The bet type itself will be validated by the program logic
            // Here we just test that the data structure can hold any u8 value
        }
    }

    #[test] 
    fn test_bet_validation_minimum_amount() {
        // Test amounts below minimum
        let invalid_amounts = [0, 500_000_000, 999_999_999]; // Less than 1 CRAP

        for invalid_amount in invalid_amounts {
            let instruction_data = create_place_bet_instruction_data(1, BET_PASS, invalid_amount);
            assert_eq!(u64::from_le_bytes(instruction_data.bet_amount), invalid_amount);
            
            // The amount validation will be handled by the program logic
        }
    }

    #[test]
    fn test_bet_validation_epoch_handling() {
        // Test epoch boundaries
        let epochs = [0, 1, 42, u64::MAX];

        for epoch in epochs {
            let instruction_data = create_place_bet_instruction_data(epoch, BET_PASS, 1_000_000_000);
            assert_eq!(u64::from_le_bytes(instruction_data.epoch), epoch);
        }
    }

    #[test]
    fn test_repeater_bet_constants_validation() {
        // Test that all repeater bet constants are within valid range and unique
        let repeater_bets = [
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6,
            BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];

        // All should be valid bet type values
        for &bet_type in &repeater_bets {
            assert!(bet_type <= 63, "Repeater bet {} should be <= 63", bet_type);
        }

        // All should be unique
        for i in 0..repeater_bets.len() {
            for j in (i + 1)..repeater_bets.len() {
                assert_ne!(repeater_bets[i], repeater_bets[j], 
                    "Repeater bet constants should be unique");
            }
        }

        // Should be in expected range (54-63)
        assert_eq!(BET_REPEATER_2, 54);
        assert_eq!(BET_REPEATER_3, 55);
        assert_eq!(BET_REPEATER_4, 56);
        assert_eq!(BET_REPEATER_5, 57);
        assert_eq!(BET_REPEATER_6, 58);
        assert_eq!(BET_REPEATER_8, 59); // Note: skip 7
        assert_eq!(BET_REPEATER_9, 60);
        assert_eq!(BET_REPEATER_10, 61);
        assert_eq!(BET_REPEATER_11, 62);
        assert_eq!(BET_REPEATER_12, 63);
    }

    #[test]
    fn test_pda_derivation_for_betting() {
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
    fn test_instruction_data_serialization() {
        let test_cases = [
            (1u64, BET_PASS, 1_000_000_000u64),
            (42u64, BET_REPEATER_6, 5_000_000_000u64),
            (100u64, BET_FIELD, 2_000_000_000u64),
            (u64::MAX, BET_REPEATER_12, 10_000_000_000u64),
        ];

        for (epoch, bet_kind, bet_amount) in test_cases {
            let instruction_data = create_place_bet_instruction_data(epoch, bet_kind, bet_amount);
            
            // Test serialization
            let bytes = bytemuck::bytes_of(&instruction_data);
            assert_eq!(bytes.len(), 32);
            
            // Test deserialization
            let deserialized = bytemuck::from_bytes::<PlaceBetData>(bytes);
            assert_eq!(u64::from_le_bytes(deserialized.epoch), epoch);
            assert_eq!(deserialized.bet_kind, bet_kind);
            assert_eq!(u64::from_le_bytes(deserialized.bet_amount), bet_amount);
        }
    }

    #[test]
    fn test_place_bet_data_padding() {
        // Test that padding fields are properly zeroed and don't affect functionality
        let mut instruction_data = create_place_bet_instruction_data(1, BET_REPEATER_2, 1_000_000_000);
        
        // Verify padding is zeroed
        assert_eq!(instruction_data._padding1, [0; 7]);
        assert_eq!(instruction_data._padding2, [0; 8]);
        
        // Modify padding and verify core data is unaffected
        instruction_data._padding1 = [255; 7];
        instruction_data._padding2 = [128; 8];
        
        assert_eq!(u64::from_le_bytes(instruction_data.epoch), 1);
        assert_eq!(instruction_data.bet_kind, BET_REPEATER_2);
        assert_eq!(u64::from_le_bytes(instruction_data.bet_amount), 1_000_000_000);
    }
}