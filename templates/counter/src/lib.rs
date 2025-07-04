#![no_std]
#![allow(unexpected_cfgs)]

pub mod constants;
#[cfg(feature = "idl")]
pub mod instructions;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pinocchio_pubkey::declare_id!("11111111111111111111111111111111");
