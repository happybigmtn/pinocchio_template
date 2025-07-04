use shank::ShankInstruction;

/// Instructions for the Counter program
#[derive(Clone, Debug, ShankInstruction)]
pub enum CounterInstruction {
    /// Initialize a new cross_program_invocation account
    /// 
    /// Accounts:
    /// 0. `[writable, signer]` Counter account (will be created)
    /// 1. `[signer]` Authority account
    /// 2. `[]` System program
    #[account(0, writable, signer, name = "cross_program_invocation", desc = "Counter account to initialize")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    #[account(2, name = "system_program", desc = "System program")]
    Initialize {
        /// Initial cross_program_invocation value (optional, defaults to 0)
        initial_value: Option<u64>,
    },

    /// Increment the cross_program_invocation by 1
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "cross_program_invocation", desc = "Counter account to increment")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Increment,

    /// Decrement the cross_program_invocation by 1
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "cross_program_invocation", desc = "Counter account to decrement")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Decrement,

    /// Set the cross_program_invocation to a specific value
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "cross_program_invocation", desc = "Counter account to update")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    SetValue {
        /// New cross_program_invocation value
        new_value: u64,
    },

    /// Reset the cross_program_invocation to zero
    /// 
    /// Accounts:
    /// 0. `[writable]` Counter account
    /// 1. `[signer]` Authority account
    #[account(0, writable, name = "cross_program_invocation", desc = "Counter account to reset")]
    #[account(1, signer, name = "authority", desc = "Authority account")]
    Reset,
}
