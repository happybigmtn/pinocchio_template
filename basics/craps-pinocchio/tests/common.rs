use pinocchio::{
    account_info::AccountInfo,
    pubkey::Pubkey,
    program_error::ProgramError,
    entrypoint::ProgramResult,
    pubkey,
    sysvars::clock::Clock,
    program::invoke_signed,
    system_program,
};
use mollusk_svm::{Mollusk, result::Check};
use solana_sdk::{
    account::Account as SolanaAccount,
    instruction::{AccountMeta, Instruction},
    signature::Keypair,
    signer::Signer,
    transaction::Transaction,
};
use bytemuck::{Pod, Zeroable};

// Import our program modules
use craps_pinocchio::*;

pub const PROGRAM_ID: Pubkey = craps_pinocchio::ID;

/// Test helper to create a mollusk instance
pub fn create_mollusk() -> Mollusk {
    Mollusk::new(&PROGRAM_ID, "target/deploy/craps_pinocchio")
}

/// Test helper to create test keypairs
pub fn create_test_keypairs() -> (Keypair, Keypair, Keypair) {
    (
        Keypair::new(), // admin
        Keypair::new(), // player
        Keypair::new(), // rng_authority
    )
}

/// Test helper to derive PDAs
pub fn derive_pdas() -> (Pubkey, Pubkey, Pubkey) {
    let (global_state_pda, _) = pubkey::find_program_address(
        &[b"global_game_state"],
        &PROGRAM_ID
    );
    
    let (treasury_pda, _) = pubkey::find_program_address(
        &[b"treasury"],
        &PROGRAM_ID
    );
    
    let (rng_state_pda, _) = pubkey::find_program_address(
        &[b"rng_state"],
        &PROGRAM_ID
    );
    
    (global_state_pda, treasury_pda, rng_state_pda)
}

/// Test helper to create player PDA
pub fn derive_player_pda(player: &Pubkey) -> (Pubkey, u8) {
    pubkey::find_program_address(
        &[b"player", player.as_ref()],
        &PROGRAM_ID
    )
}

/// Test helper to create bet batch PDA
pub fn derive_bet_batch_pda(player: &Pubkey, epoch: u64) -> (Pubkey, u8) {
    pubkey::find_program_address(
        &[b"bet_batch", player.as_ref(), &epoch.to_le_bytes()],
        &PROGRAM_ID
    )
}

/// Test helper to create instruction data
pub fn create_instruction_data(discriminant: u8, data: &[u8]) -> Vec<u8> {
    let mut instruction_data = vec![discriminant];
    instruction_data.extend_from_slice(data);
    instruction_data
}

/// Test helper to check account data
pub fn check_account_data<T: Pod>(account_data: &[u8]) -> &T {
    bytemuck::from_bytes::<T>(account_data)
}

/// Test helper to check mutable account data
pub fn check_account_data_mut<T: Pod>(account_data: &mut [u8]) -> &mut T {
    bytemuck::from_bytes_mut::<T>(account_data)
}

/// Test helper to create a basic initialized game state
pub fn setup_basic_game_state(mollusk: &Mollusk) -> (Keypair, Keypair, Keypair, Pubkey, Pubkey, Pubkey) {
    let (admin, player, rng_authority) = create_test_keypairs();
    let (global_state_pda, treasury_pda, rng_state_pda) = derive_pdas();
    
    // Initialize system instruction
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
    
    // Execute initialization
    mollusk.process_and_validate_instruction(
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
                .build(),
        ],
    );
    
    (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda)
}

/// Test helper to create a player account
pub fn create_player_account(
    mollusk: &Mollusk,
    player: &Keypair,
    global_state_pda: &Pubkey,
    treasury_pda: &Pubkey,
    rng_state_pda: &Pubkey,
) -> Pubkey {
    let (player_pda, bump) = derive_player_pda(&player.pubkey());
    
    let init_player_data = create_instruction_data(
        CrapsInstruction::InitializePlayer as u8, 
        &[bump]
    );
    
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &init_player_data,
        vec![
            AccountMeta::new(player.pubkey(), true),
            AccountMeta::new(player_pda, false),
            AccountMeta::new_readonly(*global_state_pda, false),
            AccountMeta::new_readonly(*treasury_pda, false),
            AccountMeta::new_readonly(*rng_state_pda, false),
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
            (*global_state_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                GlobalGameState::LEN,
                &PROGRAM_ID,
            )),
            (*treasury_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                Treasury::LEN,
                &PROGRAM_ID,
            )),
            (*rng_state_pda, SolanaAccount::new_ref(
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
                .build(),
        ],
    );
    
    player_pda
}

/// Test helper to place a bet
pub fn place_bet(
    mollusk: &Mollusk,
    player: &Keypair,
    player_pda: &Pubkey,
    global_state_pda: &Pubkey,
    treasury_pda: &Pubkey,
    bet_type: u8,
    amount: u64,
    repeater_target: Option<u8>,
) -> Pubkey {
    let current_epoch = 1u64; // For testing, use epoch 1
    let (bet_batch_pda, _) = derive_bet_batch_pda(&player.pubkey(), current_epoch);
    
    let mut bet_data = vec![bet_type];
    bet_data.extend_from_slice(&amount.to_le_bytes());
    bet_data.push(repeater_target.unwrap_or(0));
    
    let place_bet_data = create_instruction_data(
        CrapsInstruction::PlaceBet as u8,
        &bet_data,
    );
    
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &place_bet_data,
        vec![
            AccountMeta::new(*player_pda, false),
            AccountMeta::new(bet_batch_pda, false),
            AccountMeta::new(*global_state_pda, false),
            AccountMeta::new_readonly(*treasury_pda, false),
            AccountMeta::new_readonly(pinocchio::sysvars::clock::ID, false),
        ],
    );
    
    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (*player_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(ScalablePlayerState::LEN),
                ScalablePlayerState::LEN,
                &PROGRAM_ID,
            )),
            (bet_batch_pda, SolanaAccount::new(
                mollusk.sysvars.rent.minimum_balance(BetBatch::LEN),
                BetBatch::LEN,
                &PROGRAM_ID,
            )),
            (*global_state_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                GlobalGameState::LEN,
                &PROGRAM_ID,
            )),
            (*treasury_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                Treasury::LEN,
                &PROGRAM_ID,
            )),
        ],
        &[
            Check::success(),
            Check::account(&bet_batch_pda)
                .owner(&PROGRAM_ID)
                .build(),
        ],
    );
    
    bet_batch_pda
}

/// Test helper to simulate dice roll
pub fn simulate_dice_roll(
    mollusk: &Mollusk,
    global_state_pda: &Pubkey,
    rng_state_pda: &Pubkey,
    rng_authority: &Keypair,
) {
    let auto_roll_data = create_instruction_data(CrapsInstruction::SecureAutoRoll as u8, &[]);
    
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &auto_roll_data,
        vec![
            AccountMeta::new(*global_state_pda, false),
            AccountMeta::new_readonly(*rng_state_pda, false),
            AccountMeta::new_readonly(rng_authority.pubkey(), true),
        ],
    );
    
    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (*global_state_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                GlobalGameState::LEN,
                &PROGRAM_ID,
            )),
            (*rng_state_pda, SolanaAccount::new_ref(
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
            Check::account(global_state_pda)
                .data(|data| {
                    let state = check_account_data::<GlobalGameState>(data);
                    state.dice1 >= 1 && state.dice1 <= 6 &&
                    state.dice2 >= 1 && state.dice2 <= 6
                })
                .build(),
        ],
    );
}

/// Test helper to claim winnings
pub fn claim_winnings(
    mollusk: &Mollusk,
    player: &Keypair,
    player_pda: &Pubkey,
    bet_batch_pda: &Pubkey,
    global_state_pda: &Pubkey,
    treasury_pda: &Pubkey,
    epoch: u64,
) {
    let mut claim_data = vec![];
    claim_data.extend_from_slice(&epoch.to_le_bytes());
    
    let claim_instruction_data = create_instruction_data(
        CrapsInstruction::ClaimEpochPayoutsUnified as u8,
        &claim_data,
    );
    
    let instruction = Instruction::new_with_bytes(
        PROGRAM_ID,
        &claim_instruction_data,
        vec![
            AccountMeta::new(*player_pda, false),
            AccountMeta::new(*bet_batch_pda, false),
            AccountMeta::new_readonly(*global_state_pda, false),
            AccountMeta::new(*treasury_pda, false),
            AccountMeta::new_readonly(player.pubkey(), true),
        ],
    );
    
    mollusk.process_and_validate_instruction(
        &instruction,
        &vec![
            (*player_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(ScalablePlayerState::LEN),
                ScalablePlayerState::LEN,
                &PROGRAM_ID,
            )),
            (*bet_batch_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(BetBatch::LEN),
                BetBatch::LEN,
                &PROGRAM_ID,
            )),
            (*global_state_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(GlobalGameState::LEN),
                GlobalGameState::LEN,
                &PROGRAM_ID,
            )),
            (*treasury_pda, SolanaAccount::new_ref(
                mollusk.sysvars.rent.minimum_balance(Treasury::LEN),
                Treasury::LEN,
                &PROGRAM_ID,
            )),
            (player.pubkey(), SolanaAccount::new_ref(
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

/// Test helper to create token accounts for testing
pub fn create_token_accounts(
    mollusk: &Mollusk,
    player: &Keypair,
    mint: &Pubkey,
) -> (Pubkey, Pubkey) {
    // For testing, we'll create simple PDAs
    let (player_token_pda, _) = pubkey::find_program_address(
        &[b"player_token", player.pubkey().as_ref()],
        &PROGRAM_ID
    );
    
    let (treasury_token_pda, _) = pubkey::find_program_address(
        &[b"treasury_token"],
        &PROGRAM_ID
    );
    
    (player_token_pda, treasury_token_pda)
}

/// Test helper to validate account ownership
pub fn validate_account_ownership(account: &SolanaAccount, expected_owner: &Pubkey) -> bool {
    account.owner == *expected_owner
}

/// Test helper to validate account data length
pub fn validate_account_data_length(account: &SolanaAccount, expected_len: usize) -> bool {
    account.data.len() == expected_len
}

/// Test helper to create mock sysvars
pub fn create_mock_sysvars() -> Vec<(Pubkey, SolanaAccount)> {
    vec![
        (
            pinocchio::sysvars::clock::ID,
            SolanaAccount::new_ref(
                0,
                std::mem::size_of::<pinocchio::sysvars::clock::Clock>(),
                &pinocchio::sysvars::ID,
            ),
        ),
        (
            pinocchio::sysvars::rent::ID,
            SolanaAccount::new_ref(
                0,
                std::mem::size_of::<pinocchio::sysvars::rent::Rent>(),
                &pinocchio::sysvars::ID,
            ),
        ),
    ]
}

/// Test helper to simulate time passage
pub fn advance_time(mollusk: &mut Mollusk, seconds: u64) {
    // In a real implementation, this would advance the clock
    // For testing purposes, we'll just note the intention
    // mollusk.advance_clock(seconds);
}

/// Test helper to create random entropy for testing
pub fn create_test_entropy() -> [u8; 32] {
    let mut entropy = [0u8; 32];
    // Fill with predictable test data
    for i in 0..32 {
        entropy[i] = (i as u8).wrapping_mul(17).wrapping_add(42);
    }
    entropy
}

/// Test helper to verify bet resolution
pub fn verify_bet_resolution(
    bet_batch_data: &[u8],
    bet_index: usize,
    expected_resolved: bool,
    expected_realizable: bool,
) {
    let batch = check_account_data::<BetBatch>(bet_batch_data);
    
    if expected_resolved {
        assert!(batch.is_resolved(bet_index), "Bet {} should be resolved", bet_index);
    } else {
        assert!(!batch.is_resolved(bet_index), "Bet {} should not be resolved", bet_index);
    }
    
    if expected_realizable {
        assert!(batch.is_realizable(bet_index), "Bet {} should be realizable", bet_index);
    } else {
        assert!(!batch.is_realizable(bet_index), "Bet {} should not be realizable", bet_index);
    }
}

/// Test helper to create a full game scenario
pub fn create_full_game_scenario(
    mollusk: &Mollusk,
) -> (Keypair, Keypair, Keypair, Pubkey, Pubkey, Pubkey, Pubkey) {
    let (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda) = 
        setup_basic_game_state(mollusk);
    
    let player_pda = create_player_account(
        mollusk,
        &player,
        &global_state_pda,
        &treasury_pda,
        &rng_state_pda,
    );
    
    (admin, player, rng_authority, global_state_pda, treasury_pda, rng_state_pda, player_pda)
}