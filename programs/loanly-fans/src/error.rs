use anchor_lang::prelude::*;

#[error_code]
pub enum LoanPaidError {
    #[msg("Not enough funds in the vault account to pay the loan.")]
    NotEnoughFunds,
}

impl From<LoanPaidError> for ProgramError {
    fn from(e: LoanPaidError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
