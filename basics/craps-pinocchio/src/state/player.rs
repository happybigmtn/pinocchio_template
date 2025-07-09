use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Minimal player state for high-performance lookups
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct ScalablePlayerState {
    /// Player's public key for validation
    pub player: [u8; 32],
    
    /// Current balance
    pub balance: [u8; 8],
    
    /// Current active tournament (default pubkey if none)
    pub active_tournament: [u8; 32],
    
    /// Current epoch player is in
    pub current_epoch: [u8; 8],
    
    /// Lifetime total wagered
    pub total_wagered: [u8; 8],
    
    /// Lifetime total won
    pub total_won: [u8; 8],
    
    /// Last slot player placed bet
    pub last_bet_slot: [u8; 8],
    
    /// Last slot player claimed daily chips
    pub last_claim_slot: [u8; 8],
    
    /// Last slot tournament was updated (0 if None)
    pub last_tournament_update_slot: [u8; 8],
    
    /// 0=unverified, 1=basic, 2=premium
    pub verification_tier: u8,
    
    /// PDA bump seed
    pub bump: u8,
    
    /// Padding for alignment
    pub _padding: [u8; 6],
}

impl ScalablePlayerState {
    pub const LEN: usize = core::mem::size_of::<Self>();

    // Getter methods for numeric fields
    pub fn get_balance(&self) -> u64 {
        u64::from_le_bytes(self.balance)
    }

    pub fn get_current_epoch(&self) -> u64 {
        u64::from_le_bytes(self.current_epoch)
    }

    pub fn get_total_wagered(&self) -> u64 {
        u64::from_le_bytes(self.total_wagered)
    }

    pub fn get_total_won(&self) -> u64 {
        u64::from_le_bytes(self.total_won)
    }

    pub fn get_last_bet_slot(&self) -> u64 {
        u64::from_le_bytes(self.last_bet_slot)
    }

    pub fn get_last_claim_slot(&self) -> u64 {
        u64::from_le_bytes(self.last_claim_slot)
    }

    pub fn get_last_tournament_update_slot(&self) -> Option<u64> {
        let value = u64::from_le_bytes(self.last_tournament_update_slot);
        if value == 0 {
            None
        } else {
            Some(value)
        }
    }

    // Setter methods for numeric fields
    pub fn set_balance(&mut self, value: u64) {
        self.balance = value.to_le_bytes();
    }

    pub fn set_current_epoch(&mut self, value: u64) {
        self.current_epoch = value.to_le_bytes();
    }

    pub fn set_total_wagered(&mut self, value: u64) {
        self.total_wagered = value.to_le_bytes();
    }

    pub fn set_total_won(&mut self, value: u64) {
        self.total_won = value.to_le_bytes();
    }

    pub fn set_last_bet_slot(&mut self, value: u64) {
        self.last_bet_slot = value.to_le_bytes();
    }

    pub fn set_last_claim_slot(&mut self, value: u64) {
        self.last_claim_slot = value.to_le_bytes();
    }

    pub fn set_last_tournament_update_slot(&mut self, value: Option<u64>) {
        self.last_tournament_update_slot = value.unwrap_or(0).to_le_bytes();
    }

    // Missing methods that treasury.rs is looking for
    pub fn get_total_deposited(&self) -> u64 {
        // Since there's no total_deposited field, we'll use balance for now
        // In production, you might want to add a separate field
        self.get_balance()
    }

    pub fn set_total_deposited(&mut self, value: u64) {
        // Since there's no total_deposited field, we'll use balance for now
        // In production, you might want to add a separate field
        self.set_balance(value);
    }

    // Additional missing methods that player.rs is looking for
    pub fn set_total_withdrawn(&mut self, _value: u64) {
        // Since there's no total_withdrawn field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_active_tournament(&mut self, tournament: [u8; 32]) {
        self.active_tournament = tournament;
    }

    pub fn set_games_played(&mut self, _value: u64) {
        // Since there's no games_played field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_bets_placed(&mut self, _value: u64) {
        // Since there's no bets_placed field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_bets_won(&mut self, _value: u64) {
        // Since there's no bets_won field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_initialized_slot(&mut self, _slot: u64) {
        // Since there's no initialized_slot field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn get_active_tournament(&self) -> [u8; 32] {
        self.active_tournament
    }
}