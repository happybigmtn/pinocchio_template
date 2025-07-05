# Generic Test Template for Pinocchio Programs

This directory contains comprehensive test templates that provide a robust foundation for testing Solana programs using both **Mollusk SVM** (Rust) and **Solana Kite** (TypeScript) frameworks.

## Testing Frameworks

This template provides two complementary testing approaches:

### 🦀 **Rust Testing with Mollusk SVM**
- **Local SVM Execution**: Tests run directly against the Solana Virtual Machine
- **High Performance**: No RPC calls, fast execution
- **Comprehensive Utilities**: Account creation, instruction building, assertion helpers
- **Generic Template**: `generic_test_template.rs` provides reusable patterns

### 🟦 **TypeScript Testing with Solana Kite**
- **Real Network Testing**: Tests against devnet/testnet/mainnet
- **Infrastructure Validation**: Client generation, RPC connectivity, deployment readiness
- **Modern TypeScript**: Full type safety and IDE support
- **Template**: `account-data.test.ts` demonstrates client usage

## Mollusk Features

The Rust test template (`generic_test_template.rs`) includes:

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

## TypeScript Testing Features

The TypeScript test template (`account-data.test.ts`) includes:

### 🌐 **Network Integration**
- **Devnet Testing**: Safe testing environment with reliable RPC endpoints
- **Helius RPC Support**: High-performance RPC for production usage
- **Connection Validation**: Ensures network connectivity and client functionality

### 🔧 **Infrastructure Validation**
- **Client Import Testing**: Verifies generated TypeScript clients work correctly
- **Program Binary Checks**: Validates program compilation and deployment readiness
- **Type Safety**: Full TypeScript support with generated types

### 📦 **Test Categories**
- **Infrastructure Tests**: RPC connectivity, client generation, type checking
- **Integration Tests**: Real network interactions (when funded)
- **Deployment Readiness**: Verifies program is ready for deployment

## How to Use

### Rust Testing with Mollusk

#### 1. Copy the Template
```bash
# Copy the generic template to your program's tests directory
cp templates/generic_test_template.rs your_program/tests/your_program.rs
```

### 2. Update Imports
Edit the copied file and update the module imports:

```rust
// Change this:
use counter::{
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
let mollusk = test_utils::create_mollusk("counter");

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

### TypeScript Testing with Kite

#### 1. Install Dependencies
```bash
# Install Bun (if not already installed)
curl -fsSL https://bun.sh/install | bash

# Install dependencies
bun install
```

#### 2. Set Up Environment
```bash
# Copy environment template
cp .env.example .env

# Add your Helius API key to .env
HELIUS_API_KEY=your_helius_api_key_here
```

#### 3. Generate TypeScript Client
```bash
# Generate IDL and TypeScript client
bun gen:client:account-data
```

#### 4. Run TypeScript Tests
```bash
# Run infrastructure tests
bun test:client:account-data

# Or run all tests
bun test
```

#### 5. Customize for Your Program
Update the test file for your specific program:

```typescript
// Update imports to match your generated client
import { fetchYourData, getYourInstruction } from '../../../clients/yourprogram';

// Update test logic for your program's functionality
test('your-program:test-case', async () => {
  const kite = await connect(rpcEndpoint, wsEndpoint);
  
  // Your test implementation here
  // Use Kite functions for:
  // - Wallet management
  // - Account creation
  // - Instruction execution
  // - State validation
});
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

### ✅ Rust Testing (Mollusk) Do:
- Use the provided utilities to reduce boilerplate
- Test both success and failure cases
- Validate account state changes
- Test edge cases and boundary conditions
- Use descriptive test names
- Group related tests in modules
- Monitor compute unit consumption

### ✅ TypeScript Testing (Kite) Do:
- Test infrastructure before integration
- Use environment variables for API keys
- Validate client generation and imports
- Test against devnet before mainnet
- Use proper TypeScript types
- Handle network timeouts gracefully

### ❌ Rust Testing Don't:
- Hardcode account addresses (use `Pubkey::new_unique()`)
- Skip error case testing
- Forget to validate state changes
- Use overly complex test setups
- Ignore compute unit consumption

### ❌ TypeScript Testing Don't:
- Commit API keys to version control
- Test directly against mainnet without devnet validation
- Skip infrastructure validation tests
- Ignore network timeout errors
- Use hardcoded addresses in production tests

## Running Tests

### Rust Tests (Mollusk)

```bash
# Run all Rust tests for your program
cargo test

# Run specific test
cargo test test_my_instruction

# Run tests with output
cargo test -- --nocapture

# Run tests in release mode (faster)
cargo test --release
```

### TypeScript Tests (Kite)

```bash
# Run TypeScript tests for specific program
bun test:client:account-data

# Run all TypeScript tests
bun test

# Run with verbose output
bun test --verbose

# Run specific test file
bun test --testFiles tests/account-data.test.ts
```

### Running Both Test Suites

```bash
# Test workflow: Rust tests first (fast), then TypeScript tests (network)
cargo test && bun test:client:account-data

# Or run them independently
cargo test        # Local SVM testing
bun test         # Network integration testing
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

### Rust Testing (Mollusk) Issues:

1. **Binary not found**: Ensure your program is compiled with `cargo build-sbf`
2. **Import errors**: Update the module imports to match your program structure
3. **Account validation failures**: Check account ownership, size, and data alignment
4. **Test timeouts**: Large tests may need longer timeouts in CI environments

### TypeScript Testing (Kite) Issues:

1. **RPC connection failures**: Check network connectivity and API key configuration
2. **Client import errors**: Ensure IDL and client generation completed successfully
3. **Type errors**: Verify generated types match your program's interface
4. **Test timeouts**: Network tests may need longer timeouts (30+ seconds)
5. **Environment issues**: Verify `.env` file configuration and API key validity

### Debugging Tips:

#### Rust Tests:
- Use `println!` or `dbg!` macros for debugging
- Check Mollusk logs for detailed execution information
- Validate account states before and after instruction execution
- Use `--nocapture` flag to see test output

#### TypeScript Tests:
- Check console output for network connectivity issues
- Verify generated client files exist in `clients/` directory
- Test RPC endpoints manually using curl or similar tools
- Use browser developer tools to inspect network requests
- Check Helius dashboard for API usage and rate limits

## Test Strategy Recommendations

### Development Workflow

1. **Start with Rust Tests (Mollusk)**:
   - Fast feedback loop
   - Test core program logic
   - Validate instruction processing
   - Check error conditions

2. **Follow with TypeScript Tests (Kite)**:
   - Validate client generation
   - Test network integration
   - Verify deployment readiness
   - End-to-end functionality

3. **Use Both for Different Purposes**:
   - **Mollusk**: Unit testing, edge cases, performance
   - **Kite**: Integration testing, client validation, deployment verification

### CI/CD Integration

```bash
# Example CI workflow
#!/bin/bash
set -e

# Build program
cargo build-sbf

# Run Rust tests (fast)
cargo test

# Generate client
bun gen:client:account-data

# Run TypeScript tests (slower, network-dependent)
bun test:client:account-data

echo "All tests passed!"
```

## Contributing

When adding new test utilities to this template:

### For Rust Utilities:
1. Add comprehensive documentation
2. Include usage examples
3. Test the utilities themselves
4. Follow Rust naming conventions
5. Keep utilities generic and reusable

### For TypeScript Utilities:
1. Maintain TypeScript type safety
2. Add proper error handling
3. Include timeout configurations
4. Document network dependencies
5. Provide environment setup instructions
