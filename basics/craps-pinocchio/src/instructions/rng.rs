//! RNG instruction handler stubs for craps-pinocchio

use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
};
use pinocchio_log::log;

use crate::error::CrapsError;

/// Handler for EnableSecureRng instruction (stub - to be implemented later)
pub fn enable_secure_rng_handler(
    _accounts: &[AccountInfo],
    _data: &[u8],
) -> ProgramResult {
    log!("EnableSecureRng: Not yet implemented - optional feature");
    
    // For now, just return an error indicating this is not implemented
    // In the future, this would enable secure RNG mode
    Err(CrapsError::NotYetImplemented.into())
}