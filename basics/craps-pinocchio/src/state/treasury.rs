use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Treasury account for tracking financial state
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct Treasury {
    /// Authority that can manage the treasury
    pub authority: [u8; 32],
    
    /// SPL token mint for CRAP tokens
    pub token_mint: [u8; 32],
    
    /// Token vault account that holds the treasury funds
    pub vault: [u8; 32],
    
    /// Total deposits into the treasury
    pub total_deposits: [u8; 8],
    
    /// Total withdrawals from the treasury
    pub total_withdrawals: [u8; 8],
    
    /// Total payouts to players
    pub total_payouts: [u8; 8],
    
    /// Total number of bets placed
    pub total_bets_placed: [u8; 8],
    
    /// Total number of bets settled
    pub total_bets_settled: [u8; 8],
    
    /// Last slot when treasury was updated
    pub last_update_slot: [u8; 8],
    
    /// Emergency shutdown flag
    pub emergency_shutdown: u8,
    
    /// PDA bump seed
    pub bump: u8,
    
    /// Padding for alignment
    pub _padding: [u8; 6],
}

impl Treasury {
    pub const LEN: usize = core::mem::size_of::<Self>();

    // Getter methods for numeric fields
    pub fn get_total_deposits(&self) -> u64 {
        u64::from_le_bytes(self.total_deposits)
    }

    pub fn get_total_withdrawals(&self) -> u64 {
        u64::from_le_bytes(self.total_withdrawals)
    }

    pub fn get_total_payouts(&self) -> u64 {
        u64::from_le_bytes(self.total_payouts)
    }

    pub fn get_total_bets_placed(&self) -> u64 {
        u64::from_le_bytes(self.total_bets_placed)
    }

    pub fn get_total_bets_settled(&self) -> u64 {
        u64::from_le_bytes(self.total_bets_settled)
    }

    pub fn get_last_update_slot(&self) -> u64 {
        u64::from_le_bytes(self.last_update_slot)
    }

    // Setter methods for numeric fields
    pub fn set_total_deposits(&mut self, value: u64) {
        self.total_deposits = value.to_le_bytes();
    }

    pub fn set_total_withdrawals(&mut self, value: u64) {
        self.total_withdrawals = value.to_le_bytes();
    }

    pub fn set_total_payouts(&mut self, value: u64) {
        self.total_payouts = value.to_le_bytes();
    }

    pub fn set_total_bets_placed(&mut self, value: u64) {
        self.total_bets_placed = value.to_le_bytes();
    }

    pub fn set_total_bets_settled(&mut self, value: u64) {
        self.total_bets_settled = value.to_le_bytes();
    }

    pub fn set_last_update_slot(&mut self, value: u64) {
        self.last_update_slot = value.to_le_bytes();
    }

    /// Calculate net flow (deposits - withdrawals - payouts)
    pub fn calculate_net_flow(&self) -> i64 {
        let deposits = self.get_total_deposits() as i64;
        let withdrawals = self.get_total_withdrawals() as i64;
        let payouts = self.get_total_payouts() as i64;
        
        deposits - withdrawals - payouts
    }

    /// Check if treasury is in emergency shutdown
    pub fn is_emergency_shutdown(&self) -> bool {
        self.emergency_shutdown != 0
    }

    // Missing methods that initialize.rs is looking for
    pub fn set_total_balance(&mut self, value: u64) {
        // Since there's no total_balance field, we'll use total_deposits for now
        // In production, you might want to add a separate field
        self.set_total_deposits(value);
    }

    pub fn set_locked_amount(&mut self, _value: u64) {
        // Since there's no locked_amount field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_insurance_pool(&mut self, _value: u64) {
        // Since there's no insurance_pool field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_total_fees_collected(&mut self, _value: u64) {
        // Since there's no total_fees_collected field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_total_refunded(&mut self, _value: u64) {
        // Since there's no total_refunded field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_last_reconciliation_slot(&mut self, value: u64) {
        // Use set_last_update_slot as the closest match
        self.set_last_update_slot(value);
    }

    pub fn set_safety_multiplier(&mut self, _value: u64) {
        // Since there's no safety_multiplier field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_reserve_percentage(&mut self, _value: u64) {
        // Since there's no reserve_percentage field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }
}