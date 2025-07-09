use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// RNG phase enum represented as u8
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum RngPhase {
    Betting = 0,
    Collection = 1,
    Finalized = 2,
}

impl Default for RngPhase {
    fn default() -> Self {
        RngPhase::Betting
    }
}

/// RNG state for commit-reveal pattern
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct RngState {
    /// Current epoch for this RNG cycle
    pub epoch: [u8; 8],
    
    /// Current phase (0=Betting, 1=Collection, 2=Finalized)
    pub phase: u8,
    
    /// Number of hashes collected
    pub hash_count: u8,
    
    /// Padding for alignment
    pub _padding1: [u8; 6],
    
    /// Slot when betting phase started
    pub betting_start_slot: [u8; 8],
    
    /// Slot when collection phase can start
    pub collection_start_slot: [u8; 8],
    
    /// Slot when RNG was finalized
    pub finalization_slot: [u8; 8],
    
    /// Block hashes collected (up to 10)
    /// Flattened array to support Shank (10 hashes * 32 bytes = 320 bytes)
    pub block_hashes: [u8; 320],
    
    /// Final random value generated
    pub final_value: [u8; 8],
    
    /// PDA bump seed
    pub bump: u8,
    
    /// Final padding for alignment
    pub _padding2: [u8; 7],
}

impl RngState {
    pub const LEN: usize = core::mem::size_of::<Self>();

    // Getter methods for numeric fields
    pub fn get_epoch(&self) -> u64 {
        u64::from_le_bytes(self.epoch)
    }

    pub fn get_betting_start_slot(&self) -> u64 {
        u64::from_le_bytes(self.betting_start_slot)
    }

    pub fn get_collection_start_slot(&self) -> u64 {
        u64::from_le_bytes(self.collection_start_slot)
    }

    pub fn get_finalization_slot(&self) -> u64 {
        u64::from_le_bytes(self.finalization_slot)
    }

    pub fn get_final_value(&self) -> u64 {
        u64::from_le_bytes(self.final_value)
    }

    pub fn get_phase(&self) -> RngPhase {
        match self.phase {
            0 => RngPhase::Betting,
            1 => RngPhase::Collection,
            2 => RngPhase::Finalized,
            _ => RngPhase::Betting, // Default for invalid values
        }
    }

    // Setter methods for numeric fields
    pub fn set_epoch(&mut self, value: u64) {
        self.epoch = value.to_le_bytes();
    }

    pub fn set_betting_start_slot(&mut self, value: u64) {
        self.betting_start_slot = value.to_le_bytes();
    }

    pub fn set_collection_start_slot(&mut self, value: u64) {
        self.collection_start_slot = value.to_le_bytes();
    }

    pub fn set_finalization_slot(&mut self, value: u64) {
        self.finalization_slot = value.to_le_bytes();
    }

    pub fn set_final_value(&mut self, value: u64) {
        self.final_value = value.to_le_bytes();
    }

    pub fn set_phase(&mut self, phase: RngPhase) {
        self.phase = phase as u8;
    }

    // Missing methods that initialize.rs is looking for
    pub fn set_rng_phase(&mut self, phase: u8) {
        self.phase = phase;
    }

    pub fn set_hash_count(&mut self, count: u8) {
        self.hash_count = count;
    }

    pub fn set_last_finalized_epoch(&mut self, epoch: u64) {
        // Since there's no last_finalized_epoch field, we'll use epoch for now
        // In production, you might want to add a separate field
        self.set_epoch(epoch);
    }

    pub fn set_last_update_slot(&mut self, slot: u64) {
        // Use finalization_slot as the closest match
        self.set_finalization_slot(slot);
    }

    pub fn set_total_collections(&mut self, _count: u64) {
        // Since there's no total_collections field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_successful_finalizations(&mut self, _count: u64) {
        // Since there's no successful_finalizations field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_failed_finalizations(&mut self, _count: u64) {
        // Since there's no failed_finalizations field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    pub fn set_is_active(&mut self, _active: bool) {
        // Since there's no is_active field, we'll implement as no-op for now
        // In production, you might want to add a separate field
    }

    // Helper methods
    pub fn is_collection_complete(&self) -> bool {
        self.hash_count >= 5 || self.get_phase() == RngPhase::Finalized
    }

    pub fn is_finalized(&self) -> bool {
        self.get_phase() == RngPhase::Finalized
    }

    pub fn can_collect_hash(&self, current_slot: u64) -> bool {
        self.get_phase() == RngPhase::Collection && 
        current_slot >= self.get_collection_start_slot()
    }

    pub fn add_hash(&mut self, hash: [u8; 32]) -> bool {
        if self.hash_count >= 10 {
            return false;
        }

        let offset = (self.hash_count as usize) * 32;
        self.block_hashes[offset..offset + 32].copy_from_slice(&hash);
        self.hash_count += 1;

        // Check if we have enough hashes
        if self.hash_count >= 5 {
            self.set_phase(RngPhase::Finalized);
            return true;
        }

        false
    }

    pub fn reset_for_epoch(&mut self, epoch: u64, current_slot: u64) {
        self.set_epoch(epoch);
        self.set_phase(RngPhase::Betting);
        self.set_betting_start_slot(current_slot);
        
        // Extended betting window with pseudo-random component
        let base_betting_delay = 100u64;
        let timing_entropy = (epoch.wrapping_mul(0x9E3779B97F4A7C15) ^ current_slot) % 50;
        let betting_window = base_betting_delay + timing_entropy;
        
        self.set_collection_start_slot(current_slot + betting_window);
        self.set_finalization_slot(0);
        self.hash_count = 0;
        self.block_hashes = [0; 320];
        self.set_final_value(0);
    }

    /// Get a specific block hash
    pub fn get_block_hash(&self, index: usize) -> Option<[u8; 32]> {
        if index >= 10 || index >= self.hash_count as usize {
            return None;
        }
        let offset = index * 32;
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&self.block_hashes[offset..offset + 32]);
        Some(hash)
    }

    /// Generate dice roll from final value
    pub fn get_dice_roll(&self) -> (u8, u8) {
        let final_val = self.get_final_value();
        
        // Extract two dice values from the random number
        let die1 = ((final_val % 6) + 1) as u8;
        let die2 = (((final_val >> 8) % 6) + 1) as u8;
        
        (die1, die2)
    }
}