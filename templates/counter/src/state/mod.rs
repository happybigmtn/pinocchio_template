use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Counter {
    /// The authority that can modify this counter
    pub authority: Pubkey,
    /// The current count value
    pub count: u64,
}

impl Counter {
    pub const SIZE: usize = 32 + 8; // Pubkey + u64

    pub fn new(authority: Pubkey) -> Self {
        Self {
            authority,
            count: 0,
        }
    }

    pub fn increment(&mut self) -> Result<(), &'static str> {
        self.count = self.count.checked_add(1).ok_or("Counter overflow")?;
        Ok(())
    }

    pub fn decrement(&mut self) -> Result<(), &'static str> {
        self.count = self.count.checked_sub(1).ok_or("Counter underflow")?;
        Ok(())
    }
}
