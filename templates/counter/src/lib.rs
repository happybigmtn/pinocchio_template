#![no_std]
#![allow(unexpected_cfgs)]

#[cfg(not(feature = "no-entrypoint"))]
use pinocchio::program_entrypoint;

#[cfg(not(feature = "no-entrypoint"))]
use crate::processor::process_instruction;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod processor;
pub mod state;

pinocchio_pubkey::declare_id!("7n9593Jjq8ZWGTxkBqMJUgwmSHqBAi5u4nNGR1M41oU1");

#[cfg(not(feature = "no-entrypoint"))]
program_entrypoint!(process_instruction);
