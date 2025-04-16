use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Contract {
    pub loaner: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub interest_factor: f64,
    pub created_at: i64,
    pub due_at: i64,
    pub is_confirmed: bool,
    pub is_late_paid: bool,
    pub cashback_claimed: bool,
    pub bump: u8,
}
