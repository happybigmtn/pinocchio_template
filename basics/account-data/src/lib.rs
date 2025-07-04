#![no_std]
#![allow(unexpected_cfgs)]

pub mod constants;
pub mod instructions;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;

pinocchio_pubkey::declare_id!("Fruv5QjqNDXvvYT2hw4FjhsT5aa11bHAPtMQH46mg3SS");
