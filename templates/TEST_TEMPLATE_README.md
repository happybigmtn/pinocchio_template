# Generic Test Template for Pinocchio Programs

This directory contains a comprehensive test template (`generic_test_template.rs`) that provides a robust foundation for testing Solana programs using Mollusk SVM.

## Features

The generic test template includes:

### 🧪 **Core Testing Utilities**
- **Mollusk SVM Setup**: Simplified program initialization
- **Account Creation**: Helper functions for creating funded accounts, program accounts, and system accounts
- **Instruction Building**: Utilities for constructing and executing instructions
- **Assertion Helpers**: Success/failure validation with detailed error checking

### 🔧 **Test Categories**
- **Unit Tests**: Individual instruction testing
- **Integration Tests**: Multi-instruction workflows
- **Error Testing**: Validation of error conditions
- **Performance Tests**: Compute unit and resource usage validation
- **Cross-Program Invocation**: CPI testing patterns

### 📋 **Test Patterns Included**
- Basic instruction execution
- Account validation
- State transitions
- Error handling
- Resource usage monitoring
- Benchmarking

## How to Use

### 1. Copy the Template
```bash
# Copy the generic template to your program's tests directory
cp templates/generic_test_template.rs your_program/tests/your_program.rs
```

### 2. Update Imports
Edit the copied file and update the module imports:

```rust
// Change this:
use account_data_template::{
    state::{AddressInfo, CreateAddressInfoInstructionData},
    ID,
};

// To match your program:
use your_program_name::{
    state::{YourState, YourInstructionData},
    ID,
};
```

### 3. Update Program Binary Path
Update the program binary name in test functions:

```rust
// Change this:
let mollusk = test_utils::create_mollusk("account_data_template");

// To your program name:
let mollusk = test_utils::create_mollusk("your_program_name");
```

### 4. Implement Your Tests
Replace the example tests with your actual program logic:

```rust
#[test]
fn test_your_instruction() {
    let mollusk = test_utils::create_mollusk("your_program");
    
    // Your test implementation here
    // Use the provided utilities to:
    // - Create accounts
    // - Build instructions  
    // - Execute and validate results
}
```

## Test Utilities Reference

### Account Creation
```rust
// Create a funded user account
let (pubkey, account) = test_utils::create_funded_account(lamports, owner);

// Create a program data account
let (pubkey, account) = test_utils::create_program_account(size, owner, lamports);

// Create system accounts
let system_accounts = test_utils::create_system_accounts();
```

### Instruction Building
```rust
// Build instruction with helper
let instruction = test_utils::build_instruction(data, accounts);

// Use account meta helpers
let accounts = vec![
    test_utils::account_meta::signer(user_pubkey),
    test_utils::account_meta::writable(data_account),
    test_utils::account_meta::readonly(config_account),
    test_utils::account_meta::system_program(),
];
```

### Execution and Validation
```rust
// Assert instruction succeeds
let result = test_utils::assert_instruction_success(
    &mollusk,
    &instruction,
    &account_infos,
    &additional_checks,
);

// Assert instruction fails with specific error
test_utils::assert_instruction_error(
    &mollusk,
    &instruction, 
    &account_infos,
    expected_error_code,
);
```

## Example Test Structure

```rust
#[test]
fn test_my_instruction() {
    // 1. Setup
    let mollusk = test_utils::create_mollusk("my_program");
    let (user, user_account) = test_utils::create_funded_account(LAMPORTS_PER_SOL, &system_program::id());
    
    // 2. Prepare instruction
    let instruction_data = MyInstructionData { /* fields */ };
    let data = serialize_instruction(&instruction_data);
    let instruction = test_utils::build_instruction(&data, accounts);
    
    // 3. Execute
    let result = test_utils::assert_instruction_success(
        &mollusk,
        &instruction,
        &[(user, user_account.into())],
        &[/* additional checks */],
    );
    
    // 4. Validate
    assert_eq!(result.program_result, ProgramResult::Success);
    // Additional assertions...
}
```

## Best Practices

### ✅ Do:
- Use the provided utilities to reduce boilerplate
- Test both success and failure cases
- Validate account state changes
- Test edge cases and boundary conditions
- Use descriptive test names
- Group related tests in modules

### ❌ Don't:
- Hardcode account addresses (use `Pubkey::new_unique()`)
- Skip error case testing
- Forget to validate state changes
- Use overly complex test setups
- Ignore compute unit consumption

## Running Tests

```bash
# Run all tests for your program
cargo test

# Run specific test
cargo test test_my_instruction

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster)
cargo test --release
```

## Advanced Features

### Cross-Program Invocation Testing
For programs that use CPI, you can set up multiple programs in Mollusk:

```rust
let mollusk = Mollusk::new(&PROGRAM_ID, "../../target/deploy/my_program");
// Add other programs as needed
// mollusk.add_program(&OTHER_PROGRAM_ID, "../../target/deploy/other_program");
```

### Performance Testing
Monitor compute unit usage:

```rust
let result = mollusk.process_instruction(&instruction, &accounts);
println!("Compute units used: {}", result.compute_units_consumed);
assert!(result.compute_units_consumed < MAX_COMPUTE_UNITS);
```

### State Validation
Use Mollusk's account checking features:

```rust
let checks = vec![
    Check::account(&account_pubkey)
        .data(&expected_data)
        .lamports(expected_lamports)
        .owner(&expected_owner)
        .build(),
];
```

## Troubleshooting

### Common Issues:

1. **Binary not found**: Ensure your program is compiled with `cargo build-sbf`
2. **Import errors**: Update the module imports to match your program structure
3. **Account validation failures**: Check account ownership, size, and data alignment
4. **Test timeouts**: Large tests may need longer timeouts in CI environments

### Debugging Tips:

- Use `println!` or `dbg!` macros for debugging
- Check Mollusk logs for detailed execution information
- Validate account states before and after instruction execution
- Use `--nocapture` flag to see test output

## Contributing

When adding new test utilities to this template:

1. Add comprehensive documentation
2. Include usage examples
3. Test the utilities themselves
4. Follow Rust naming conventions
5. Keep utilities generic and reusable
