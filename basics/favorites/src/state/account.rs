use bytemuck::{Pod, Zeroable};
use shank::ShankAccount;

#[derive(Clone, Copy, Pod, Zeroable, ShankAccount)]
#[repr(C)]
pub struct Favorites {
    pub number: [u8; 8],
    pub color: [u8; 50],
    pub hobby1: [u8; 50],
    pub hobby2: [u8; 50],
    pub hobby3: [u8; 50],
    pub hobby4: [u8; 50],
    pub hobby5: [u8; 50],
    pub bump: u8,
}

impl Favorites {
    pub const LEN: usize = core::mem::size_of::<Self>();

    pub fn set_inner(&mut self, data: Self) -> Self {
        self.number = data.number;
        self.color.copy_from_slice(&data.color);
        self.hobby1.copy_from_slice(&data.hobby1);
        self.hobby2.copy_from_slice(&data.hobby2);
        self.hobby3.copy_from_slice(&data.hobby3);
        self.hobby4.copy_from_slice(&data.hobby4);
        self.hobby5.copy_from_slice(&data.hobby5);
        self.bump = data.bump;
        *self
    }
}
