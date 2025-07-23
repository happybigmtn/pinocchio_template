use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::*,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use pinocchio_token::state::Mint;

use crate::constants::CREATE_TOKEN_SEED;

pub struct CreateTokenIxsAccounts<'info> {
    pub payer: &'info AccountInfo,
    pub mint: &'info AccountInfo,
    pub token_program: &'info AccountInfo,
}

impl<'info> TryFrom<&'info [AccountInfo]> for CreateTokenIxsAccounts<'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'info [AccountInfo]) -> Result<Self, Self::Error> {
        let [payer, mint, token_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        if !payer.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !mint.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if mint.data_len() != 0 {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if !token_program.executable() {
            return Err(ProgramError::IncorrectProgramId);
        }

        Ok(Self {
            payer,
            mint,
            token_program,
        })
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CreateTokenIxsData {
    pub token_decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority: Pubkey,
}

impl CreateTokenIxsData {
    pub const LEN: usize = core::mem::size_of::<CreateTokenIxsData>();
}

impl<'info> TryFrom<&'info [u8]> for CreateTokenIxsData {
    type Error = ProgramError;

    fn try_from(data: &'info [u8]) -> Result<Self, Self::Error> {
        let result = bytemuck::try_from_bytes::<Self>(&data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        Ok(*result)
    }
}

pub struct CreateToken<'info> {
    pub accounts: CreateTokenIxsAccounts<'info>,
    pub data: CreateTokenIxsData,
}

impl<'info> TryFrom<(&'info [AccountInfo], &'info [u8])> for CreateToken<'info> {
    type Error = ProgramError;

    fn try_from(
        (accounts, data): (&'info [AccountInfo], &'info [u8]),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateTokenIxsAccounts::try_from(accounts)?;
        let data = CreateTokenIxsData::try_from(data)?;
        Ok(Self { accounts, data })
    }
}

impl<'info> CreateToken<'info> {
    pub fn handler(&mut self) -> ProgramResult {
        pinocchio_system::instructions::CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.mint,
            space: Mint::LEN as u64,
            lamports: Rent::get()?.minimum_balance(Mint::LEN),
            owner: &self.accounts.token_program.key(),
        }
        .invoke()?;

        pinocchio_token::instructions::InitializeMint2 {
            mint: self.accounts.mint,
            decimals: self.data.token_decimals,
            mint_authority: &self.data.mint_authority,
            freeze_authority: Option::Some(&self.data.freeze_authority),
        }
        .invoke()?;

        Ok(())
    }
}
