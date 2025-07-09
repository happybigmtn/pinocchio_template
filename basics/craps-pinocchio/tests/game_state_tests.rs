use craps_pinocchio::state::*;

/// Test suite for game state transitions and dice roll handling
#[cfg(test)]
mod game_state_tests {
    use super::*;

    #[test]
    fn test_come_out_roll_natural_seven() {
        // Test basic dice roll validation logic
        use craps_pinocchio::utils::dice::*;
        
        // Test that 7 is recognized as a natural
        assert!(is_natural(3, 4));
        assert!(is_natural(2, 5));
        assert!(is_natural(1, 6));
        
        // Test that 7 is a seven out
        assert!(is_seven_out(3, 4));
        assert!(is_seven_out(2, 5));
        assert!(is_seven_out(1, 6));
    }

    #[test]
    fn test_come_out_roll_establishes_point() {
        use craps_pinocchio::utils::dice::*;
        
        // Test point establishment (4, 5, 6, 8, 9, 10)
        let point_numbers = [4, 5, 6, 8, 9, 10];
        
        for point in point_numbers {
            // Should not be natural or craps
            assert!(!is_natural(point / 2, point - point / 2));
            assert!(!is_craps(point / 2, point - point / 2));
        }
    }

    #[test]
    fn test_repeater_bet_hit_tracking() {
        let mut bonus_state = BonusState::new();
        
        // Test that repeater hits are tracked correctly
        bonus_state.update_for_roll(1, 1, 2, 0); // Repeater 2 hit
        assert_eq!(bonus_state.get_hit_count(2), 1);
        
        bonus_state.update_for_roll(6, 6, 12, 0); // Repeater 12 hit
        assert_eq!(bonus_state.get_hit_count(12), 1);
        
        // Test multiple hits
        bonus_state.update_for_roll(1, 1, 2, 0); // Another repeater 2 hit
        assert_eq!(bonus_state.get_hit_count(2), 2);
        
        // Test seven out resets
        bonus_state.reset_on_seven_out();
        assert_eq!(bonus_state.get_hit_count(2), 0);
        assert_eq!(bonus_state.get_hit_count(12), 0);
    }

    #[test]
    fn test_game_state_validation() {
        // Test basic game state structure validation
        let mut game_state: GlobalGameState = unsafe { std::mem::zeroed() };
        
        // Test epoch increments
        game_state.game_epoch = 1u64.to_le_bytes();
        assert_eq!(u64::from_le_bytes(game_state.game_epoch), 1);
        
        let new_epoch = 2u64;
        game_state.game_epoch = new_epoch.to_le_bytes();
        assert_eq!(u64::from_le_bytes(game_state.game_epoch), new_epoch);
        
        // Test dice validation
        game_state.current_die1 = 3;
        game_state.current_die2 = 4;
        assert!(game_state.current_die1 >= 1 && game_state.current_die1 <= 6);
        assert!(game_state.current_die2 >= 1 && game_state.current_die2 <= 6);
    }

    #[test]
    fn test_bonus_state_integration() {
        let mut bonus_state = BonusState::new();

        // Test that bonus state integrates properly with game flow
        
        // Simulate some game events
        bonus_state.update_for_roll(2, 2, 4, 0); // Hard 4
        assert!(bonus_state.is_double_rolled(2));

        bonus_state.update_for_roll(1, 1, 2, 0); // Repeater 2 hit
        assert_eq!(bonus_state.get_hit_count(2), 1);

        bonus_state.mark_small_rolled(2);
        bonus_state.mark_small_rolled(3);
        bonus_state.mark_small_rolled(4);
        bonus_state.mark_small_rolled(5);
        bonus_state.mark_small_rolled(6);
        assert!(bonus_state.all_small_rolled());

        // Test seven out resets everything
        bonus_state.reset_on_seven_out();
        assert_eq!(bonus_state.get_hit_count(2), 0);
        assert!(!bonus_state.is_double_rolled(2));
        assert!(!bonus_state.all_small_rolled());
    }

    #[test]
    fn test_epoch_progression() {
        let mut game_state: GlobalGameState = unsafe { std::mem::zeroed() };
        let initial_epoch = 1u64;
        game_state.game_epoch = initial_epoch.to_le_bytes();

        // Test epoch increments on game events
        let new_epoch = initial_epoch + 1;
        game_state.game_epoch = new_epoch.to_le_bytes();

        assert_eq!(u64::from_le_bytes(game_state.game_epoch), new_epoch);
        assert_ne!(u64::from_le_bytes(game_state.game_epoch), initial_epoch);
    }

    #[test]
    fn test_rng_state_security() {
        let mut rng_state: RngState = unsafe { std::mem::zeroed() };

        // Test that RNG state has proper phase management
        rng_state.set_phase(RngPhase::Collection);
        assert!(matches!(rng_state.get_phase(), RngPhase::Collection));

        rng_state.set_phase(RngPhase::Betting);
        assert!(matches!(rng_state.get_phase(), RngPhase::Betting));

        rng_state.set_phase(RngPhase::Finalized);
        assert!(matches!(rng_state.get_phase(), RngPhase::Finalized));

        // Test epoch management
        rng_state.set_epoch(42);
        assert_eq!(rng_state.get_epoch(), 42);

        // Test final value
        rng_state.set_final_value(12345);
        assert_eq!(rng_state.get_final_value(), 12345);

        // Test dice generation
        let (die1, die2) = rng_state.get_dice_roll();
        assert!(die1 >= 1 && die1 <= 6);
        assert!(die2 >= 1 && die2 <= 6);
    }

    #[test]
    fn test_phase_transitions() {
        // Test RNG phase transitions
        let mut rng_state: RngState = unsafe { std::mem::zeroed() };
        
        // Start in betting phase
        rng_state.set_phase(RngPhase::Betting);
        assert!(matches!(rng_state.get_phase(), RngPhase::Betting));
        
        // Transition to collection
        rng_state.set_phase(RngPhase::Collection);
        assert!(matches!(rng_state.get_phase(), RngPhase::Collection));
        
        // Finalize
        rng_state.set_phase(RngPhase::Finalized);
        assert!(matches!(rng_state.get_phase(), RngPhase::Finalized));
        assert!(rng_state.is_finalized());
    }

    #[test]
    fn test_dice_utilities_comprehensive() {
        use craps_pinocchio::utils::dice::*;

        // Test all valid dice combinations
        for d1 in 1..=6 {
            for d2 in 1..=6 {
                assert!(are_dice_valid(d1, d2));
                let total = calculate_roll_total(d1, d2);
                assert!(total >= 2 && total <= 12);
                
                // Test specific conditions
                if total == 7 || total == 11 {
                    assert!(is_natural(d1, d2));
                }
                
                if total == 2 || total == 3 || total == 12 {
                    assert!(is_craps(d1, d2));
                }
                
                if total == 7 {
                    assert!(is_seven_out(d1, d2));
                }
                
                if d1 == d2 && (total == 4 || total == 6 || total == 8 || total == 10) {
                    assert!(is_hard_way(d1, d2));
                }
            }
        }
    }
}