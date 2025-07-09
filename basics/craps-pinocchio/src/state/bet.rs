use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Maximum values
pub const MAX_BETS_PER_BATCH: usize = 16;

/// Compressed bet data - pack multiple bets into single account
/// This is the core innovation for scalability. Each player can have multiple
/// BetBatch accounts (one per epoch), allowing up to 16 bets per batch.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct BetBatch {
    /// Which epoch these bets are for (matches GlobalGameState.game_epoch)
    pub epoch: [u8; 8],

    /// Player who owns all bets in this batch
    pub player: [u8; 32],

    /// Number of bets in this batch (max 16)
    pub bet_count: u8,

    /// Padding for alignment
    pub _padding1: [u8; 7],

    /// Total amount wagered in this batch
    pub total_amount: [u8; 8],

    /// Packed bet data: kind(6 bits) + amount_index(10 bits) = 2 bytes per bet
    /// Flattened array to support Shank (16 bets * 2 bytes = 32 bytes)
    pub packed_bets: [u8; 32],

    /// Bitfield: which bets have been evaluated (known outcome)
    pub resolved_mask: [u8; 2],

    /// Bitfield: which bets have final outcome (can be settled)
    pub realizable_mask: [u8; 2],

    /// Bitfield: which bets have been paid out
    pub settled_mask: [u8; 2],

    /// Bitfield: which bets won (only valid if realizable)
    pub winning_mask: [u8; 2],

    /// Total payout for this batch (sum of all winning bets)
    pub payout_total: [u8; 8],

    /// Individual payout for each bet (includes original stake)
    /// Flattened array to support Shank (16 bets * 8 bytes = 128 bytes)
    pub individual_payouts: [u8; 128],

    /// Come point for each bet (0 if not come/don't come)
    pub come_points: [u8; 16],

    /// Index of linked bet for odds (255 if no link)
    pub linked_bets: [u8; 16],

    /// Target numbers for repeater bets (0 if not a repeater bet)
    pub repeater_targets: [u8; 16],

    /// Cached outcome states for each bet to avoid redundant calculations
    /// 0 = not computed, 1 = loss, 2 = win, 3 = continue
    pub cached_outcomes: [u8; 16],

    /// Epoch when outcomes were last cached (to detect stale cache)
    pub cache_epoch: [u8; 8],

    /// PDA bump seed
    pub bump: u8,

    /// Final padding for alignment
    pub _padding2: [u8; 7],
}

impl BetBatch {
    pub const LEN: usize = core::mem::size_of::<Self>();

    // Getter methods for numeric fields
    pub fn get_epoch(&self) -> u64 {
        u64::from_le_bytes(self.epoch)
    }

    pub fn get_total_amount(&self) -> u64 {
        u64::from_le_bytes(self.total_amount)
    }

    pub fn get_packed_bet(&self, index: usize) -> u16 {
        if index < MAX_BETS_PER_BATCH {
            let offset = index * 2;
            u16::from_le_bytes([self.packed_bets[offset], self.packed_bets[offset + 1]])
        } else {
            0
        }
    }

    pub fn get_resolved_mask(&self) -> u16 {
        u16::from_le_bytes(self.resolved_mask)
    }

    pub fn get_realizable_mask(&self) -> u16 {
        u16::from_le_bytes(self.realizable_mask)
    }

    pub fn get_settled_mask(&self) -> u16 {
        u16::from_le_bytes(self.settled_mask)
    }

    pub fn get_winning_mask(&self) -> u16 {
        u16::from_le_bytes(self.winning_mask)
    }

    pub fn get_payout_total(&self) -> u64 {
        u64::from_le_bytes(self.payout_total)
    }

    pub fn get_individual_payout(&self, index: usize) -> u64 {
        if index < MAX_BETS_PER_BATCH {
            let offset = index * 8;
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&self.individual_payouts[offset..offset + 8]);
            u64::from_le_bytes(bytes)
        } else {
            0
        }
    }

    pub fn get_cache_epoch(&self) -> u64 {
        u64::from_le_bytes(self.cache_epoch)
    }

    // Setter methods for numeric fields
    pub fn set_epoch(&mut self, value: u64) {
        self.epoch = value.to_le_bytes();
    }

    pub fn set_total_amount(&mut self, value: u64) {
        self.total_amount = value.to_le_bytes();
    }

    pub fn set_packed_bet(&mut self, index: usize, value: u16) {
        if index < MAX_BETS_PER_BATCH {
            let offset = index * 2;
            let bytes = value.to_le_bytes();
            self.packed_bets[offset] = bytes[0];
            self.packed_bets[offset + 1] = bytes[1];
        }
    }

    pub fn set_resolved_mask(&mut self, value: u16) {
        self.resolved_mask = value.to_le_bytes();
    }

    pub fn set_realizable_mask(&mut self, value: u16) {
        self.realizable_mask = value.to_le_bytes();
    }

    pub fn set_settled_mask(&mut self, value: u16) {
        self.settled_mask = value.to_le_bytes();
    }

    pub fn set_winning_mask(&mut self, value: u16) {
        self.winning_mask = value.to_le_bytes();
    }

    pub fn set_payout_total(&mut self, value: u64) {
        self.payout_total = value.to_le_bytes();
    }

    pub fn set_individual_payout(&mut self, index: usize, value: u64) {
        if index < MAX_BETS_PER_BATCH {
            let offset = index * 8;
            let bytes = value.to_le_bytes();
            self.individual_payouts[offset..offset + 8].copy_from_slice(&bytes);
        }
    }

    pub fn set_cache_epoch(&mut self, value: u64) {
        self.cache_epoch = value.to_le_bytes();
    }

    // Helper methods
    pub fn is_bet_resolved(&self, index: usize) -> bool {
        if index >= MAX_BETS_PER_BATCH {
            return false;
        }
        let mask = self.get_resolved_mask();
        (mask & (1 << index)) != 0
    }

    pub fn is_bet_realizable(&self, index: usize) -> bool {
        if index >= MAX_BETS_PER_BATCH {
            return false;
        }
        let mask = self.get_realizable_mask();
        (mask & (1 << index)) != 0
    }

    pub fn is_bet_settled(&self, index: usize) -> bool {
        if index >= MAX_BETS_PER_BATCH {
            return false;
        }
        let mask = self.get_settled_mask();
        (mask & (1 << index)) != 0
    }

    pub fn is_bet_winner(&self, index: usize) -> bool {
        if index >= MAX_BETS_PER_BATCH {
            return false;
        }
        let mask = self.get_winning_mask();
        (mask & (1 << index)) != 0
    }
}

/// Bonus state for tracking complex multi-roll bets
/// Optimized for minimal storage while maintaining all functionality
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct BonusState {
    /// Bit-packed tracking for space efficiency
    /// Bits 0-4: numbers 2,3,4,5,6 (bit 0 = 2, bit 4 = 6)
    pub small_rolled: u8,
    
    /// Bits 0-4: numbers 8,9,10,11,12 (bit 0 = 8, bit 4 = 12)
    pub tall_rolled: u8,
    
    /// Bits 0-5: doubles 1-1 through 6-6 (bit 0 = 1-1, bit 5 = 6-6)
    pub doubles_rolled: u8,

    /// Count of unique pass line points won
    pub fire_points: u8,
    
    /// Count of unique point combinations hit
    pub hot_roller_count: u8,
    
    /// Total pass line wins (max 255)
    pub ride_line_streak: u8,
    
    /// Padding for alignment
    pub _padding1: [u8; 2],
    
    /// Hit counts for numbers 2-12 (for Repeater bets)
    pub hits: [u8; 11],
    
    /// Padding for alignment
    pub _padding2: [u8; 5],
    
    /// Pass wins for points 4,5,6,8,9,10 (for Replay bet)
    pub pass_wins: [u8; 6],
    
    /// Padding for alignment
    pub _padding3: [u8; 2],
    
    /// Count of each double (for TwiceHard bet)
    pub doubles: [u8; 6],

    /// Bump for bonus PDA
    pub bump: u8,
    
    /// Final padding for alignment
    pub _padding4: [u8; 1],
}

impl BonusState {
    pub const LEN: usize = core::mem::size_of::<Self>();

    /// Check if a small number (2-6) has been rolled
    pub fn is_small_rolled(&self, num: u8) -> bool {
        if num < 2 || num > 6 {
            return false;
        }
        let bit = num - 2;
        (self.small_rolled & (1 << bit)) != 0
    }

    /// Mark a small number as rolled
    pub fn mark_small_rolled(&mut self, num: u8) {
        if num >= 2 && num <= 6 {
            let bit = num - 2;
            self.small_rolled |= 1 << bit;
        }
    }

    /// Check if a tall number (8-12) has been rolled
    pub fn is_tall_rolled(&self, num: u8) -> bool {
        if num < 8 || num > 12 {
            return false;
        }
        let bit = num - 8;
        (self.tall_rolled & (1 << bit)) != 0
    }

    /// Mark a tall number as rolled
    pub fn mark_tall_rolled(&mut self, num: u8) {
        if num >= 8 && num <= 12 {
            let bit = num - 8;
            self.tall_rolled |= 1 << bit;
        }
    }

    /// Check if a specific double has been rolled
    pub fn is_double_rolled(&self, die_value: u8) -> bool {
        if die_value < 1 || die_value > 6 {
            return false;
        }
        let bit = die_value - 1;
        (self.doubles_rolled & (1 << bit)) != 0
    }

    /// Mark a double as rolled
    pub fn mark_double_rolled(&mut self, die_value: u8) {
        if die_value >= 1 && die_value <= 6 {
            let bit = die_value - 1;
            self.doubles_rolled |= 1 << bit;
        }
    }

    /// Check if all small numbers have been rolled
    pub fn all_small_rolled(&self) -> bool {
        self.small_rolled == 0b11111
    }

    /// Check if all tall numbers have been rolled
    pub fn all_tall_rolled(&self) -> bool {
        self.tall_rolled == 0b11111
    }

    /// Count how many different doubles have been rolled
    pub fn count_doubles_rolled(&self) -> u8 {
        self.doubles_rolled.count_ones() as u8
    }
}