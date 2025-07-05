use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError};
use shank::ShankAccount;

pub struct CreateAddressInfoAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub address_info: &'info AccountInfo,
}

/// Address information account containing personal address details
#[derive(ShankAccount)]
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct AddressInfo {
    /// Full name (up to 50 bytes, UTF-8 encoded)
    pub name: [u8; 50],
    /// House number (0-255)
    pub house_number: u8,
    /// Street name (up to 50 bytes, UTF-8 encoded)
    pub street: [u8; 50],
    /// City name (up to 50 bytes, UTF-8 encoded)
    pub city: [u8; 50],
}

impl AddressInfo {
    pub const LEN: usize = core::mem::size_of::<AddressInfo>();

    pub fn set_inner(&mut self, data: Self) -> Self {
        self.name = data.name;
        self.house_number = data.house_number;
        self.street = data.street;
        self.city = data.city;
        *self
    }
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateAddressInfoAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, address_info, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !address_info.is_signer() {
            return Err(ProgramError::InvalidAccountData);
        }

        if address_info.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }
        Ok(Self {
            payer,
            address_info,
        })
    }
}

/// Instruction data for creating an address info account
#[derive(shank::ShankType)]
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateAddressInfoInstructionData {
    /// Full name (up to 50 bytes, UTF-8 encoded)
    pub name: [u8; 50],
    /// House number (0-255)
    pub house_number: u8,
    /// Street name (up to 50 bytes, UTF-8 encoded)
    pub street: [u8; 50],
    /// City name (up to 50 bytes, UTF-8 encoded)
    pub city: [u8; 50],
}

impl CreateAddressInfoInstructionData {
    pub const LEN: usize = core::mem::size_of::<CreateAddressInfoInstructionData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateAddressInfoInstructionData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(*result)
    }
}
