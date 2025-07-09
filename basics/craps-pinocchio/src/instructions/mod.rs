use shank::ShankInstruction;

// Re-export instruction handlers
pub mod betting;
pub mod claim;
pub mod game;
pub mod initialize;
pub mod player;
pub mod treasury;
pub mod stubs;

// Re-export all handler functions
pub use betting::*;
pub use claim::*;
pub use game::*;
pub use initialize::*;
pub use player::*;
pub use treasury::*;
pub use stubs::*;

/// All instructions supported by the craps-pinocchio program
#[repr(u8)]
#[derive(ShankInstruction, Debug, PartialEq)]
pub enum CrapsInstruction {
    // ===== System Instructions =====
    /// Initialize the global game state
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "treasury", desc = "Treasury account")]
    #[account(2, writable, name = "bonus_state", desc = "Bonus state account")]
    #[account(3, writable, name = "rng_state", desc = "RNG state account")]
    #[account(4, signer, name = "authority", desc = "System authority")]
    #[account(5, name = "system_program", desc = "System program")]
    InitializeSystem = 0,

    /// Initialize critical PDAs for the system
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "treasury", desc = "Treasury account")]
    #[account(2, writable, name = "rng_state", desc = "RNG state account")]
    #[account(3, signer, name = "authority", desc = "System authority")]
    InitializeCriticalPDAs = 1,

    // ===== Player Instructions =====
    /// Initialize a new player account
    #[account(0, writable, name = "player_state", desc = "Player state account")]
    #[account(1, signer, writable, name = "player", desc = "Player account")]
    #[account(2, name = "global_game_state", desc = "Global game state account")]
    #[account(3, name = "system_program", desc = "System program")]
    InitializePlayer = 2,

    /// Close a player account and return rent
    #[account(0, writable, name = "player_state", desc = "Player state account to close")]
    #[account(1, signer, name = "player", desc = "Player account")]
    #[account(2, writable, name = "receiver", desc = "Account to receive rent")]
    ClosePlayerAccount = 3,

    // ===== Treasury Instructions =====
    /// Deposit tokens to the treasury (v2)
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(3, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "token_program", desc = "Token program")]
    DepositV2 = 4,

    /// Withdraw tokens from the treasury (v2)
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(3, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "treasury_authority", desc = "Treasury authority")]
    #[account(6, name = "token_program", desc = "Token program")]
    WithdrawV2 = 5,

    /// Deposit with auto-claim of pending payouts
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(3, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "token_program", desc = "Token program")]
    DepositWithAutoClaimV2 = 6,

    /// Withdraw with auto-claim of pending payouts
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(3, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "treasury_authority", desc = "Treasury authority")]
    #[account(6, name = "token_program", desc = "Token program")]
    WithdrawWithAutoClaimV2 = 7,

    // ===== Betting Instructions =====
    /// Place a bet for the current epoch
    #[account(0, writable, name = "bet_batch", desc = "Bet batch account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, name = "global_game_state", desc = "Global game state account")]
    #[account(3, signer, name = "player", desc = "Player account")]
    #[account(4, name = "system_program", desc = "System program")]
    PlaceBet = 8,

    // ===== Game Instructions =====
    /// Perform secure automatic dice roll
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "bonus_state", desc = "Bonus state account")]
    #[account(2, writable, name = "rng_state", desc = "RNG state account")]
    #[account(3, writable, name = "treasury", desc = "Treasury account")]
    #[account(4, signer, name = "rng_authority", desc = "RNG authority")]
    SecureAutoRoll = 9,

    /// Collect block hash for RNG
    #[account(0, writable, name = "rng_state", desc = "RNG state account")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    CollectBlockHash = 10,

    /// Finalize RNG and prepare for betting
    #[account(0, writable, name = "rng_state", desc = "RNG state account")]
    #[account(1, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    FinalizeRng = 11,

    /// Start betting phase after RNG finalization
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, name = "rng_state", desc = "RNG state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    StartBettingPhase = 12,

    // ===== Settlement Instructions =====
    /// Settle realizable bets for multiple epochs
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "treasury", desc = "Treasury account")]
    #[account(2, signer, name = "authority", desc = "Authority account")]
    SettleRealizableBets = 13,

    /// Claim epoch payouts with unified logic
    #[account(0, signer, name = "player", desc = "Player account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "treasury", desc = "Treasury account")]
    #[account(3, writable, name = "treasury_token_account", desc = "Treasury token account")]
    #[account(4, writable, name = "player_token_account", desc = "Player token account")]
    #[account(5, name = "epoch_outcome", desc = "Epoch outcome account")]
    #[account(6, writable, name = "bet_batch", desc = "Bet batch account")]
    #[account(7, name = "bonus_state", desc = "Bonus state account")]
    #[account(8, name = "token_program", desc = "Token program")]
    #[account(9, name = "mint", desc = "Token mint")]
    ClaimEpochPayoutsUnified = 14,

    // ===== Cleanup Instructions =====
    /// Clean up bet batch for an epoch
    #[account(0, writable, name = "bet_batch", desc = "Bet batch account to clean")]
    #[account(1, name = "player_state", desc = "Player state account")]
    #[account(2, signer, name = "player", desc = "Player account")]
    CleanupBetBatch = 15,

    /// Clean up old bet batch (admin)
    #[account(0, writable, name = "bet_batch", desc = "Bet batch account to clean")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "admin", desc = "Admin authority")]
    CleanupOldBetBatch = 16,

    /// Clean up old epoch outcome
    #[account(0, writable, name = "epoch_outcome", desc = "Epoch outcome to clean")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "admin", desc = "Admin authority")]
    CleanupOldEpochOutcome = 17,

    // ===== Authority Instructions =====
    /// Update system authority
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateAuthority = 18,

    /// Update RNG authority
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateRngAuthority = 19,

    /// Update admin authority
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateAdminAuthority = 20,

    /// Update emergency authority
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateEmergencyAuthority = 21,

    /// Execute pending authority transfer
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "new_authority", desc = "New authority")]
    ExecuteAuthorityTransfer = 22,

    // ===== Emergency Instructions =====
    /// Emergency shutdown of the system
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "emergency_authority", desc = "Emergency authority")]
    EmergencyShutdown = 23,

    /// Resume operations after shutdown
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "emergency_authority", desc = "Emergency authority")]
    ResumeOperations = 24,

    /// Emergency pause the game
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "emergency_authority", desc = "Emergency authority")]
    EmergencyPause = 25,

    /// Resume game after pause
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    ResumeGame = 26,

    // ===== RNG Instructions =====
    /// Enable secure RNG mode
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    EnableSecureRng = 27,

    // ===== Tournament Instructions =====
    /// Update player's tournament status
    #[account(0, writable, name = "player_state", desc = "Player state account")]
    #[account(1, signer, name = "tournament_program", desc = "Tournament program")]
    UpdatePlayerTournament = 28,

    /// Clear player's tournament status
    #[account(0, writable, name = "player_state", desc = "Player state account")]
    #[account(1, signer, name = "tournament_program", desc = "Tournament program")]
    ClearPlayerTournament = 29,

    // ===== Treasury Admin Instructions =====
    /// Update treasury authority
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateTreasuryAuthority = 30,

    /// Update treasury parameters
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "admin", desc = "Admin authority")]
    UpdateTreasuryParameters = 31,

    // ===== Test Instructions (from template) =====
    /// Create a PDA (test instruction)
    CreatePda = 32,

    /// Get a PDA (test instruction)
    GetPda = 33,
}

impl TryFrom<&u8> for CrapsInstruction {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CrapsInstruction::InitializeSystem),
            1 => Ok(CrapsInstruction::InitializeCriticalPDAs),
            2 => Ok(CrapsInstruction::InitializePlayer),
            3 => Ok(CrapsInstruction::ClosePlayerAccount),
            4 => Ok(CrapsInstruction::DepositV2),
            5 => Ok(CrapsInstruction::WithdrawV2),
            6 => Ok(CrapsInstruction::DepositWithAutoClaimV2),
            7 => Ok(CrapsInstruction::WithdrawWithAutoClaimV2),
            8 => Ok(CrapsInstruction::PlaceBet),
            9 => Ok(CrapsInstruction::SecureAutoRoll),
            10 => Ok(CrapsInstruction::CollectBlockHash),
            11 => Ok(CrapsInstruction::FinalizeRng),
            12 => Ok(CrapsInstruction::StartBettingPhase),
            13 => Ok(CrapsInstruction::SettleRealizableBets),
            14 => Ok(CrapsInstruction::ClaimEpochPayoutsUnified),
            15 => Ok(CrapsInstruction::CleanupBetBatch),
            16 => Ok(CrapsInstruction::CleanupOldBetBatch),
            17 => Ok(CrapsInstruction::CleanupOldEpochOutcome),
            18 => Ok(CrapsInstruction::UpdateAuthority),
            19 => Ok(CrapsInstruction::UpdateRngAuthority),
            20 => Ok(CrapsInstruction::UpdateAdminAuthority),
            21 => Ok(CrapsInstruction::UpdateEmergencyAuthority),
            22 => Ok(CrapsInstruction::ExecuteAuthorityTransfer),
            23 => Ok(CrapsInstruction::EmergencyShutdown),
            24 => Ok(CrapsInstruction::ResumeOperations),
            25 => Ok(CrapsInstruction::EmergencyPause),
            26 => Ok(CrapsInstruction::ResumeGame),
            27 => Ok(CrapsInstruction::EnableSecureRng),
            28 => Ok(CrapsInstruction::UpdatePlayerTournament),
            29 => Ok(CrapsInstruction::ClearPlayerTournament),
            30 => Ok(CrapsInstruction::UpdateTreasuryAuthority),
            31 => Ok(CrapsInstruction::UpdateTreasuryParameters),
            32 => Ok(CrapsInstruction::CreatePda),
            33 => Ok(CrapsInstruction::GetPda),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}