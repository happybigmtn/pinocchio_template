// Test file for craps_pinocchio
// Add your Mollusk SVM tests here

#[cfg(test)]
mod tests {
    use craps_pinocchio::ID;
    use solana_sdk::pubkey::Pubkey;

    pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array(ID);

    #[test]
    fn test_program_id() {
        // Basic test that verifies program ID is set correctly
        assert_ne!(PROGRAM_ID, Pubkey::default());
    }
}
