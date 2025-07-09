#![cfg(test)]

use mollusk_svm::{Mollusk, Instruction};
use solana_sdk::{
    account::AccountSharedData,
    instruction::{AccountMeta, Instruction as SolanaInstruction},
    pubkey::Pubkey,
    rent::Rent,
    signature::{Keypair, Signer},
    system_program,
};
use craps_pinocchio::{
    ID as PROGRAM_ID,
    instructions::{
        CrapsInstruction,
        InitializeSystemData,
        PlaceBetData,
    },
    state::{
        GlobalGameState, Treasury, RngState, BonusState, ScalablePlayerState, BetBatch,
        RngPhase, MAX_BETS_PER_BATCH,
    },
    constants::*,
    utils::bet_encoding::encode_bet,
};

/// Test fixture for craps program
struct CrapsTestFixture {
    mollusk: Mollusk,
    authority: Keypair,
    player: Keypair,
    global_game_state: Pubkey,
    treasury: Pubkey,
    bonus_state: Pubkey,
    rng_state: Pubkey,
}

impl CrapsTestFixture {
    fn new() -> Self {
        let mut mollusk = Mollusk::new(&PROGRAM_ID, "craps_pinocchio");
        
        // Enable features
        mollusk.sysvars.rent = Some(Rent::default());
        mollusk.sysvars.clock = Some(Default::default());
        
        let authority = Keypair::new();
        let player = Keypair::new();
        
        // Derive PDAs
        let (global_game_state, _) = Pubkey::find_program_address(&[GLOBAL_GAME_STATE_SEED], &PROGRAM_ID);
        let (treasury, _) = Pubkey::find_program_address(&[TREASURY_SEED], &PROGRAM_ID);
        let (bonus_state, _) = Pubkey::find_program_address(&[BONUS_STATE_SEED], &PROGRAM_ID);
        let (rng_state, _) = Pubkey::find_program_address(&[RNG_STATE_SEED], &PROGRAM_ID);
        
        Self {
            mollusk,
            authority,
            player,
            global_game_state,
            treasury,
            bonus_state,
            rng_state,
        }
    }
    
    fn initialize_system(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let rng_authority = Keypair::new();
        let data = InitializeSystemData {
            rng_authority: rng_authority.pubkey().to_bytes(),
            _padding: [0; 16],
        };
        
        let instruction = SolanaInstruction::new_with_bytes(
            PROGRAM_ID,
            &[CrapsInstruction::InitializeSystem as u8]
                .iter()
                .chain(&bytemuck::bytes_of(&data))
                .copied()
                .collect::<Vec<u8>>(),
            vec![
                AccountMeta::new(self.global_game_state, false),
                AccountMeta::new(self.treasury, false),
                AccountMeta::new(self.bonus_state, false),
                AccountMeta::new(self.rng_state, false),
                AccountMeta::new(self.authority.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );
        
        let result = self.mollusk.process_instruction(
            &instruction,
            &vec![
                (&self.global_game_state, AccountSharedData::new(0, 0, &system_program::id())),
                (&self.treasury, AccountSharedData::new(0, 0, &system_program::id())),
                (&self.bonus_state, AccountSharedData::new(0, 0, &system_program::id())),
                (&self.rng_state, AccountSharedData::new(0, 0, &system_program::id())),
                (&self.authority.pubkey(), AccountSharedData::new(1_000_000_000, 0, &system_program::id())),
                (&system_program::id(), mollusk_svm::program::create_program_account()),
            ],
        );
        
        result.result.map_err(|e| format!("Initialize system failed: {:?}", e).into())
    }
    
    fn initialize_player(&mut self) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let (player_state, _) = Pubkey::find_program_address(
            &[SCALABLE_PLAYER_SEED, self.player.pubkey().as_ref()],
            &PROGRAM_ID
        );
        
        let instruction = SolanaInstruction::new_with_bytes(
            PROGRAM_ID,
            &[CrapsInstruction::InitializePlayer as u8],
            vec![
                AccountMeta::new(player_state, false),
                AccountMeta::new(self.player.pubkey(), true),
                AccountMeta::new_readonly(self.global_game_state, false),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );
        
        let result = self.mollusk.process_instruction(
            &instruction,
            &vec![
                (&player_state, AccountSharedData::new(0, 0, &system_program::id())),
                (&self.player.pubkey(), AccountSharedData::new(1_000_000_000, 0, &system_program::id())),
                (&self.global_game_state, self.get_account(&self.global_game_state)?),
                (&system_program::id(), mollusk_svm::program::create_program_account()),
            ],
        );
        
        result.result.map_err(|e| format!("Initialize player failed: {:?}", e).into())?;
        Ok(player_state)
    }
    
    fn place_bet(&mut self, player_state: &Pubkey, bet_type: u8, amount: u64, epoch: u64) -> Result<Pubkey, Box<dyn std::error::Error>> {
        let batch_index = 0u32; // Simplified - would need to track actual index
        let (bet_batch, _) = Pubkey::find_program_address(
            &[
                BET_BATCH_SEED,
                self.player.pubkey().as_ref(),
                &epoch.to_le_bytes(),
                &batch_index.to_le_bytes(),
            ],
            &PROGRAM_ID
        );
        
        let data = PlaceBetData {
            epoch: epoch.to_le_bytes(),
            bet_kind: bet_type,
            _padding1: [0; 7],
            bet_amount: amount.to_le_bytes(),
            _padding2: [0; 8],
        };
        
        let instruction = SolanaInstruction::new_with_bytes(
            PROGRAM_ID,
            &[CrapsInstruction::PlaceBet as u8]
                .iter()
                .chain(&bytemuck::bytes_of(&data))
                .copied()
                .collect::<Vec<u8>>(),
            vec![
                AccountMeta::new(bet_batch, false),
                AccountMeta::new(*player_state, false),
                AccountMeta::new_readonly(self.global_game_state, false),
                AccountMeta::new(self.player.pubkey(), true),
                AccountMeta::new_readonly(system_program::id(), false),
            ],
        );
        
        let result = self.mollusk.process_instruction(
            &instruction,
            &vec![
                (&bet_batch, AccountSharedData::new(0, 0, &system_program::id())),
                (player_state, self.get_account(player_state)?),
                (&self.global_game_state, self.get_account(&self.global_game_state)?),
                (&self.player.pubkey(), AccountSharedData::new(1_000_000_000, 0, &system_program::id())),
                (&system_program::id(), mollusk_svm::program::create_program_account()),
            ],
        );
        
        result.result.map_err(|e| format!("Place bet failed: {:?}", e).into())?;
        Ok(bet_batch)
    }
    
    fn get_account(&self, pubkey: &Pubkey) -> Result<AccountSharedData, Box<dyn std::error::Error>> {
        self.mollusk
            .get_account(pubkey)
            .ok_or_else(|| format!("Account not found: {}", pubkey).into())
    }
}

#[test]
fn test_system_initialization() {
    let mut fixture = CrapsTestFixture::new();
    
    // Initialize the system
    fixture.initialize_system().expect("System initialization should succeed");
    
    // Verify global game state
    let game_state_account = fixture.get_account(&fixture.global_game_state).expect("Game state should exist");
    assert_eq!(game_state_account.owner, &PROGRAM_ID);
    
    let game_state_data = game_state_account.data();
    let game_state = bytemuck::from_bytes::<GlobalGameState>(&game_state_data[..GlobalGameState::LEN]);
    
    assert_eq!(game_state.get_current_epoch(), 0);
    assert_eq!(game_state.get_current_phase(), PHASE_COME_OUT);
    assert!(!game_state.get_is_paused());
    
    // Verify treasury
    let treasury_account = fixture.get_account(&fixture.treasury).expect("Treasury should exist");
    assert_eq!(treasury_account.owner, &PROGRAM_ID);
    
    // Verify RNG state
    let rng_account = fixture.get_account(&fixture.rng_state).expect("RNG state should exist");
    assert_eq!(rng_account.owner, &PROGRAM_ID);
    
    let rng_data = rng_account.data();
    let rng_state = bytemuck::from_bytes::<RngState>(&rng_data[..RngState::LEN]);
    assert_eq!(rng_state.get_phase(), RngPhase::Betting);
}

#[test]
fn test_player_initialization() {
    let mut fixture = CrapsTestFixture::new();
    
    // Initialize system first
    fixture.initialize_system().expect("System initialization should succeed");
    
    // Initialize player
    let player_state = fixture.initialize_player().expect("Player initialization should succeed");
    
    // Verify player state
    let player_account = fixture.get_account(&player_state).expect("Player state should exist");
    assert_eq!(player_account.owner, &PROGRAM_ID);
    
    let player_data = player_account.data();
    let player_state_data = bytemuck::from_bytes::<ScalablePlayerState>(&player_data[..ScalablePlayerState::LEN]);
    
    assert_eq!(player_state_data.player, fixture.player.pubkey().to_bytes());
    assert_eq!(player_state_data.get_balance(), 0);
    assert_eq!(player_state_data.get_total_wagered(), 0);
}

#[test]
fn test_place_bet_flow() {
    let mut fixture = CrapsTestFixture::new();
    
    // Initialize system and player
    fixture.initialize_system().expect("System initialization should succeed");
    let player_state = fixture.initialize_player().expect("Player initialization should succeed");
    
    // Update player balance (simulate deposit)
    let mut player_account = fixture.get_account(&player_state).expect("Player state should exist");
    let player_data = player_account.data_as_mut_slice();
    let player_state_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_data[..ScalablePlayerState::LEN]);
    player_state_data.set_balance(1_000_000_000); // 1 CRAP token
    
    // Place a pass line bet
    let bet_amount = 100_000_000; // 0.1 CRAP
    let bet_batch = fixture.place_bet(&player_state, BET_PASS, bet_amount, 0)
        .expect("Place bet should succeed");
    
    // Verify bet batch
    let batch_account = fixture.get_account(&bet_batch).expect("Bet batch should exist");
    let batch_data = batch_account.data();
    let batch = bytemuck::from_bytes::<BetBatch>(&batch_data[..BetBatch::LEN]);
    
    assert_eq!(batch.bet_count, 1);
    assert_eq!(batch.get_epoch(), 0);
    
    // Decode and verify the bet
    let packed_bet = batch.get_packed_bet(0);
    let encoded_bet = encode_bet(BET_PASS, bet_amount).expect("Encoding should succeed");
    assert_eq!(packed_bet, encoded_bet);
}

#[test]
fn test_multiple_bets_in_batch() {
    let mut fixture = CrapsTestFixture::new();
    
    // Setup
    fixture.initialize_system().expect("System initialization should succeed");
    let player_state = fixture.initialize_player().expect("Player initialization should succeed");
    
    // Give player balance
    let mut player_account = fixture.get_account(&player_state).expect("Player state should exist");
    let player_data = player_account.data_as_mut_slice();
    let player_state_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_data[..ScalablePlayerState::LEN]);
    player_state_data.set_balance(10_000_000_000); // 10 CRAP tokens
    
    // Place multiple bets
    let bets = vec![
        (BET_PASS, 100_000_000),
        (BET_FIELD, 200_000_000),
        (BET_HARD6, 50_000_000),
        (BET_ANY_SEVEN, 25_000_000),
    ];
    
    for (i, (bet_type, amount)) in bets.iter().enumerate() {
        fixture.place_bet(&player_state, *bet_type, *amount, 0)
            .expect(&format!("Bet {} should succeed", i));
    }
    
    // Verify all bets are in the batch
    let batch_index = 0u32;
    let (bet_batch, _) = Pubkey::find_program_address(
        &[
            BET_BATCH_SEED,
            fixture.player.pubkey().as_ref(),
            &0u64.to_le_bytes(),
            &batch_index.to_le_bytes(),
        ],
        &PROGRAM_ID
    );
    
    let batch_account = fixture.get_account(&bet_batch).expect("Bet batch should exist");
    let batch_data = batch_account.data();
    let batch = bytemuck::from_bytes::<BetBatch>(&batch_data[..BetBatch::LEN]);
    
    assert_eq!(batch.bet_count, bets.len() as u8);
    
    // Verify each bet
    for (i, (bet_type, amount)) in bets.iter().enumerate() {
        let packed_bet = batch.get_packed_bet(i);
        let expected = encode_bet(*bet_type, *amount).expect("Encoding should succeed");
        assert_eq!(packed_bet, expected, "Bet {} should match", i);
    }
}

#[test]
fn test_bet_validation() {
    let mut fixture = CrapsTestFixture::new();
    
    // Setup
    fixture.initialize_system().expect("System initialization should succeed");
    let player_state = fixture.initialize_player().expect("Player initialization should succeed");
    
    // Try to place bet without balance - should fail
    let result = fixture.place_bet(&player_state, BET_PASS, 100_000_000, 0);
    assert!(result.is_err(), "Bet without balance should fail");
    
    // Try to place bet with invalid bet type
    let result = fixture.place_bet(&player_state, 255, 100_000_000, 0);
    assert!(result.is_err(), "Invalid bet type should fail");
    
    // Try to place bet with invalid amount (0)
    let result = fixture.place_bet(&player_state, BET_PASS, 0, 0);
    assert!(result.is_err(), "Zero amount bet should fail");
}

#[test]
fn test_repeater_bet_placement() {
    let mut fixture = CrapsTestFixture::new();
    
    // Setup
    fixture.initialize_system().expect("System initialization should succeed");
    let player_state = fixture.initialize_player().expect("Player initialization should succeed");
    
    // Give player balance
    let mut player_account = fixture.get_account(&player_state).expect("Player state should exist");
    let player_data = player_account.data_as_mut_slice();
    let player_state_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_data[..ScalablePlayerState::LEN]);
    player_state_data.set_balance(1_000_000_000);
    
    // Place all repeater bets
    let repeater_bets = vec![
        BET_REPEATER_2, BET_REPEATER_3, BET_REPEATER_4, BET_REPEATER_5,
        BET_REPEATER_6, BET_REPEATER_8, BET_REPEATER_9, BET_REPEATER_10,
        BET_REPEATER_11, BET_REPEATER_12,
    ];
    
    for bet_type in repeater_bets {
        let result = fixture.place_bet(&player_state, bet_type, 50_000_000, 0);
        assert!(result.is_ok(), "Repeater bet {} should succeed", bet_type);
    }
}

// Additional tests would include:
// - test_secure_auto_roll
// - test_collect_block_hash
// - test_finalize_rng
// - test_claim_epoch_payouts
// - test_deposit_withdraw
// - test_emergency_pause
// - test_authority_updates
// - test_tournament_flow
// - test_edge_cases