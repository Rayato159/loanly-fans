use anchor_lang::prelude::*;

#[error_code]
pub enum LoanPaidError {
    #[msg("Not enough funds in the vault account to pay the loan.")]
    NotEnoughFunds,

    #[msg("It's too late to pay the loan. The due date has passed.")]
    LoanDueAtPassed,
}

impl From<LoanPaidError> for ProgramError {
    fn from(e: LoanPaidError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
