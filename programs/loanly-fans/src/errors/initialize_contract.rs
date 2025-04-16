use anchor_lang::prelude::*;

#[error_code]
pub enum InitializeContractError {
    #[msg("Need at least 100_000_000 lamports to create a contract.")]
    NeedMoreAmount,
}

impl From<InitializeContractError> for ProgramError {
    fn from(e: InitializeContractError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
