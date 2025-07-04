use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::state::{AddressInfo, CreateAddressInfoAccounts, CreateAddressInfoInstructionData};

pub struct Create<'info> {
    pub accounts: CreateAddressInfoAccounts<'info>,
    pub instruction_data: CreateAddressInfoInstructionData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for Create<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateAddressInfoAccounts::try_from(accounts)?;
        let instruction_data = CreateAddressInfoInstructionData::try_from(data)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'info> Create<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.address_info,
            space: AddressInfo::LEN as u64,
            lamports: Rent::get()?.minimum_balance(AddressInfo::LEN),
            owner: &crate::ID,
        }
        .invoke()?;

        let address_info_state = unsafe {
            bytemuck::try_from_bytes_mut::<AddressInfo>(
                self.accounts.address_info.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        address_info_state.set_inner(AddressInfo {
            name: self.instruction_data.name,
            house_number: self.instruction_data.house_number,
            street: self.instruction_data.street,
            city: self.instruction_data.city,
        });

        Ok(())
    }
}
