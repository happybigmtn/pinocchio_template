use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{self},
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};
use pinocchio_log::log;
use bytemuck::{Pod, Zeroable};

use crate::{
    constants::*,
    error::CrapsError,
    state::{Treasury, ScalablePlayerState, GlobalGameState},
    utils::{
        validation::validate_deposit_amount,
        token::{validate_token_account, get_token_balance, transfer_tokens},
        circuit_breaker::{validate_treasury_operation, TreasuryOperation},
    },
};

/// Instruction data for deposit/withdraw operations
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AmountData {
    /// The amount to deposit/withdraw
    pub amount: [u8; 8],
}

/// Handler for DepositV2 instruction
pub fn deposit_v2_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 8 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [treasury, player_state, player_token_account, treasury_token_account, player, token_program, mint, global_game_state] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate player is signer
    if !player.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate token program
    if token_program.key() != &pinocchio_token::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse instruction data
    if data.len() < core::mem::size_of::<AmountData>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount_data = bytemuck::from_bytes::<AmountData>(
        &data[..core::mem::size_of::<AmountData>()]
    );
    let amount = u64::from_le_bytes(amount_data.amount);

    // Validate amount
    validate_deposit_amount(amount)?;

    // Load global game state for circuit breaker
    let game_state_data = global_game_state.try_borrow_data()?;
    let game_state = bytemuck::from_bytes::<GlobalGameState>(&game_state_data[..]);

    // Validate PDAs
    let (treasury_pda, _) = pubkey::find_program_address(
        &[TREASURY_SEED],
        &crate::ID,
    );
    if treasury.key() != &treasury_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (player_state_pda, _) = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    );
    if player_state.key() != &player_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Validate token accounts
    validate_token_account(player_token_account, player.key(), mint.key())?;
    validate_token_account(treasury_token_account, &treasury_pda, mint.key())?;

    // Check player has sufficient balance
    let player_token_balance = get_token_balance(player_token_account)?;
    if player_token_balance < amount {
        return Err(CrapsError::InsufficientFunds.into());
    }

    // Load and validate treasury state for circuit breaker
    let treasury_data = treasury.try_borrow_data()?;
    let treasury_state = bytemuck::from_bytes::<Treasury>(&treasury_data[..]);
    
    // Validate deposit with circuit breaker
    validate_treasury_operation(treasury_state, game_state, TreasuryOperation::Deposit, amount)?;

    // Perform the token transfer from player to treasury
    transfer_tokens(
        player_token_account,
        treasury_token_account,
        player,
        token_program,
        mint,
        amount,
        9, // TOKEN_DECIMALS for $CRAP tokens
        &[], // No signer seeds needed, player is direct signer
    )?;

    // Load and update player state
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    let player_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..]);
    
    // Verify player ownership
    if player_data.player != player.key().as_ref() {
        return Err(CrapsError::InvalidPlayer.into());
    }

    // Update player balance
    let current_balance = player_data.get_balance();
    let new_balance = current_balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    player_data.set_balance(new_balance);

    // Update player stats
    let total_deposited = player_data.get_total_deposited();
    player_data.set_total_deposited(
        total_deposited.checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?
    );

    // Load and update treasury
    let mut treasury_data = treasury.try_borrow_mut_data()?;
    let treasury_state = bytemuck::from_bytes_mut::<Treasury>(&mut treasury_data[..]);
    
    let total_deposits = treasury_state.get_total_deposits();
    treasury_state.set_total_deposits(
        total_deposits.checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?
    );

    log!("Deposit successful");
    log!("Player: {}", player.key());
    log!("Amount: {}", amount);
    log!("New balance: {}", new_balance);

    // Emit deposit event
    let clock = Clock::get()?;
    let current_epoch = player_data.get_current_epoch();
    crate::events::emit_deposit(
        player.key(),
        amount,
        new_balance,
        false, // Regular deposit, not auto-claimed
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}

/// Handler for WithdrawV2 instruction
pub fn withdraw_v2_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Validate accounts
    if accounts.len() < 9 {
        return Err(ProgramError::NotEnoughAccountKeys);
    }

    let [treasury, player_state, treasury_token_account, player_token_account, player, treasury_authority, token_program, mint, global_game_state] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate player is signer
    if !player.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validate token program
    if token_program.key() != &pinocchio_token::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Parse instruction data
    if data.len() < core::mem::size_of::<AmountData>() {
        return Err(ProgramError::InvalidInstructionData);
    }
    let amount_data = bytemuck::from_bytes::<AmountData>(
        &data[..core::mem::size_of::<AmountData>()]
    );
    let amount = u64::from_le_bytes(amount_data.amount);

    // Validate amount
    if amount == 0 {
        return Err(CrapsError::InvalidAmount.into());
    }
    if amount > MAX_WITHDRAWAL_AMOUNT {
        return Err(CrapsError::ExceedsWithdrawalLimit.into());
    }

    // Validate PDAs
    let (treasury_pda, treasury_bump) = pubkey::find_program_address(
        &[TREASURY_SEED],
        &crate::ID,
    );
    if treasury.key() != &treasury_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    let (player_state_pda, _) = pubkey::find_program_address(
        &[SCALABLE_PLAYER_SEED, player.key().as_ref()],
        &crate::ID,
    );
    if player_state.key() != &player_state_pda {
        return Err(CrapsError::InvalidPDA.into());
    }

    // Validate treasury authority is the treasury PDA
    if treasury_authority.key() != &treasury_pda {
        return Err(CrapsError::InvalidAuthority.into());
    }

    // Validate token accounts
    validate_token_account(treasury_token_account, &treasury_pda, mint.key())?;
    validate_token_account(player_token_account, player.key(), mint.key())?;

    // Load global game state for circuit breaker
    let game_state_data = global_game_state.try_borrow_data()?;
    let game_state = bytemuck::from_bytes::<GlobalGameState>(&game_state_data[..]);

    // Load and update player state
    let mut player_state_data = player_state.try_borrow_mut_data()?;
    let player_data = bytemuck::from_bytes_mut::<ScalablePlayerState>(&mut player_state_data[..]);
    
    // Verify player ownership
    if player_data.player != player.key().as_ref() {
        return Err(CrapsError::InvalidPlayer.into());
    }

    // Check player balance
    let current_balance = player_data.get_balance();
    if current_balance < amount {
        log!("Insufficient balance: {} < {}", current_balance, amount);
        return Err(CrapsError::InsufficientFunds.into());
    }

    // Load and update treasury
    let mut treasury_data = treasury.try_borrow_mut_data()?;
    let treasury_state = bytemuck::from_bytes_mut::<Treasury>(&mut treasury_data[..]);
    
    // Validate withdrawal with circuit breaker
    validate_treasury_operation(treasury_state, game_state, TreasuryOperation::Withdrawal, amount)?;
    
    // Check treasury balance
    let total_deposits = treasury_state.get_total_deposits();
    let total_withdrawals = treasury_state.get_total_withdrawals();
    let available = total_deposits.saturating_sub(total_withdrawals);
    
    if available < amount {
        log!("Insufficient treasury funds: {} < {}", available, amount);
        return Err(CrapsError::InsufficientTreasuryFunds.into());
    }

    // Check treasury token account has sufficient balance
    let treasury_token_balance = get_token_balance(treasury_token_account)?;
    if treasury_token_balance < amount {
        log!("Insufficient treasury token balance: {} < {}", treasury_token_balance, amount);
        return Err(CrapsError::InsufficientTreasuryFunds.into());
    }

    // Perform the token transfer from treasury to player
    let treasury_bump_bytes = [treasury_bump];
    let treasury_seeds = &[TREASURY_SEED, &treasury_bump_bytes];
    transfer_tokens(
        treasury_token_account,
        player_token_account,
        treasury_authority,
        token_program,
        mint,
        amount,
        9, // TOKEN_DECIMALS for $CRAP tokens
        treasury_seeds,
    )?;

    // Update player balance - add withdrawn amount
    let new_balance = current_balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    player_data.set_balance(new_balance);

    // Update treasury balance
    let new_total_withdrawals = total_withdrawals.checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    treasury_state.set_total_withdrawals(new_total_withdrawals);

    log!("Withdrawal successful");
    log!("Player: {}", player.key());
    log!("Amount: {}", amount);
    log!("New balance: {}", new_balance);

    // Emit withdrawal event
    let clock = Clock::get()?;
    let game_state_data = player_state.try_borrow_data()?;
    let player_data = bytemuck::from_bytes::<ScalablePlayerState>(&game_state_data[..]);
    let current_epoch = player_data.get_current_epoch();
    crate::events::emit_withdrawal(
        player.key(),
        amount,
        new_balance,
        false, // Regular withdrawal, not auto-claimed
        current_epoch,
        clock.slot,
        clock.unix_timestamp,
    );

    Ok(())
}

/// Handler for DepositWithAutoClaimV2 instruction
pub fn deposit_with_auto_claim_v2_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // First process the deposit
    deposit_v2_handler(accounts, data)?;

    // Auto-claim logic would go here
    // This would check for any pending payouts from previous epochs
    // and automatically claim them as part of the deposit transaction

    log!("Deposit with auto-claim completed");

    Ok(())
}

/// Handler for WithdrawWithAutoClaimV2 instruction
pub fn withdraw_with_auto_claim_v2_handler(
    accounts: & [AccountInfo],
    data: &[u8],
) -> ProgramResult {
    // Auto-claim logic would go here first
    // This would check for any pending payouts from previous epochs
    // and automatically claim them before processing the withdrawal

    // Then process the withdrawal
    withdraw_v2_handler(accounts, data)?;

    log!("Withdraw with auto-claim completed");

    Ok(())
}