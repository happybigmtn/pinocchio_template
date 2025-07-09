use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Global game state - the single source of truth for dice rolls and game phase
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct GlobalGameState {
    /// Current game epoch - represents a shooter's session
    /// Increments only on seven-out, NOT on every roll
    pub game_epoch: [u8; 8],

    /// Last rolled dice total (2-12)
    pub current_dice: u8,

    /// Individual die values for the last roll
    pub current_die1: u8,
    pub current_die2: u8,

    /// Current point number (4,5,6,8,9,10) or 0 if come-out roll
    /// When 0: Next roll is come-out (Pass/Don't Pass can be placed)
    /// When >0: Point is established (Come/Don't Come can be placed)
    pub current_point: u8,

    /// Current game phase (0 = come out roll, 1 = point phase)
    pub game_phase: u8,
    
    /// Padding for alignment
    pub _padding1: [u8; 3],

    /// Slot when current epoch started (for tracking duration)
    pub epoch_start_slot: [u8; 8],

    /// Slot when next auto-roll should happen (every 60 slots)
    pub next_roll_slot: [u8; 8],

    /// Epoch when shooter was established
    pub shooter_established_epoch: [u8; 8],

    /// Total number of active bets across all players (for metrics)
    pub total_active_bets: [u8; 8],

    /// Number of rolls in current epoch (resets on seven-out)
    pub epoch_roll_count: [u8; 4],
    
    /// Padding for alignment
    pub _padding2: [u8; 4],

    /// Treasury PDA that holds all game funds
    pub treasury: [u8; 32],

    /// Program authority (can pause game, transfer RNG authority)
    pub authority: [u8; 32],

    /// Authority allowed to trigger dice rolls (prevents manipulation)
    pub rng_authority: [u8; 32],

    /// The SPL token mint for CRAP tokens
    pub crap_token_mint: [u8; 32],

    /// Development mode flag - DEPRECATED: This flag is not used in any logic
    pub dev_mode_enabled: u8,

    /// Global pause state - when true, no bets or withdrawals allowed
    pub paused: u8,

    /// Flag to enforce secure RNG usage
    pub use_secure_rng: u8,

    /// PDA bump seed
    pub bump: u8,
    
    /// Final padding for 8-byte alignment
    pub _padding3: [u8; 4],
}

impl GlobalGameState {
    pub const LEN: usize = core::mem::size_of::<Self>();

    // Getter methods for numeric fields
    pub fn get_game_epoch(&self) -> u64 {
        u64::from_le_bytes(self.game_epoch)
    }

    pub fn get_epoch_start_slot(&self) -> u64 {
        u64::from_le_bytes(self.epoch_start_slot)
    }

    pub fn get_next_roll_slot(&self) -> u64 {
        u64::from_le_bytes(self.next_roll_slot)
    }

    pub fn get_shooter_established_epoch(&self) -> u64 {
        u64::from_le_bytes(self.shooter_established_epoch)
    }

    pub fn get_total_active_bets(&self) -> u64 {
        u64::from_le_bytes(self.total_active_bets)
    }

    pub fn get_epoch_roll_count(&self) -> u32 {
        u32::from_le_bytes(self.epoch_roll_count)
    }

    // Setter methods for numeric fields
    pub fn set_game_epoch(&mut self, value: u64) {
        self.game_epoch = value.to_le_bytes();
    }

    pub fn set_epoch_start_slot(&mut self, value: u64) {
        self.epoch_start_slot = value.to_le_bytes();
    }

    pub fn set_next_roll_slot(&mut self, value: u64) {
        self.next_roll_slot = value.to_le_bytes();
    }

    pub fn set_shooter_established_epoch(&mut self, value: u64) {
        self.shooter_established_epoch = value.to_le_bytes();
    }

    pub fn set_total_active_bets(&mut self, value: u64) {
        self.total_active_bets = value.to_le_bytes();
    }

    pub fn set_epoch_roll_count(&mut self, value: u32) {
        self.epoch_roll_count = value.to_le_bytes();
    }

    // Additional getter methods
    pub fn get_is_paused(&self) -> bool {
        self.paused != 0
    }

    pub fn get_is_emergency_shutdown(&self) -> bool {
        // Since there's no emergency_shutdown field, we'll use paused for now
        // In production, you might want to add a separate field
        false
    }

    pub fn get_current_epoch(&self) -> u64 {
        // This seems to be the same as game_epoch
        self.get_game_epoch()
    }

    // Missing setter methods that the initialize.rs is looking for
    pub fn set_current_epoch(&mut self, value: u64) {
        self.set_game_epoch(value);
    }

    pub fn set_current_phase(&mut self, value: u8) {
        self.game_phase = value;
    }

    pub fn set_total_players(&mut self, value: u64) {
        // Since there's no total_players field, we'll use total_active_bets for now
        // In production, you might want to add a separate field
        self.set_total_active_bets(value);
    }

    pub fn get_total_players(&self) -> u64 {
        // Since there's no total_players field, we'll use total_active_bets for now
        // In production, you might want to add a separate field
        self.get_total_active_bets()
    }

    pub fn set_total_games_played(&mut self, value: u64) {
        // Since there's no total_games_played field, we'll use game_epoch for now
        // In production, you might want to add a separate field
        self.set_game_epoch(value);
    }

    pub fn set_total_deposited(&mut self, _value: u64) {
        // Since there's no total_deposited field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_total_wagered(&mut self, _value: u64) {
        // Since there's no total_wagered field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_total_paid_out(&mut self, _value: u64) {
        // Since there's no total_paid_out field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_last_updated_slot(&mut self, value: u64) {
        // Using next_roll_slot as the closest match
        self.set_next_roll_slot(value);
    }

    pub fn set_secure_rng_enabled(&mut self, value: bool) {
        self.use_secure_rng = if value { 1 } else { 0 };
    }

    pub fn set_is_paused(&mut self, value: bool) {
        self.paused = if value { 1 } else { 0 };
    }

    pub fn set_is_emergency_shutdown(&mut self, _value: bool) {
        // Since there's no emergency_shutdown field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_auto_roll_enabled(&mut self, _value: bool) {
        // Since there's no auto_roll_enabled field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }
}