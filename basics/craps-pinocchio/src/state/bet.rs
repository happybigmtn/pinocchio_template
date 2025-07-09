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

