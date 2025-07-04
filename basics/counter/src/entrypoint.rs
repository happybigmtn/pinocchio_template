use pinocchio::{no_allocator, program_entrypoint};
use crate::processor::process_instruction;

program_entrypoint!(process_instruction);
no_allocator!();
