use crate::processor::process_instruction;
use pinocchio::program_entrypoint;
use pinocchio_pubkey::declare_id;

pub mod constants;
pub mod entrypoint;
pub mod instructions;
pub mod processor;
pub mod state;

declare_id!("8mqZdKKFP1rLWGJk8BtwV88t5YHHfF8v5rQbL9cEqrQx");

program_entrypoint!(process_instruction);
