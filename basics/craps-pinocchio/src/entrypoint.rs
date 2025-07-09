use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};
use pinocchio_log::log;

use crate::instructions::CrapsInstruction;
use crate::processor::{create_pda::CreatePda, get_pda::GetPda};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id != &crate::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let (discriminator, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match CrapsInstruction::try_from(discriminator)? {
        // System Instructions
        CrapsInstruction::InitializeSystem => {
            log!("Instruction: InitializeSystem");
            crate::instructions::initialize_system_handler(accounts, data)
        }
        CrapsInstruction::InitializeCriticalPDAs => {
            log!("Instruction: InitializeCriticalPDAs");
            crate::instructions::initialize_critical_pdas_handler(accounts, data)
        }
        
        // Player Instructions
        CrapsInstruction::InitializePlayer => {
            log!("Instruction: InitializePlayer");
            crate::instructions::initialize_player_handler(accounts, data)
        }
        CrapsInstruction::ClosePlayerAccount => {
            log!("Instruction: ClosePlayerAccount");
            crate::instructions::close_player_account_handler(accounts, data)
        }
        
        // Treasury Instructions
        CrapsInstruction::DepositV2 => {
            log!("Instruction: DepositV2");
            crate::instructions::deposit_v2_handler(accounts, data)
        }
        CrapsInstruction::WithdrawV2 => {
            log!("Instruction: WithdrawV2");
            crate::instructions::withdraw_v2_handler(accounts, data)
        }
        CrapsInstruction::DepositWithAutoClaimV2 => {
            log!("Instruction: DepositWithAutoClaimV2");
            crate::instructions::deposit_with_auto_claim_v2_handler(accounts, data)
        }
        CrapsInstruction::WithdrawWithAutoClaimV2 => {
            log!("Instruction: WithdrawWithAutoClaimV2");
            crate::instructions::withdraw_with_auto_claim_v2_handler(accounts, data)
        }
        
        // Betting Instructions
        CrapsInstruction::PlaceBet => {
            log!("Instruction: PlaceBet");
            crate::instructions::place_bet_handler(accounts, data)
        }
        
        // Game Instructions
        CrapsInstruction::SecureAutoRoll => {
            log!("Instruction: SecureAutoRoll");
            crate::instructions::secure_auto_roll_handler(accounts, data)
        }
        CrapsInstruction::CollectBlockHash => {
            log!("Instruction: CollectBlockHash");
            crate::instructions::collect_block_hash_handler(accounts, data)
        }
        CrapsInstruction::FinalizeRng => {
            log!("Instruction: FinalizeRng");
            crate::instructions::finalize_rng_handler(accounts, data)
        }
        CrapsInstruction::StartBettingPhase => {
            log!("Instruction: StartBettingPhase");
            crate::instructions::start_betting_phase_handler(accounts, data)
        }
        
        // Settlement Instructions
        CrapsInstruction::SettleRealizableBets => {
            log!("Instruction: SettleRealizableBets");
            crate::instructions::settle_realizable_bets_handler(accounts, data)
        }
        CrapsInstruction::ClaimEpochPayoutsUnified => {
            log!("Instruction: ClaimEpochPayoutsUnified");
            crate::instructions::claim_epoch_payouts_unified_handler(accounts, data)
        }
        
        // Cleanup Instructions
        CrapsInstruction::CleanupBetBatch => {
            log!("Instruction: CleanupBetBatch");
            crate::instructions::cleanup_bet_batch_handler(accounts, data)
        }
        CrapsInstruction::CleanupOldBetBatch => {
            log!("Instruction: CleanupOldBetBatch");
            crate::instructions::cleanup_old_bet_batch_handler(accounts, data)
        }
        CrapsInstruction::CleanupOldEpochOutcome => {
            log!("Instruction: CleanupOldEpochOutcome");
            crate::instructions::cleanup_old_epoch_outcome_handler(accounts, data)
        }
        
        // Authority Instructions
        CrapsInstruction::UpdateAuthority => {
            log!("Instruction: UpdateAuthority");
            crate::instructions::update_authority_handler(accounts, data)
        }
        CrapsInstruction::UpdateRngAuthority => {
            log!("Instruction: UpdateRngAuthority");
            crate::instructions::update_rng_authority_handler(accounts, data)
        }
        CrapsInstruction::UpdateAdminAuthority => {
            log!("Instruction: UpdateAdminAuthority");
            crate::instructions::update_admin_authority_handler(accounts, data)
        }
        CrapsInstruction::UpdateEmergencyAuthority => {
            log!("Instruction: UpdateEmergencyAuthority");
            crate::instructions::update_emergency_authority_handler(accounts, data)
        }
        CrapsInstruction::ExecuteAuthorityTransfer => {
            log!("Instruction: ExecuteAuthorityTransfer");
            crate::instructions::execute_authority_transfer_handler(accounts, data)
        }
        
        // Emergency Instructions
        CrapsInstruction::EmergencyShutdown => {
            log!("Instruction: EmergencyShutdown");
            crate::instructions::emergency_shutdown_handler(accounts, data)
        }
        CrapsInstruction::ResumeOperations => {
            log!("Instruction: ResumeOperations");
            crate::instructions::resume_operations_handler(accounts, data)
        }
        CrapsInstruction::EmergencyPause => {
            log!("Instruction: EmergencyPause");
            crate::instructions::emergency_pause_handler(accounts, data)
        }
        CrapsInstruction::ResumeGame => {
            log!("Instruction: ResumeGame");
            crate::instructions::resume_game_handler(accounts, data)
        }
        
        // RNG Instructions
        CrapsInstruction::EnableSecureRng => {
            log!("Instruction: EnableSecureRng");
            crate::instructions::enable_secure_rng_handler(accounts, data)
        }
        
        // Tournament Instructions
        CrapsInstruction::UpdatePlayerTournament => {
            log!("Instruction: UpdatePlayerTournament");
            crate::instructions::update_player_tournament_handler(accounts, data)
        }
        CrapsInstruction::ClearPlayerTournament => {
            log!("Instruction: ClearPlayerTournament");
            crate::instructions::clear_player_tournament_handler(accounts, data)
        }
        
        // Treasury Admin Instructions
        CrapsInstruction::UpdateTreasuryAuthority => {
            log!("Instruction: UpdateTreasuryAuthority");
            crate::instructions::update_treasury_authority_handler(accounts, data)
        }
        CrapsInstruction::UpdateTreasuryParameters => {
            log!("Instruction: UpdateTreasuryParameters");
            crate::instructions::update_treasury_parameters_handler(accounts, data)
        }
        
        // Test Instructions (from template)
        CrapsInstruction::CreatePda => {
            log!("Instruction: CreatePda");
            CreatePda::try_from((accounts, data))?.handler()
        }
        CrapsInstruction::GetPda => {
            log!("Instruction: GetPda");
            GetPda::try_from(accounts)?.handler()
        }
    }
}
