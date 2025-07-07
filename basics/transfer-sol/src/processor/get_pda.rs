use crate::{constants::TRANSFER_SOL_SEED, state::Favorites};
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey, ProgramResult};
use pinocchio_log::log;

pub struct GetPdaIxsAccounts<'info> {
    pub user: &'info AccountInfo,
    pub transfer_sol: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for GetPdaIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [user, transfer_sol] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if user.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if transfer_sol.data_len() == 0 {
            return Err(ProgramError::InvalidAccountData);
        }

        if !transfer_sol.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(Self { user, transfer_sol })
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
        let transfer_sol = unsafe {
            bytemuck::try_from_bytes_mut::<Favorites>(
                self.accounts.transfer_sol.borrow_mut_data_unchecked(),
            )
            .map_err(|_| ProgramError::InvalidAccountData)?
        };

        let seeds = &[TRANSFER_SOL_SEED, self.accounts.user.key().as_ref()];
        let (transfer_sol_pubkey, _) = pubkey::find_program_address(seeds, &crate::ID);
        if self.accounts.transfer_sol.key() != &transfer_sol_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        log!(
            "User {}'s favorite number is {} and favorite color is {}",
            self.accounts.user.key(),
            u64::from_le_bytes(transfer_sol.number),
            bytemuck::from_bytes::<[u8; 50]>(&transfer_sol.color)
        );

        Ok(())
    }
}
