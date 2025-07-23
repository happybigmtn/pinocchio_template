use crate::{constants::FAVORITES_SEED, state::Favorites};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};
use pinocchio_log::log;

pub struct GetPdaIxsAccounts<'info> {
    pub user: &'info AccountInfo,
    pub favorites: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for GetPdaIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, favorites] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if favorites.data_len() == 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        if !favorites.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { user, favorites })
    }
}

pub struct GetPda<'info> {
    pub accounts: GetPdaIxsAccounts<'info>,
}

impl<'info> TryFrom<&'info [AccountInfo]> for GetPda<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = GetPdaIxsAccounts::try_from(accounts)?;
        Ok(Self { accounts })
    }
}

impl<'info> GetPda<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        let favorites = unsafe {
            bytemuck::try_from_bytes_mut::<Favorites>(
                self.accounts.favorites.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        let seeds = &[FAVORITES_SEED, self.accounts.user.key().as_ref()];
        let (favorites_pubkey, _) = pubkey::find_program_address(seeds, &crate::ID);
        if self.accounts.favorites.key() != &favorites_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        log!(
            "User {}'s favorite number is {} and favorite color is {}",
            self.accounts.user.key(),
            u64::from_le_bytes(favorites.number),
            bytemuck::from_bytes::<[u8; 50]>(&favorites.color)
        );

        Ok(())
    }
}
