//! Circuit breaker utilities for treasury protection
//!
//! This module provides circuit breaker functionality to prevent treasury
//! from being drained or manipulated through various attack vectors.

use pinocchio::{
    program_error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
};

use crate::{
    constants::*,
    error::CrapsError,
    state::{Treasury, GlobalGameState},
};

/// Circuit breaker state for tracking limits
#[derive(Debug, Clone)]
pub struct CircuitBreakerState {
    /// Current slot
    pub current_slot: u64,
    /// Treasury balance
    pub treasury_balance: u64,
    /// Total deposits in treasury
    pub total_deposits: u64,
    /// Total withdrawals from treasury
    pub total_withdrawals: u64,
    /// Total payouts from treasury
    pub total_payouts: u64,
    /// Available liquidity
    pub available_liquidity: u64,
}

impl CircuitBreakerState {
    /// Create new circuit breaker state from treasury and game state
    pub fn new(treasury: &Treasury, _game_state: &GlobalGameState) -> Result<Self, ProgramError> {
        let current_slot = Clock::get()?.slot;
        let total_deposits = treasury.get_total_deposits();
        let total_withdrawals = treasury.get_total_withdrawals();
        let total_payouts = treasury.get_total_payouts();
        
        // Calculate available liquidity (deposits - withdrawals - payouts)
        let available_liquidity = total_deposits
            .saturating_sub(total_withdrawals)
            .saturating_sub(total_payouts);
        
        Ok(Self {
            current_slot,
            treasury_balance: available_liquidity,
            total_deposits,
            total_withdrawals,
            total_payouts,
            available_liquidity,
        })
    }

    /// Check if a payout amount is within circuit breaker limits
    pub fn check_payout_limits(&self, payout_amount: u64) -> Result<(), CrapsError> {
        // Check single payout limit
        if payout_amount > MAX_SINGLE_PAYOUT {
            return Err(CrapsError::SinglePayoutTooLarge);
        }

        // Check payout ratio limit (max 80% of treasury)
        let max_payout = self.available_liquidity
            .saturating_mul(MAX_PAYOUT_RATIO as u64)
            .saturating_div(100);
        
        if payout_amount > max_payout {
            return Err(CrapsError::PayoutRatioExceeded);
        }

        // Check emergency reserve is maintained
        let emergency_reserve = self.available_liquidity
            .saturating_mul(EMERGENCY_RESERVE_RATIO as u64)
            .saturating_div(100);
        
        if payout_amount > self.available_liquidity.saturating_sub(emergency_reserve) {
            return Err(CrapsError::EmergencyReserveInsufficient);
        }

        Ok(())
    }

    /// Check if a deposit amount is within circuit breaker limits
    pub fn check_deposit_limits(&self, deposit_amount: u64) -> Result<(), CrapsError> {
        // Check deposit amount limits
        if deposit_amount > MAX_DEPOSIT_AMOUNT {
            return Err(CrapsError::DepositExceedsLimit);
        }

        // TODO: Add hourly deposit limit checking
        // This would require tracking deposits per hour
        // For now, just check the single deposit limit
        
        Ok(())
    }

    /// Check if a withdrawal amount is within circuit breaker limits
    pub fn check_withdrawal_limits(&self, withdrawal_amount: u64) -> Result<(), CrapsError> {
        // Check withdrawal amount limits
        if withdrawal_amount > MAX_WITHDRAWAL_AMOUNT {
            return Err(CrapsError::WithdrawalExceedsLimit);
        }

        // Check liquidity threshold
        let liquidity_threshold = self.available_liquidity
            .saturating_mul(LIQUIDITY_THRESHOLD as u64)
            .saturating_div(100);
        
        if withdrawal_amount > liquidity_threshold {
            return Err(CrapsError::LiquidityThresholdExceeded);
        }

        // Check emergency reserve is maintained
        let emergency_reserve = self.available_liquidity
            .saturating_mul(EMERGENCY_RESERVE_RATIO as u64)
            .saturating_div(100);
        
        if withdrawal_amount > self.available_liquidity.saturating_sub(emergency_reserve) {
            return Err(CrapsError::EmergencyReserveInsufficient);
        }

        Ok(())
    }

    /// Check if treasury has sufficient balance for operation
    pub fn check_treasury_health(&self) -> Result<(), CrapsError> {
        // Check minimum treasury balance
        if self.available_liquidity < MIN_TREASURY_BALANCE {
            return Err(CrapsError::InsufficientTreasuryBalance);
        }

        // Check if we're over liquidity threshold
        let total_committed = self.total_withdrawals + self.total_payouts;
        let utilization_ratio = if self.total_deposits > 0 {
            (total_committed * 100) / self.total_deposits
        } else {
            0
        };

        if utilization_ratio > LIQUIDITY_THRESHOLD as u64 {
            return Err(CrapsError::LiquidityThresholdExceeded);
        }

        Ok(())
    }

    /// Get current liquidity utilization percentage
    pub fn get_liquidity_utilization(&self) -> u8 {
        if self.total_deposits == 0 {
            return 0;
        }
        
        let total_committed = self.total_withdrawals + self.total_payouts;
        let utilization = (total_committed * 100) / self.total_deposits;
        
        utilization.min(100) as u8
    }

    /// Check if the circuit breaker should trip
    pub fn should_trip(&self) -> bool {
        // Trip if liquidity utilization is too high
        if self.get_liquidity_utilization() > LIQUIDITY_THRESHOLD {
            return true;
        }

        // Trip if available liquidity is below emergency reserve
        let emergency_reserve = self.available_liquidity
            .saturating_mul(EMERGENCY_RESERVE_RATIO as u64)
            .saturating_div(100);
        
        if self.available_liquidity < emergency_reserve {
            return true;
        }

        false
    }

    /// Get the maximum safe payout amount
    pub fn get_max_safe_payout(&self) -> u64 {
        let ratio_limit = self.available_liquidity
            .saturating_mul(MAX_PAYOUT_RATIO as u64)
            .saturating_div(100);
        
        let emergency_reserve = self.available_liquidity
            .saturating_mul(EMERGENCY_RESERVE_RATIO as u64)
            .saturating_div(100);
        
        let reserve_limit = self.available_liquidity.saturating_sub(emergency_reserve);
        
        MAX_SINGLE_PAYOUT
            .min(ratio_limit)
            .min(reserve_limit)
    }

    /// Get the maximum safe withdrawal amount
    pub fn get_max_safe_withdrawal(&self) -> u64 {
        let liquidity_limit = self.available_liquidity
            .saturating_mul(LIQUIDITY_THRESHOLD as u64)
            .saturating_div(100);
        
        let emergency_reserve = self.available_liquidity
            .saturating_mul(EMERGENCY_RESERVE_RATIO as u64)
            .saturating_div(100);
        
        let reserve_limit = self.available_liquidity.saturating_sub(emergency_reserve);
        
        MAX_WITHDRAWAL_AMOUNT
            .min(liquidity_limit)
            .min(reserve_limit)
    }
}

/// Validate treasury operation with circuit breaker
pub fn validate_treasury_operation(
    treasury: &Treasury,
    game_state: &GlobalGameState,
    operation_type: TreasuryOperation,
    amount: u64,
) -> Result<(), CrapsError> {
    let circuit_breaker = CircuitBreakerState::new(treasury, game_state)
        .map_err(|_| CrapsError::InvalidAccount)?;
    
    // Check if circuit breaker should trip
    if circuit_breaker.should_trip() {
        return Err(CrapsError::CircuitBreakerTripped);
    }

    // Check treasury health
    circuit_breaker.check_treasury_health()?;

    // Check operation-specific limits
    match operation_type {
        TreasuryOperation::Payout => {
            circuit_breaker.check_payout_limits(amount)?;
        }
        TreasuryOperation::Deposit => {
            circuit_breaker.check_deposit_limits(amount)?;
        }
        TreasuryOperation::Withdrawal => {
            circuit_breaker.check_withdrawal_limits(amount)?;
        }
    }

    Ok(())
}

/// Treasury operation types
#[derive(Debug, Clone, Copy)]
pub enum TreasuryOperation {
    Payout,
    Deposit,
    Withdrawal,
}

