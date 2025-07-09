//! Utility modules for craps-pinocchio

pub mod bet_encoding;
pub mod dice;
pub mod rng;
pub mod token;
pub mod validation;

#[cfg(test)]
mod test_encoding;

pub use bet_encoding::*;
// Export dice functions with explicit naming to avoid conflicts
pub use dice::{
    calculate_roll_total, get_roll_type, is_valid_point_roll, is_point_made, is_seven_out
};
// Export RNG functions
pub use rng::*;
pub use token::*;
// Export validation functions except those that conflict with dice
pub use validation::{
    validate_bet_amount, validate_bet_for_phase, validate_bet_type,
    validate_deposit_amount, is_valid_point,
    // Skip exporting is_natural, is_craps, is_hard_way to avoid conflicts
};