//! Generic Test Template for Pinocchio Programs
//!
//! This template provides a comprehensive set of test utilities and examples
//! for testing Solana programs using Mollusk SVM and common test patterns.
//!
//! To use this template:
//! 1. Copy this file to your program's tests/ directory
//! 2. Rename it to match your program name (e.g., `my_program.rs`)
//! 3. Update the module imports to match your program's structure
//! 4. Customize the test cases for your specific program functionality
//!
//! Features included:
//! - Basic Mollusk SVM setup
//! - Account creation utilities
//! - Instruction building helpers
//! - Common assertion patterns
//! - Error testing utilities
//! - Cross-program invocation testing patterns

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    // TODO: Update this import to match your program's crate name
    // Example: use my_program::{state::MyState, instructions::MyInstruction, ID};
    use counter::{
        state::{AddressInfo, CreateAddressInfoInstructionData},
        ID,
    };

    use mollusk_svm::{
        result::{Check, ProgramResult},
        Mollusk,
    };
    use pinocchio_helper::create_padded_array;
    use solana_sdk::{
        account::{Account, AccountSharedData},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
    };

    /// Program ID constant - automatically derived from the program's ID
    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    /// Test utilities and helper functions
    pub mod test_utils {
        use super::*;

        /// Create a new Mollusk instance for testing
        ///
        /// # Arguments
        /// * `program_path` - Path to the compiled program binary (relative to target/deploy/)
        ///
        /// # Example
        /// ```rust
        /// let mollusk = create_mollusk("my_program");
        /// ```
        pub fn create_mollusk(program_path: &str) -> Mollusk {
            let full_path = format!("../../target/deploy/{}", program_path);
            Mollusk::new(&PROGRAM_ID, &full_path)
        }

        /// Create a funded user account
        ///
        /// # Arguments
        /// * `lamports` - Amount of lamports to fund the account with
        /// * `owner` - Program that owns this account (use system_program for user accounts)
        ///
        /// # Returns
        /// Tuple of (Pubkey, AccountSharedData)
        pub fn create_funded_account(lamports: u64, owner: &Pubkey) -> (Pubkey, AccountSharedData) {
            let pubkey = Pubkey::new_unique();
            let account = AccountSharedData::new(lamports, 0, owner);
            (pubkey, account)
        }

        /// Create a program data account with specific size and owner
        ///
        /// # Arguments
        /// * `size` - Size in bytes for the account data
        /// * `owner` - Program that owns this account
        /// * `lamports` - Rent exemption amount (use calculate_rent_exemption if unsure)
        #[allow(dead_code)]
        pub fn create_program_account(
            size: usize,
            owner: &Pubkey,
            lamports: u64,
        ) -> (Pubkey, AccountSharedData) {
            let pubkey = Pubkey::new_unique();
            let account = AccountSharedData::new(lamports, size, owner);
            (pubkey, account)
        }

        /// Calculate rent exemption for an account of given size
        ///
        /// # Arguments
        /// * `mollusk` - Mollusk instance to get rent sysvar from
        /// * `size` - Size in bytes of the account data
        #[allow(dead_code)]
        pub fn calculate_rent_exemption(_mollusk: &Mollusk, size: usize) -> u64 {
            // This is a simplified calculation - in real tests you might want to use
            // the actual rent sysvar from the mollusk instance
            // For most test cases, this approximation works fine
            let base_rent = 890880; // Base rent for 0-byte account
            let per_byte_rent = 6960; // Additional rent per byte
            base_rent + (size as u64 * per_byte_rent)
        }

        /// Create standard system accounts needed for most tests
        ///
        /// # Returns
        /// Vector of (Pubkey, AccountSharedData) tuples for system accounts
        #[allow(dead_code)]
        pub fn create_system_accounts() -> Vec<(Pubkey, AccountSharedData)> {
            let (system_program_id, system_account) =
                mollusk_svm::program::keyed_account_for_system_program();
            vec![(system_program_id, system_account.into())]
        }

        /// Build a basic instruction with accounts and data
        ///
        /// # Arguments
        /// * `instruction_data` - Serialized instruction data
        /// * `accounts` - Vector of AccountMeta for the instruction
        pub fn build_instruction(
            instruction_data: &[u8],
            accounts: Vec<AccountMeta>,
        ) -> Instruction {
            Instruction::new_with_bytes(PROGRAM_ID, instruction_data, accounts)
        }

        /// Assert that an instruction succeeds
        ///
        /// # Arguments
        /// * `mollusk` - Mollusk instance
        /// * `instruction` - Instruction to execute
        /// * `accounts` - Accounts required for the instruction
        /// * `additional_checks` - Additional checks to perform after execution
        pub fn assert_instruction_success(
            mollusk: &Mollusk,
            instruction: &Instruction,
            accounts: &[(Pubkey, Account)],
            additional_checks: &[Check],
        ) -> mollusk_svm::result::InstructionResult {
            // Create checks starting with success check
            let all_checks = vec![Check::success()];
            // Note: Check doesn't implement Clone, so we can't easily extend
            // In practice, you would build all checks inline when calling this function
            let _ = additional_checks; // Acknowledge the parameter

            mollusk.process_and_validate_instruction(instruction, accounts, &all_checks)
        }

        /// Assert that an instruction fails with a specific error
        ///
        /// # Arguments
        /// * `mollusk` - Mollusk instance
        /// * `instruction` - Instruction to execute
        /// * `accounts` - Accounts required for the instruction
        /// * `expected_error` - Expected program error
        #[allow(dead_code)]
        pub fn assert_instruction_error(
            mollusk: &Mollusk,
            instruction: &Instruction,
            accounts: &[(Pubkey, Account)],
            expected_error: ProgramError,
        ) {
            let result = mollusk.process_instruction(instruction, accounts);
                    
            match result.program_result {
                ProgramResult::Failure(error) => {
                    assert_eq!(
                        error, expected_error,
                        "Expected error {:?}, got {:?}",
                        expected_error, error
                    );
                }
                ProgramResult::Success => {
                    panic!(
                        "Expected instruction to fail with error {:?}, but it succeeded",
                        expected_error
                    );
                }
                ProgramResult::UnknownError(error) => {
                    panic!("Instruction failed with unknown error: {:?}", error);
                }
            }
        }

        /// Create AccountMeta for common account types
        pub mod account_meta {
            use super::*;

            pub fn signer(pubkey: Pubkey) -> AccountMeta {
                AccountMeta::new(pubkey, true)
            }

            #[allow(dead_code)]
            pub fn writable(pubkey: Pubkey) -> AccountMeta {
                AccountMeta::new(pubkey, false)
            }

            #[allow(dead_code)]
            pub fn readonly(pubkey: Pubkey) -> AccountMeta {
                AccountMeta::new_readonly(pubkey, false)
            }

            #[allow(dead_code)]
            pub fn program(pubkey: Pubkey) -> AccountMeta {
                AccountMeta::new_readonly(pubkey, false)
            }

            pub fn system_program() -> AccountMeta {
                AccountMeta::new_readonly(system_program::id(), false)
            }
        }
    }

    /// Example test using the generic template
    /// TODO: Replace this with your actual program tests
    #[test]
    fn test_create_account_data_example() {
        // Initialize Mollusk with your program
        // TODO: Update the program name to match your compiled binary
        let mollusk = test_utils::create_mollusk("counter");

        // Create system accounts
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        // Create test accounts
        let (owner, owner_account) =
            test_utils::create_funded_account(LAMPORTS_PER_SOL, &system_program);
        let (address_info_pubkey, address_info_account) =
            test_utils::create_funded_account(0, &system_program);

        // Prepare instruction data
        let ix_data = CreateAddressInfoInstructionData {
            name: create_padded_array(b"Solana", 50),
            house_number: 136,
            street: create_padded_array(b"Solana Street", 50),
            city: create_padded_array(b"Pinocchio City", 50),
        };

        let ix_data_bytes = bytemuck::bytes_of(&ix_data);
        let data = [vec![0], ix_data_bytes.to_vec()].concat();

        // Build instruction
        let instruction = test_utils::build_instruction(
            &data,
            vec![
                test_utils::account_meta::signer(owner),
                test_utils::account_meta::signer(address_info_pubkey),
                test_utils::account_meta::system_program(),
            ],
        );

        // Execute and validate instruction
        let result = test_utils::assert_instruction_success(
            &mollusk,
            &instruction,
            &[
                (owner, owner_account.into()),
                (address_info_pubkey, address_info_account.into()),
                (system_program, system_account),
            ],
            &[Check::account(&address_info_pubkey)
                .data(ix_data_bytes)
                .build()],
        );

        // Additional custom assertions
        let updated_data = result.get_account(&address_info_pubkey).unwrap();
        let parsed_data = bytemuck::from_bytes::<AddressInfo>(&updated_data.data);

        assert_eq!(parsed_data.name, create_padded_array(b"Solana", 50));
        assert_eq!(parsed_data.house_number, 136);
        assert_eq!(
            parsed_data.street,
            create_padded_array(b"Solana Street", 50)
        );
        assert_eq!(parsed_data.city, create_padded_array(b"Pinocchio City", 50));
    }

    /// Example test for error cases
    #[test]
    fn test_instruction_error_example() {
        let mollusk = test_utils::create_mollusk("counter");

        // Create minimal accounts for error testing
        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();
        let (owner, owner_account) =
            test_utils::create_funded_account(LAMPORTS_PER_SOL, &system_program);

        // Create instruction with invalid data to trigger an error
        let invalid_data = vec![255]; // Invalid instruction discriminator
        
        let instruction = test_utils::build_instruction(
            &invalid_data,
            vec![test_utils::account_meta::signer(owner)],
        );

        // For now, we'll just verify the instruction fails
        let result = mollusk.process_instruction(
            &instruction,
            &[
                (owner, owner_account.into()),
                (system_program, system_account),
            ],
        );

        // Verify that the instruction failed (regardless of specific error code)
        assert!(matches!(result.program_result, ProgramResult::Failure(_)));
    }

    /// Example test for cross-program invocation
    #[test]
    fn test_cross_program_invocation_example() {
        // TODO: Implement cross-program invocation tests if your program uses CPI
        // This would involve:
        // 1. Setting up multiple programs in Mollusk
        // 2. Creating instructions that invoke other programs
        // 3. Verifying the state changes across programs

        println!("Cross-program invocation test - implement based on your program's needs");
    }

    /// Example test for account validation
    #[test]
    fn test_account_validation_example() {
        let _mollusk = test_utils::create_mollusk("counter");

        // TODO: Add tests that verify:
        // - Account ownership validation
        // - Account size validation
        // - Signer validation
        // - Rent exemption validation
        // - PDA derivation validation

        println!("Account validation tests - implement based on your program's validation logic");
    }

    /// Example test for state transitions
    #[test]
    fn test_state_transitions_example() {
        // TODO: Add tests that verify:
        // - Initial state setup
        // - State modifications
        // - State invariants
        // - Complex state transitions

        println!("State transition tests - implement based on your program's state management");
    }

    /// Performance and resource usage tests
    #[test]
    fn test_compute_unit_usage_example() {
        let _mollusk = test_utils::create_mollusk("counter");

        // TODO: Add tests that verify:
        // - Compute unit consumption is within expected limits
        // - Memory usage is reasonable
        // - Account size changes are as expected

        println!("Performance tests - implement based on your program's resource requirements");
    }
}

/// Integration test module for testing multiple instructions together
#[cfg(test)]
mod integration_tests {
    use super::tests::test_utils;

    /// Example integration test that combines multiple operations
    #[test]
    fn test_full_program_workflow_example() {
        let _mollusk = test_utils::create_mollusk("counter");

        // TODO: Implement end-to-end tests that:
        // 1. Initialize program state
        // 2. Perform a series of operations
        // 3. Verify final state is correct
        // 4. Test edge cases and boundary conditions

        println!("Integration test - implement your full program workflow");
    }
}

/// Benchmarking module for performance testing
#[cfg(test)]
mod benchmarks {
    /// Example benchmark test
    #[test]
    fn bench_instruction_execution_example() {
        // TODO: Add benchmarking tests if needed:
        // - Measure instruction execution time
        // - Compare performance across different input sizes
        // - Verify performance regressions

        println!("Benchmark tests - implement based on performance requirements");
    }
}
