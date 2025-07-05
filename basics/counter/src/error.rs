use pinocchio::program_error::ProgramError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CounterError {
    Overflow,
}

impl From<CounterError> for ProgramError {
    fn from(_error: CounterError) -> Self {
        ProgramError::Custom(6001)
    }
}
