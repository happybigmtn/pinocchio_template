use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account as SolanaAccount,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    system_program,
};
use pinocchio::{
    pubkey::Pubkey,
    pubkey,
    sysvars::clock,
};

mod common;
use common::*;
use craps_pinocchio::*;
use craps_pinocchio::constants::*;

/// Test suite for comprehensive craps-pinocchio functionality
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_complete_initialization_flow() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority) = create_test_keypairs();
        let (global_state_pda, treasury_pda, rng_state_pda) = derive_pdas();
        
        // Test system initialization
        let init_data = create_instruction_data(CrapsInstruction::InitializeSystem as u8, &[]);
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &init_data,
            vec![
                AccountMeta::new(global_state_pda, false),
                AccountMeta::new(treasury_pda, false),
                AccountMeta::new(rng_state_pda, false),
                AccountMeta::new(admin.pubkey(), true),
                AccountMeta::new_readonly(pinocchio_system::ID, false),
            ],
        );
        
        let result = mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (global_state_pda, SolanaAccount::new(
                    mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                    GlobalGameState::LEN,
                    &PROGRAM_ID,
                )),
                (treasury_pda, SolanaAccount::new(
                    mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                    Treasury::LEN,
                    &PROGRAM_ID,
                )),
                (rng_state_pda, SolanaAccount::new(
                    mollusk.sysvars.rent.minimum_balance(RngState::LEN),
                    RngState::LEN,
                    &PROGRAM_ID,
                )),
                (admin.pubkey(), SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(0),
                    0,
                    &system_program::ID,
                )),
            ],
            &[
                Check::success(),
                Check::account(&global_state_pda)
                    .owner(&PROGRAM_ID)
                    .data_len(GlobalGameState::LEN)
                    .data(|data| {
                        let state = check_account_data::<GlobalGameState>(data);
                        state.get_current_epoch() == 1
                    })
                    .build(),
                Check::account(&treasury_pda)
                    .owner(&PROGRAM_ID)
                    .data_len(Treasury::LEN)
                    .build(),
                Check::account(&rng_state_pda)
                    .owner(&PROGRAM_ID)
                    .data_len(RngState::LEN)
                    .build(),
            ],
        );
        
        assert!(result.program_result.is_ok());
    }

    #[test]
    fn test_player_lifecycle() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        // Create player
        let (player_pda, bump) = derive_player_pda(&player.pubkey());
        
        let init_player_data = create_instruction_data(
            CrapsInstruction::InitializePlayer as u8,
            &[bump],
        );
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &init_player_data,
            vec![
                AccountMeta::new(player.pubkey(), true),
                AccountMeta::new(player_pda, false),
                AccountMeta::new_readonly(global_state_pda, false),
                AccountMeta::new_readonly(treasury_pda, false),
                AccountMeta::new_readonly(rng_state_pda, false),
                AccountMeta::new_readonly(pinocchio_system::ID, false),
            ],
        );
        
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (player.pubkey(), SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(0),
                    0,
                    &system_program::ID,
                )),
                (player_pda, SolanaAccount::new(
                    mollusk.sysvars.rent.minimum_balance(ScalablePlayerState::LEN),
                    ScalablePlayerState::LEN,
                    &PROGRAM_ID,
                )),
                (global_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                    GlobalGameState::LEN,
                    &PROGRAM_ID,
                )),
                (treasury_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                    Treasury::LEN,
                    &PROGRAM_ID,
                )),
                (rng_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(RngState::LEN),
                    RngState::LEN,
                    &PROGRAM_ID,
                )),
            ],
            &[
                Check::success(),
                Check::account(&player_pda)
                    .owner(&PROGRAM_ID)
                    .data_len(ScalablePlayerState::LEN)
                    .data(|data| {
                        let player_state = check_account_data::<ScalablePlayerState>(data);
                        player_state.get_balance() == 0
                    })
                    .build(),
            ],
        );
    }

    #[test]
    fn test_comprehensive_betting_system() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        let player_pda = create_player_account(
            &mollusk,
            &player,
            &global_state_pda,
            &treasury_pda,
            &rng_state_pda,
        );
        
        // Test each major bet type
        let bet_scenarios = vec![
            (BET_PASS, 100u64, None, "Pass Line"),
            (BET_DONT_PASS, 100u64, None, "Don't Pass"),
            (BET_FIELD, 50u64, None, "Field"),
            (BET_COME, 75u64, None, "Come"),
            (BET_DONT_COME, 75u64, None, "Don't Come"),
            (BET_YES_4, 25u64, None, "Yes 4"),
            (BET_NO_4, 25u64, None, "No 4"),
            (BET_HARD_4, 10u64, None, "Hard 4"),
            (BET_REPEATER_4, 20u64, Some(3u8), "Repeater 4 (3 times)"),
        ];
        
        for (bet_type, amount, repeater_target, description) in bet_scenarios {
            let bet_batch_pda = place_bet(
                &mollusk,
                &player,
                &player_pda,
                &global_state_pda,
                &treasury_pda,
                bet_type,
                amount,
                repeater_target,
            );
            
            // Verify bet was placed correctly
            assert_ne!(bet_batch_pda, Pubkey::default(), "Failed to place bet: {}", description);
        }
    }

    #[test]
    fn test_rng_cycle() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        // 1. Start betting phase
        let start_betting_data = create_instruction_data(
            CrapsInstruction::StartBettingPhase as u8,
            &[],
        );
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &start_betting_data,
            vec![
                AccountMeta::new(rng_state_pda, false),
                AccountMeta::new(global_state_pda, false),
                AccountMeta::new_readonly(rng_authority.pubkey(), true),
                AccountMeta::new_readonly(clock::ID, false),
            ],
        );
        
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (rng_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(RngState::LEN),
                    RngState::LEN,
                    &PROGRAM_ID,
                )),
                (global_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                    GlobalGameState::LEN,
                    &PROGRAM_ID,
                )),
                (rng_authority.pubkey(), SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(0),
                    0,
                    &system_program::ID,
                )),
            ],
            &[
                Check::success(),
                Check::account(&rng_state_pda)
                    .data(|data| {
                        let rng_state = check_account_data::<RngState>(data);
                        rng_state.rng_phase == RngPhase::Betting as u8
                    })
                    .build(),
            ],
        );
        
        // 2. Simulate collecting block hashes
        for i in 0..5 {  // Collect 5 hashes for testing
            let collect_data = create_instruction_data(
                CrapsInstruction::CollectBlockHash as u8,
                &[],
            );
            
            let instruction = Instruction::new_with_bytes(
                PROGRAM_ID,
                &collect_data,
                vec![
                    AccountMeta::new(rng_state_pda, false),
                    AccountMeta::new_readonly(rng_authority.pubkey(), true),
                    AccountMeta::new_readonly(clock::ID, false),
                ],
            );
            
            mollusk.process_and_validate_instruction(
                &instruction,
                &vec![
                    (rng_state_pda, SolanaAccount::new_ref(
                        mollusk.sysvars.rent.minimum_balance(RngState::LEN),
                        RngState::LEN,
                        &PROGRAM_ID,
                    )),
                    (rng_authority.pubkey(), SolanaAccount::new_ref(
                        mollusk.sysvars.rent.minimum_balance(0),
                        0,
                        &system_program::ID,
                    )),
                ],
                &[
                    Check::success(),
                ],
            );
        }
        
        // 3. Finalize RNG
        let finalize_data = create_instruction_data(
            CrapsInstruction::FinalizeRng as u8,
            &[],
        );
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &finalize_data,
            vec![
                AccountMeta::new(rng_state_pda, false),
                AccountMeta::new(global_state_pda, false),
                AccountMeta::new_readonly(rng_authority.pubkey(), true),
            ],
        );
        
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (rng_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(RngState::LEN),
                    RngState::LEN,
                    &PROGRAM_ID,
                )),
                (global_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                    GlobalGameState::LEN,
                    &PROGRAM_ID,
                )),
                (rng_authority.pubkey(), SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(0),
                    0,
                    &system_program::ID,
                )),
            ],
            &[
                Check::success(),
                Check::account(&rng_state_pda)
                    .data(|data| {
                        let rng_state = check_account_data::<RngState>(data);
                        rng_state.rng_phase == RngPhase::Finalized as u8
                    })
                    .build(),
            ],
        );
        
        // 4. Now test dice roll
        simulate_dice_roll(&mollusk, &global_state_pda, &rng_state_pda, &rng_authority);
    }

    #[test]
    fn test_game_phase_transitions() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        let player_pda = create_player_account(
            &mollusk,
            &player,
            &global_state_pda,
            &treasury_pda,
            &rng_state_pda,
        );
        
        // Place a pass bet
        let bet_batch_pda = place_bet(
            &mollusk,
            &player,
            &player_pda,
            &global_state_pda,
            &treasury_pda,
            BET_PASS,
            100,
            None,
        );
        
        // Test multiple dice rolls to see phase transitions
        for round in 0..3 {
            simulate_dice_roll(&mollusk, &global_state_pda, &rng_state_pda, &rng_authority);
            
            // Check that dice were rolled
            // Note: In a real test, you'd check the actual state to verify phase transitions
            // For now, we just ensure the roll succeeds
        }
    }

    #[test]
    fn test_bet_encoding_edge_cases() {
        use craps_pinocchio::utils::bet_encoding::*;
        
        // Test minimum amounts
        assert!(encode_bet(BET_PASS, 1).is_ok());
        
        // Test maximum amounts
        assert!(encode_bet(BET_PASS, 100000).is_ok());
        
        // Test invalid amounts
        assert!(encode_bet(BET_PASS, 0).is_err());
        assert!(encode_bet(BET_PASS, 100001).is_err());
        
        // Test invalid bet types
        assert!(encode_bet(64, 100).is_err());
        
        // Test round-trip encoding/decoding
        let original_bet = (BET_FIELD, 500u64);
        let encoded = encode_bet(original_bet.0, original_bet.1).unwrap();
        let decoded = decode_bet(encoded);
        assert_eq!(original_bet, decoded);
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
        assert_eq!(get_field_multiplier(6, 6), 2);  // 12 pays 2:1
        assert_eq!(get_field_multiplier(1, 2), 1);  // 3 pays 1:1
    }

    #[test]
    fn test_pda_derivation() {
        let test_player = Pubkey::new_unique();
        let test_epoch = 42u64;
        
        // Test player PDA
        let (player_pda, _) = derive_player_pda(&test_player);
        assert_ne!(player_pda, Pubkey::default());
        
        // Test bet batch PDA
        let (bet_batch_pda, _) = derive_bet_batch_pda(&test_player, test_epoch);
        assert_ne!(bet_batch_pda, Pubkey::default());
        
        // Test that different inputs give different PDAs
        let different_player = Pubkey::new_unique();
        let (different_pda, _) = derive_player_pda(&different_player);
        assert_ne!(player_pda, different_pda);
        
        let different_epoch = 43u64;
        let (different_batch_pda, _) = derive_bet_batch_pda(&test_player, different_epoch);
        assert_ne!(bet_batch_pda, different_batch_pda);
    }

    #[test]
    fn test_error_handling() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        // Test invalid bet type
        let invalid_bet_data = create_instruction_data(
            CrapsInstruction::PlaceBet as u8,
            &[
                64u8, // Invalid bet type
                100u64.to_le_bytes().as_slice(),
                &[0u8], // repeater target
            ].concat(),
        );
        
        let instruction = Instruction::new_with_bytes(
            PROGRAM_ID,
            &invalid_bet_data,
            vec![
                AccountMeta::new(Pubkey::new_unique(), false),
                AccountMeta::new(Pubkey::new_unique(), false),
                AccountMeta::new(global_state_pda, false),
                AccountMeta::new_readonly(treasury_pda, false),
                AccountMeta::new_readonly(clock::ID, false),
            ],
        );
        
        let result = mollusk.process_instruction(
            &instruction,
            &vec![
                (Pubkey::new_unique(), SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(ScalablePlayerState::LEN),
                    ScalablePlayerState::LEN,
                    &PROGRAM_ID,
                )),
                (Pubkey::new_unique(), SolanaAccount::new(
                    mollusk.sysvars.rent.minimum_balance(BetBatch::LEN),
                    BetBatch::LEN,
                    &PROGRAM_ID,
                )),
                (global_state_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                    GlobalGameState::LEN,
                    &PROGRAM_ID,
                )),
                (treasury_pda, SolanaAccount::new_ref(
                    mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                    Treasury::LEN,
                    &PROGRAM_ID,
                )),
            ],
        );
        
        assert!(result.program_result.is_err());
    }

    #[test]
    fn test_performance_characteristics() {
        let mollusk = create_mollusk();
        let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
            setup_basic_game_state(&mollusk);
        
        let player_pda = create_player_account(
            &mollusk,
            &player,
            &global_state_pda,
            &treasury_pda,
            &rng_state_pda,
        );
        
        // Test placing maximum bets in a single batch
        let max_bets = 16;
        
        for i in 0..max_bets {
            let bet_batch_pda = place_bet(
                &mollusk,
                &player,
                &player_pda,
                &global_state_pda,
                &treasury_pda,
                BET_PASS,
                10,
                None,
            );
            
            // Verify each bet placement succeeds
            assert_ne!(bet_batch_pda, Pubkey::default());
        }
        
        // Test dice roll performance
        let start_time = std::time::Instant::now();
        simulate_dice_roll(&mollusk, &global_state_pda, &rng_state_pda, &rng_authority);
        let duration = start_time.elapsed();
        
        // Ensure dice roll is reasonably fast (less than 1 second in test environment)
        assert!(duration.as_secs() < 1);
    }

    #[test]
    fn test_all_bet_types_encoding() {
        use craps_pinocchio::utils::bet_encoding::*;
        
        // Test all bet types can be encoded/decoded
        for bet_type in 0..64u8 {
            let amount = 100u64;
            if let Ok(encoded) = encode_bet(bet_type, amount) {
                let (decoded_type, decoded_amount) = decode_bet(encoded);
                assert_eq!(bet_type, decoded_type);
                assert_eq!(amount, decoded_amount);
            }
        }
    }

    #[test]
    fn test_account_size_constraints() {
        // Verify account sizes match expected values
        assert_eq!(GlobalGameState::LEN, 216);
        assert_eq!(ScalablePlayerState::LEN, 144);
        assert_eq!(BetBatch::LEN, 336);
        assert_eq!(BonusState::LEN, 40);
        assert_eq!(Treasury::LEN, 144);
        assert_eq!(RngState::LEN, 368);
        
        // Verify all structures are Pod-compatible
        use std::mem::size_of;
        assert_eq!(size_of::<GlobalGameState>(), GlobalGameState::LEN);
        assert_eq!(size_of::<ScalablePlayerState>(), ScalablePlayerState::LEN);
        assert_eq!(size_of::<BetBatch>(), BetBatch::LEN);
        assert_eq!(size_of::<BonusState>(), BonusState::LEN);
        assert_eq!(size_of::<Treasury>(), Treasury::LEN);
        assert_eq!(size_of::<RngState>(), RngState::LEN);
    }
}