#![no_std]
#![allow(unexpected_cfgs)]
use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};

pub mod constants;
pub mod error;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

// TODO update with your program ID
pinocchio_pubkey::declare_id!("E4V6siQsowLXsu9akW4CT57ALDEMiMXerTzgYvy3yG7R");
