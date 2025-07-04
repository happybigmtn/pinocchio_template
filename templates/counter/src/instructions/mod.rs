use shank::ShankInstruction;

/// Instructions for the Counter program
#[derive(Clone, Debug, ShankInstruction)]
pub enum CounterInstruction {
    /// Initialize a new counter account
    /// 
    /// Accounts:
    /// 0. `[writable, signer]` Counter account (will be created)
    /// 1. `[signer]` Authority account
    /// 2. `[]` System program
    #[account(0, writable, signer, name = "counter", desc = "Counter account to initialize")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    #[account(2, name = "system_program", desc = "System program")]
    Initialize {
        /// Initial counter value (optional, defaults to 0)
        initial_value: Option<u64>,
    },

    /// Increment the counter by 1
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "counter", desc = "Counter account to increment")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Increment,

    /// Decrement the counter by 1
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "counter", desc = "Counter account to decrement")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Decrement,

    /// Set the counter to a specific value
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "counter", desc = "Counter account to update")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    SetValue {
        /// New counter value
        new_value: u64,
    },

    /// Reset the counter to zero
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "counter", desc = "Counter account to reset")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Reset,
}
