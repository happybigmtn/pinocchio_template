// Integration test modules
mod integration_tests;
mod instruction_handler_tests;
mod edge_case_tests;
mod security_tests;

// Existing test modules
mod bet_batch_tests;
mod game_state_tests;
mod place_bet_tests;
mod repeater_tests;
mod simple_tests;

// Re-export test utilities if needed
pub use integration_tests::*;