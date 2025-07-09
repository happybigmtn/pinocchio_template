//! Token validation and helper utilities

extern crate alloc;

use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use pinocchio_token::{instructions::TransferChecked};
use bytemuck::{Pod, Zeroable};

use crate::{
    error::CrapsError,
};

/// SPL Token Account structure
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct TokenAccount {
    /// The mint associated with this account
    pub mint: [u8; 32],
    /// The owner of this account
    pub owner: [u8; 32],
    /// The amount of tokens this account holds
    pub amount: [u8; 8],
    /// Account state
    pub state: u8,
    /// Delegate (optional)
    pub delegate: [u8; 32],
    /// Is native
    pub is_native: u8,
    /// Delegated amount
    pub delegated_amount: [u8; 8],
    /// Close authority (optional)
    pub close_authority: [u8; 32],
}

impl TokenAccount {
    pub const LEN: usize = 165;
    
    pub fn get_amount(&self) -> u64 {
        u64::from_le_bytes(self.amount)
    }
    
    pub fn get_mint(&self) -> Pubkey {
        Pubkey::from(self.mint)
    }
    
    pub fn get_owner(&self) -> Pubkey {
        Pubkey::from(self.owner)
    }
    
    pub fn is_initialized(&self) -> bool {
        self.state != 0
    }
}

/// Validate that a token account is owned by the expected owner and has the correct mint
pub fn validate_token_account(
    token_account: &AccountInfo,
    expected_owner: &Pubkey,
    expected_mint: &Pubkey,
) -> Result<(), ProgramError> {
    // Check that the account is owned by the SPL Token program
    if unsafe { token_account.owner() } != &pinocchio_token::ID {
        return Err(CrapsError::InvalidTokenAccount.into());
    }
    
    // Deserialize the token account data
    let data = token_account.try_borrow_data()?;
    if data.len() < TokenAccount::LEN {
        return Err(CrapsError::InvalidTokenAccount.into());
    }
    
    let token_data = bytemuck::from_bytes::<TokenAccount>(&data[..TokenAccount::LEN]);
    
    // Check if initialized
    if !token_data.is_initialized() {
        return Err(CrapsError::AccountNotInitialized.into());
    }
    
    // Check mint
    if &token_data.get_mint() != expected_mint {
        return Err(CrapsError::InvalidTokenMint.into());
    }
    
    // Check owner
    if &token_data.get_owner() != expected_owner {
        return Err(CrapsError::InvalidTokenAccount.into());
    }
    
    Ok(())
}

/// Get the token balance from a token account
pub fn get_token_balance(token_account: &AccountInfo) -> Result<u64, ProgramError> {
    // Check that the account is owned by the SPL Token program
    if unsafe { token_account.owner() } != &pinocchio_token::ID {
        return Err(CrapsError::InvalidTokenAccount.into());
    }
    
    // Deserialize the token account data
    let data = token_account.try_borrow_data()?;
    if data.len() < TokenAccount::LEN {
        return Err(CrapsError::InvalidTokenAccount.into());
    }
    
    let token_data = bytemuck::from_bytes::<TokenAccount>(&data[..TokenAccount::LEN]);
    
    // Check if initialized
    if !token_data.is_initialized() {
        return Err(CrapsError::AccountNotInitialized.into());
    }
    
    Ok(token_data.get_amount())
}

/// Validate that a mint account is the expected one
pub fn validate_mint(
    mint_account: &AccountInfo,
    expected_mint: &Pubkey,
) -> Result<(), ProgramError> {
    // Check that the account is owned by the SPL Token program
    if unsafe { mint_account.owner() } != &pinocchio_token::ID {
        return Err(CrapsError::InvalidTokenMint.into());
    }
    
    // Check that the mint pubkey matches
    if mint_account.key() != expected_mint {
        return Err(CrapsError::InvalidTokenMint.into());
    }
    
    Ok(())
}

/// Transfer tokens using the SPL Token program
pub fn transfer_tokens<'a>(
    source: &AccountInfo,
    destination: &AccountInfo,
    authority: &AccountInfo,
    _token_program: &AccountInfo,
    mint: &AccountInfo,
    amount: u64,
    decimals: u8,
    signer_seeds: &[&[u8]],
) -> Result<(), ProgramError> {
    // Create the transfer instruction
    let transfer_ix = TransferChecked {
        from: source,
        to: destination,
        mint,
        authority,
        amount,
        decimals,
    };
    
    // Execute the transfer
    if signer_seeds.is_empty() {
        transfer_ix.invoke()?;
    } else {
        // Convert &[&[u8]] to &[Signer]
        let seeds: alloc::vec::Vec<pinocchio::instruction::Seed> = signer_seeds.iter()
            .map(|seeds| pinocchio::instruction::Seed::from(*seeds))
            .collect();
        let signer = pinocchio::instruction::Signer::from(&seeds[..]);
        transfer_ix.invoke_signed(&[signer])?;
    }
    
    Ok(())
}