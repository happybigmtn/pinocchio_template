use shank::ShankInstruction;

// Re-export instruction handlers
pub mod authority;
pub mod betting;
pub mod claim;
pub mod cleanup;
pub mod emergency;
pub mod game;
pub mod initialize;
pub mod player;
pub mod rng;
pub mod settlement;
pub mod tournament;
pub mod treasury;
pub mod treasury_admin;

// Re-export all handler functions
pub use authority::*;
pub use betting::*;
pub use claim::*;
pub use cleanup::*;
pub use emergency::*;
pub use game::*;
pub use initialize::*;
pub use player::*;
pub use rng::*;
pub use settlement::*;
pub use tournament::*;
pub use treasury::*;
pub use treasury_admin::*;

/// Authority type for update operations
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum AuthorityType {
    System = 0,
    Rng = 1,
    Admin = 2,
    Emergency = 3,
    Treasury = 4,
}

/// Emergency operation type
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum EmergencyOperation {
    Shutdown = 0,
    Resume = 1,
    PauseGame = 2,
    ResumeGame = 3,
}

/// All instructions supported by the craps-pinocchio program (consolidated)
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

    // ===== Treasury Instructions (Consolidated) =====
    /// Deposit tokens to the treasury (with optional auto-claim)
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(3, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "token_program", desc = "Token program")]
    Deposit = 4,

    /// Withdraw tokens from the treasury (with optional auto-claim)
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, writable, name = "treasury_token_account", desc = "Treasury's token account")]
    #[account(3, writable, name = "player_token_account", desc = "Player's token account")]
    #[account(4, signer, name = "player", desc = "Player account")]
    #[account(5, name = "treasury_authority", desc = "Treasury authority")]
    #[account(6, name = "token_program", desc = "Token program")]
    Withdraw = 5,

    // ===== Betting Instructions =====
    /// Place a bet for the current epoch
    #[account(0, writable, name = "bet_batch", desc = "Bet batch account")]
    #[account(1, writable, name = "player_state", desc = "Player state account")]
    #[account(2, name = "global_game_state", desc = "Global game state account")]
    #[account(3, signer, name = "player", desc = "Player account")]
    #[account(4, name = "system_program", desc = "System program")]
    PlaceBet = 6,

    // ===== Game Instructions =====
    /// Perform secure automatic dice roll
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "bonus_state", desc = "Bonus state account")]
    #[account(2, writable, name = "rng_state", desc = "RNG state account")]
    #[account(3, writable, name = "treasury", desc = "Treasury account")]
    #[account(4, signer, name = "rng_authority", desc = "RNG authority")]
    SecureAutoRoll = 7,

    /// Collect block hash for RNG
    #[account(0, writable, name = "rng_state", desc = "RNG state account")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    CollectBlockHash = 8,

    /// Finalize RNG and prepare for betting
    #[account(0, writable, name = "rng_state", desc = "RNG state account")]
    #[account(1, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    FinalizeRng = 9,

    /// Start betting phase after RNG finalization
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, name = "rng_state", desc = "RNG state account")]
    #[account(2, signer, name = "rng_authority", desc = "RNG authority")]
    StartBettingPhase = 10,

    // ===== Settlement Instructions =====
    /// Settle realizable bets for multiple epochs
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, writable, name = "treasury", desc = "Treasury account")]
    #[account(2, signer, name = "authority", desc = "Authority account")]
    SettleRealizableBets = 11,

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
    ClaimEpochPayoutsUnified = 12,

    // ===== Cleanup Instructions =====
    /// Clean up bet batch for an epoch
    #[account(0, writable, name = "bet_batch", desc = "Bet batch account to clean")]
    #[account(1, name = "player_state", desc = "Player state account")]
    #[account(2, signer, name = "player", desc = "Player account")]
    CleanupBetBatch = 13,

    /// Clean up old epoch outcome
    #[account(0, writable, name = "epoch_outcome", desc = "Epoch outcome to clean")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "admin", desc = "Admin authority")]
    CleanupOldEpochOutcome = 14,

    // ===== Authority Instructions (Consolidated) =====
    /// Update any authority type
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "current_authority", desc = "Current authority")]
    UpdateAuthority = 15,

    /// Execute pending authority transfer
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "new_authority", desc = "New authority")]
    ExecuteAuthorityTransfer = 16,

    // ===== Emergency Instructions (Consolidated) =====
    /// Emergency operations (shutdown, resume, pause game, resume game)
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    EmergencyOperation = 17,

    // ===== RNG Instructions =====
    /// Enable secure RNG mode
    #[account(0, writable, name = "global_game_state", desc = "Global game state account")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    EnableSecureRng = 18,

    // ===== Tournament Instructions =====
    /// Update player's tournament status
    #[account(0, writable, name = "player_state", desc = "Player state account")]
    #[account(1, signer, name = "tournament_program", desc = "Tournament program")]
    UpdatePlayerTournament = 19,

    /// Clear player's tournament status
    #[account(0, writable, name = "player_state", desc = "Player state account")]
    #[account(1, signer, name = "tournament_program", desc = "Tournament program")]
    ClearPlayerTournament = 20,

    // ===== Treasury Admin Instructions =====
    /// Update treasury parameters
    #[account(0, writable, name = "treasury", desc = "Treasury account")]
    #[account(1, name = "global_game_state", desc = "Global game state account")]
    #[account(2, signer, name = "admin", desc = "Admin authority")]
    UpdateTreasuryParameters = 21,
}

impl TryFrom<&u8> for CrapsInstruction {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match *value {
            0 => Ok(CrapsInstruction::InitializeSystem),
            1 => Ok(CrapsInstruction::InitializeCriticalPDAs),
            2 => Ok(CrapsInstruction::InitializePlayer),
            3 => Ok(CrapsInstruction::ClosePlayerAccount),
            4 => Ok(CrapsInstruction::Deposit),
            5 => Ok(CrapsInstruction::Withdraw),
            6 => Ok(CrapsInstruction::PlaceBet),
            7 => Ok(CrapsInstruction::SecureAutoRoll),
            8 => Ok(CrapsInstruction::CollectBlockHash),
            9 => Ok(CrapsInstruction::FinalizeRng),
            10 => Ok(CrapsInstruction::StartBettingPhase),
            11 => Ok(CrapsInstruction::SettleRealizableBets),
            12 => Ok(CrapsInstruction::ClaimEpochPayoutsUnified),
            13 => Ok(CrapsInstruction::CleanupBetBatch),
            14 => Ok(CrapsInstruction::CleanupOldEpochOutcome),
            15 => Ok(CrapsInstruction::UpdateAuthority),
            16 => Ok(CrapsInstruction::ExecuteAuthorityTransfer),
            17 => Ok(CrapsInstruction::EmergencyOperation),
            18 => Ok(CrapsInstruction::EnableSecureRng),
            19 => Ok(CrapsInstruction::UpdatePlayerTournament),
            20 => Ok(CrapsInstruction::ClearPlayerTournament),
            21 => Ok(CrapsInstruction::UpdateTreasuryParameters),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}

impl TryFrom<&[u8]> for AuthorityType {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(pinocchio::program_error::ProgramError::InvalidInstructionData);
        }
        match value[0] {
            0 => Ok(AuthorityType::System),
            1 => Ok(AuthorityType::Rng),
            2 => Ok(AuthorityType::Admin),
            3 => Ok(AuthorityType::Emergency),
            4 => Ok(AuthorityType::Treasury),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}

impl TryFrom<&[u8]> for EmergencyOperation {
    type Error = pinocchio::program_error::ProgramError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(pinocchio::program_error::ProgramError::InvalidInstructionData);
        }
        match value[0] {
            0 => Ok(EmergencyOperation::Shutdown),
            1 => Ok(EmergencyOperation::Resume),
            2 => Ok(EmergencyOperation::PauseGame),
            3 => Ok(EmergencyOperation::ResumeGame),
            _ => Err(pinocchio::program_error::ProgramError::InvalidInstructionData),
        }
    }
}