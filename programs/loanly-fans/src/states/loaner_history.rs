use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct LoanerHistory {
    pub loaner: Pubkey,
    pub total_loans: u64,
    pub late_paid_loans: u64,
    pub bump: u8,
}
