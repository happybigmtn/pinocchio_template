#![no_std]
#![allow(unexpected_cfgs)]

use pinocchio_pubkey::declare_id;

pub mod constants;
pub mod instructions;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

declare_id!("8mqZdKKFP1rLWGJk8BtwV88t5YHHfF8v5rQbL9cEqrQx");
