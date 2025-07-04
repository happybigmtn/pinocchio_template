use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

/// Counter account state
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
pub struct Counter {
    /// Current counter value
    pub count: u64,
    /// Authority that can modify this counter
    pub authority: [u8; 32],
    /// Reserved space for future use
    pub reserved: [u8; 64],
}

impl Counter {
    /// Size of the Counter account in bytes
    pub const SIZE: usize = 8 + 32 + 64; // count + authority + reserved

    /// Create a new Counter instance
    pub fn new(authority: [u8; 32]) -> Self {
        Self {
            count: 0,
            authority,
            reserved: [0; 64],
        }
    }

    /// Increment the counter by 1
    pub fn increment(&mut self) -> Result<(), &'static str> {
        if self.count == u64::MAX {
            return Err("Counter overflow");
        }
        self.count = self.count.saturating_add(1);
        Ok(())
    }

    /// Decrement the counter by 1
    pub fn decrement(&mut self) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("Counter underflow");
        }
        self.count = self.count.saturating_sub(1);
        Ok(())
    }

    /// Set the counter to a specific value
    pub fn set_count(&mut self, new_count: u64) {
        self.count = new_count;
    }

    /// Check if the given authority matches the counter's authority
    pub fn is_authority(&self, authority: &[u8; 32]) -> bool {
        self.authority == *authority
    }
}
