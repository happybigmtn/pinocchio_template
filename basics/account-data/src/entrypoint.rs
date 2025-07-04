use pinocchio::{no_allocator, nostd_panic_handler, program_entrypoint};
use crate::processor::process_instruction;

program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();
