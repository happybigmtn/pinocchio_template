use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::Seed,
    program_error::ProgramError,
    pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::{constants::FAVORITES_SEED, processor::create_pda_account, state::Favorites};

pub struct CreatePdaIxsAccounts<'info> {
    pub user: &'info AccountInfo,
    pub favorites: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreatePdaIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, favorites, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if favorites.is_writable() {
            return Err(ProgramError::InvalidAccountData);
        }

        if favorites.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(Self { user, favorites })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreatePdaIxsData {
    pub number: [u8; 8],
    pub color: [u8; 50],
    pub hobby1: [u8; 50],
    pub hobby2: [u8; 50],
    pub hobby3: [u8; 50],
    pub hobby4: [u8; 50],
    pub hobby5: [u8; 50],
    pub bump: u8,
}

impl CreatePdaIxsData {
    pub const LEN: usize = core::mem::size_of::<CreatePdaIxsData>();
}

impl<'info> TryFrom<&'info [u8]> for CreatePdaIxsData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok(*result)
    }
}

pub struct CreatePda<'info> {
    pub accounts: CreatePdaIxsAccounts<'info>,
    pub data: CreatePdaIxsData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreatePda<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreatePdaIxsAccounts::try_from(accounts)?;
        let data = CreatePdaIxsData::try_from(data)?;
        Ok(Self { accounts, data })
    }
}

impl<'info> CreatePda<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let favorites_pubkey = pubkey::create_program_address(
            &[
                FAVORITES_SEED,
                self.accounts.user.key().as_ref(),
                &[self.data.bump as u8],
            ],
            &crate::ID,
        )
        .map_err(|_| ProgramError::InvalidSeeds)?;

        if self.accounts.favorites.key() != &favorites_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        let bump = [self.data.bump as u8];
        let seed = [
            Seed::from(FAVORITES_SEED),
            Seed::from(self.accounts.user.key().as_ref()),
            Seed::from(&bump),
        ];

        create_pda_account(
            self.accounts.user,
            &Rent::get()?,
            Favorites::LEN,
            &crate::ID,
            self.accounts.favorites,
            seed,
        )?;

        let favorites = unsafe {
            bytemuck::try_from_bytes_mut::<Favorites>(
                self.accounts.favorites.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        favorites.set_inner(Favorites {
            number: self.data.number,
            color: self.data.color,
            hobby1: self.data.hobby1,
            hobby2: self.data.hobby2,
            hobby3: self.data.hobby3,
            hobby4: self.data.hobby4,
            hobby5: self.data.hobby5,
            bump: self.data.bump,
        });

        Ok(())
    }
}
