use anchor_lang::prelude::*;

#[error_code]
pub enum LoanConfirmError {
    #[msg("Not enough funds in the vault account to pay the loan.")]
    NotEnoughFunds,

    #[msg("This loaner is have a bad history.")]
    BadLoaner,
}

impl From<LoanConfirmError> for ProgramError {
    fn from(e: LoanConfirmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
