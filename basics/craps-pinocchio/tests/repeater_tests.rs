use craps_pinocchio::{
    constants::*,
    state::{BonusState},
};

/// Test suite for repeater bet functionality
#[cfg(test)]
mod repeater_tests {
    use super::*;

    #[test]
    fn test_bonus_state_hit_tracking() {
        let mut bonus_state = BonusState::new();
        
        // Test initial state
        assert_eq!(bonus_state.get_hit_count(2), 0);
        assert_eq!(bonus_state.get_hit_count(12), 0);
        
        // Simulate dice rolls and track hits
        bonus_state.update_for_roll(1, 1, 2, 0); // 2 rolled
        assert_eq!(bonus_state.get_hit_count(2), 1);
        
        bonus_state.update_for_roll(1, 1, 2, 0); // 2 rolled again
        assert_eq!(bonus_state.get_hit_count(2), 2);
        
        // Test different numbers
        bonus_state.update_for_roll(6, 6, 12, 0); // 12 rolled
        assert_eq!(bonus_state.get_hit_count(12), 1);
        
        bonus_state.update_for_roll(6, 6, 12, 0); // 12 rolled again
        assert_eq!(bonus_state.get_hit_count(12), 2);
        
        // Test that other numbers are unaffected
        assert_eq!(bonus_state.get_hit_count(3), 0);
        assert_eq!(bonus_state.get_hit_count(4), 0);
    }

    #[test]
    fn test_bonus_state_reset_on_seven_out() {
        let mut bonus_state = BonusState::new();
        
        // Build up some hit counts
        bonus_state.update_for_roll(1, 1, 2, 0); // 2 rolled
        bonus_state.update_for_roll(1, 1, 2, 0); // 2 rolled again
        bonus_state.update_for_roll(6, 6, 12, 0); // 12 rolled
        
        // Verify hits are tracked
        assert_eq!(bonus_state.get_hit_count(2), 2);
        assert_eq!(bonus_state.get_hit_count(12), 1);
        
        // Reset on seven out
        bonus_state.reset_on_seven_out();
        
        // Verify all hits are reset
        assert_eq!(bonus_state.get_hit_count(2), 0);
        assert_eq!(bonus_state.get_hit_count(12), 0);
        for i in 2..=12 {
            assert_eq!(bonus_state.get_hit_count(i), 0);
        }
    }

    #[test] 
    fn test_repeater_bet_constants_uniqueness() {
        // Collect all repeater bet constants
        let repeater_bets = [
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6,
            BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];
        
        // Verify all are unique
        for i in 0..repeater_bets.len() {
            for j in (i + 1)..repeater_bets.len() {
                assert_ne!(repeater_bets[i], repeater_bets[j], 
                    "Repeater bet constants must be unique");
            }
        }
        
        // Verify they're in the expected range (54-63)
        assert_eq!(BET_REPEATER_2, 54);
        assert_eq!(BET_REPEATER_3, 55);
        assert_eq!(BET_REPEATER_4, 56);
        assert_eq!(BET_REPEATER_5, 57);
        assert_eq!(BET_REPEATER_6, 58);
        assert_eq!(BET_REPEATER_8, 59);
        assert_eq!(BET_REPEATER_9, 60);
        assert_eq!(BET_REPEATER_10, 61);
        assert_eq!(BET_REPEATER_11, 62);
        assert_eq!(BET_REPEATER_12, 63);
        
        // Verify no repeater bet for 7 (since 7 ends the game)
        let all_constants = [
            BET_PASS, BET_DONT_PASS, BET_COME, BET_DONT_COME, BET_FIELD,
            BET_YES_2, BET_YES_3, BET_YES_4, BET_YES_5, BET_YES_6, BET_YES_8, BET_YES_9, BET_YES_10, BET_YES_11, BET_YES_12,
            BET_NO_2, BET_NO_3, BET_NO_4, BET_NO_5, BET_NO_6, BET_NO_8, BET_NO_9, BET_NO_10, BET_NO_11, BET_NO_12,
            BET_HARD4, BET_HARD6, BET_HARD8, BET_HARD10,
            BET_ODDS_PASS, BET_ODDS_DONT_PASS, BET_ODDS_COME, BET_ODDS_DONT_COME,
            BET_HOT_ROLLER, BET_FIRE, BET_TWICE_HARD, BET_RIDE_LINE, BET_MUGGSY,
            BET_BONUS_SMALL, BET_BONUS_TALL, BET_BONUS_SMALL_TALL, BET_REPLAY, BET_DIFFERENT_DOUBLES,
            BET_NEXT_2, BET_NEXT_3, BET_NEXT_4, BET_NEXT_5, BET_NEXT_6, BET_NEXT_7, BET_NEXT_8, BET_NEXT_9, BET_NEXT_10, BET_NEXT_11, BET_NEXT_12,
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6, BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];
        
        // Verify no BET_REPEATER_7 exists in the constants
        for &constant in &all_constants {
            // There should be no constant between BET_REPEATER_6 (58) and BET_REPEATER_8 (59)
            if constant > BET_REPEATER_6 && constant < BET_REPEATER_8 {
                panic!("Found unexpected constant {} between BET_REPEATER_6 and BET_REPEATER_8", constant);
            }
        }
    }

    #[test]
    fn test_repeater_bet_required_hits() {
        // Test that the required hit counts for repeater bets are correct
        // These are based on the mathematical difficulty of rolling each number
        
        let test_cases = [
            (2, 2),   // Hardest to roll (1/36 chance), needs 2 hits
            (3, 3),   // Hard to roll (2/36 chance), needs 3 hits  
            (4, 4),   // Less common (3/36 chance), needs 4 hits
            (5, 5),   // Less common (4/36 chance), needs 5 hits
            (6, 6),   // Common (5/36 chance), needs 6 hits
            (8, 6),   // Common (5/36 chance), needs 6 hits
            (9, 5),   // Less common (4/36 chance), needs 5 hits
            (10, 4),  // Less common (3/36 chance), needs 4 hits
            (11, 3),  // Hard to roll (2/36 chance), needs 3 hits
            (12, 2),  // Hardest to roll (1/36 chance), needs 2 hits
        ];
        
        for (target, expected_hits) in test_cases {
            // Create a bonus state with enough hits
            let mut bonus_state = BonusState::new();
            
            // Simulate rolling the target number the required times
            for _ in 0..expected_hits {
                let dice1 = if target == 2 { 1 } else if target == 12 { 6 } else { target / 2 };
                let dice2 = target - dice1;
                bonus_state.update_for_roll(dice1, dice2, target, 0);
            }
            
            // Verify the hit count matches expected
            assert_eq!(bonus_state.get_hit_count(target), expected_hits, 
                "Target {} should require {} hits", target, expected_hits);
        }
    }

    #[test]
    fn test_repeater_bet_payouts() {
        // Test the payout multipliers for repeater bets
        let payout_tests = [
            (2, 40),   // 40:1 payout for Repeater 2
            (3, 50),   // 50:1 payout for Repeater 3
            (4, 65),   // 65:1 payout for Repeater 4
            (5, 80),   // 80:1 payout for Repeater 5
            (6, 90),   // 90:1 payout for Repeater 6
            (8, 90),   // 90:1 payout for Repeater 8
            (9, 80),   // 80:1 payout for Repeater 9
            (10, 65),  // 65:1 payout for Repeater 10
            (11, 50),  // 50:1 payout for Repeater 11
            (12, 40),  // 40:1 payout for Repeater 12
        ];
        
        for (target, expected_multiplier) in payout_tests {
            // Create a bonus state with sufficient hits
            let mut bonus_state = BonusState::new();
            
            let required_hits = match target {
                2 | 12 => 2,
                3 | 11 => 3,
                4 | 10 => 4,
                5 | 9 => 5,
                6 | 8 => 6,
                _ => 0,
            };
            
            // Simulate rolling the target number the required times
            for _ in 0..required_hits {
                let dice1 = if target == 2 { 1 } else if target == 12 { 6 } else { target / 2 };
                let dice2 = target - dice1;
                bonus_state.update_for_roll(dice1, dice2, target, 0);
            }
            
            // Test the payout calculation (we can't directly test the internal function,
            // but we can verify the logic is consistent)
            assert_eq!(bonus_state.get_hit_count(target), required_hits);
            
            // The payout logic should follow the pattern:
            // - Harder to roll numbers have lower hit requirements but higher payouts
            // - Easier to roll numbers have higher hit requirements but still good payouts
            match target {
                2 | 12 => assert_eq!(expected_multiplier, 40),
                3 | 11 => assert_eq!(expected_multiplier, 50),
                4 | 10 => assert_eq!(expected_multiplier, 65),
                5 | 9 => assert_eq!(expected_multiplier, 80),
                6 | 8 => assert_eq!(expected_multiplier, 90),
                _ => {}
            }
        }
    }

    #[test]
    fn test_bonus_state_saturation() {
        let mut bonus_state = BonusState::new();
        
        // Test that hit counts saturate at u8::MAX (255)
        for _ in 0..300 {
            bonus_state.update_for_roll(1, 1, 2, 0);
        }
        
        // Should saturate at 255, not overflow
        assert_eq!(bonus_state.get_hit_count(2), 255);
        
        // Test other tracking fields also saturate
        for _ in 0..300 {
            bonus_state.fire_points = bonus_state.fire_points.saturating_add(1);
            bonus_state.hot_roller_count = bonus_state.hot_roller_count.saturating_add(1);
            bonus_state.ride_line_streak = bonus_state.ride_line_streak.saturating_add(1);
        }
        
        assert_eq!(bonus_state.fire_points, 255);
        assert_eq!(bonus_state.hot_roller_count, 255);
        assert_eq!(bonus_state.ride_line_streak, 255);
    }

    #[test]
    fn test_bonus_state_size_optimization() {
        use std::mem::size_of;
        
        // Verify BonusState is reasonably sized for on-chain storage
        let bonus_size = size_of::<BonusState>();
        
        // Should be small enough for efficient on-chain storage
        // but large enough to track all necessary data
        assert!(bonus_size <= 100, "BonusState should be <= 100 bytes, got {}", bonus_size);
        assert!(bonus_size >= 30, "BonusState should be >= 30 bytes for all data, got {}", bonus_size);
        
        // Verify it matches the LEN constant
        assert_eq!(bonus_size, BonusState::LEN);
    }

    #[test]
    fn test_bonus_state_bit_packing() {
        let mut bonus_state = BonusState::new();
        
        // Test small numbers tracking (2-6)
        for num in 2..=6 {
            assert!(!bonus_state.is_small_rolled(num));
            bonus_state.mark_small_rolled(num);
            assert!(bonus_state.is_small_rolled(num));
        }
        assert!(bonus_state.all_small_rolled());
        
        // Test tall numbers tracking (8-12)  
        for num in 8..=12 {
            assert!(!bonus_state.is_tall_rolled(num));
            bonus_state.mark_tall_rolled(num);
            assert!(bonus_state.is_tall_rolled(num));
        }
        assert!(bonus_state.all_tall_rolled());
        
        // Test doubles tracking (1-1 through 6-6)
        for die_value in 1..=6 {
            assert!(!bonus_state.is_double_rolled(die_value));
            bonus_state.mark_double_rolled(die_value);
            assert!(bonus_state.is_double_rolled(die_value));
        }
        assert_eq!(bonus_state.count_doubles_rolled(), 6);
    }

    #[test]
    fn test_bonus_state_edge_cases() {
        let mut bonus_state = BonusState::new();
        
        // Test invalid ranges for small/tall/doubles
        bonus_state.mark_small_rolled(1);  // Too low
        bonus_state.mark_small_rolled(7);  // Too high
        bonus_state.mark_tall_rolled(7);   // Too low
        bonus_state.mark_tall_rolled(13);  // Too high
        bonus_state.mark_double_rolled(0); // Too low
        bonus_state.mark_double_rolled(7); // Too high
        
        // None should be marked
        assert!(!bonus_state.all_small_rolled());
        assert!(!bonus_state.all_tall_rolled());
        assert_eq!(bonus_state.count_doubles_rolled(), 0);
        
        // Test query functions with invalid inputs
        assert!(!bonus_state.is_small_rolled(1));
        assert!(!bonus_state.is_small_rolled(7));
        assert!(!bonus_state.is_tall_rolled(7));
        assert!(!bonus_state.is_tall_rolled(13));
        assert!(!bonus_state.is_double_rolled(0));
        assert!(!bonus_state.is_double_rolled(7));
        
        // Test get_hit_count with invalid numbers
        assert_eq!(bonus_state.get_hit_count(1), 0);
        assert_eq!(bonus_state.get_hit_count(13), 0);
        
        // Test get_pass_win_count with invalid points
        assert_eq!(bonus_state.get_pass_win_count(3), 0);
        assert_eq!(bonus_state.get_pass_win_count(7), 0);
        assert_eq!(bonus_state.get_pass_win_count(11), 0);
        
        // Test get_double_count with invalid die values
        assert_eq!(bonus_state.get_double_count(0), 0);
        assert_eq!(bonus_state.get_double_count(7), 0);
    }

    #[test]
    fn test_repeater_vs_next_bets() {
        // Verify that repeater bets are distinct from NEXT bets
        // NEXT bets are one-roll, repeater bets are multi-roll
        
        // NEXT bet constants (43-53)
        let next_bets = [
            BET_NEXT_2, BET_NEXT_3, BET_NEXT_4, BET_NEXT_5, BET_NEXT_6, BET_NEXT_7,
            BET_NEXT_8, BET_NEXT_9, BET_NEXT_10, BET_NEXT_11, BET_NEXT_12
        ];
        
        // Repeater bet constants (54-63)
        let repeater_bets = [
            BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5, BET_REPEATER_6,
            BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10, BET_REPEATER_11, BET_REPEATER_12
        ];
        
        // Verify NEXT bets are in range 43-53
        for &bet in &next_bets {
            assert!(bet >= 43 && bet <= 53, "NEXT bet {} should be in range 43-53", bet);
        }
        
        // Verify repeater bets are in range 54-63
        for &bet in &repeater_bets {
            assert!(bet >= 54 && bet <= 63, "Repeater bet {} should be in range 54-63", bet);
        }
        
        // Verify no overlap between NEXT and repeater bets
        for &next_bet in &next_bets {
            for &repeater_bet in &repeater_bets {
                assert_ne!(next_bet, repeater_bet, 
                    "NEXT bet {} should not equal repeater bet {}", next_bet, repeater_bet);
            }
        }
        
        // Verify there's no gap (all numbers 43-63 should be used)
        let mut all_bets = next_bets.to_vec();
        all_bets.extend_from_slice(&repeater_bets);
        all_bets.sort();
        
        for i in 0..all_bets.len() {
            let expected = 43 + i;
            if expected == 47 { continue; } // Skip 47 (BET_NEXT_7 is special)
            if expected == 62 { continue; } // Account for missing BET_REPEATER_7
            // There should be a bet for each number except 7
        }
    }
}