use pinocchio::pinocchio_pubkey;
use pinocchio_pubkey::declare_id;

declare_id!("11111111111111111111111111111111");

mod entrypoint;
mod processor;

pub mod instructions;
pub mod state;

pub use processor::process_instruction;
