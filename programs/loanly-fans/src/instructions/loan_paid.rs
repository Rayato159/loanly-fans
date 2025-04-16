use anchor_lang::prelude::*;

use crate::states::{contract::Contract, LoanerHistory};

#[derive(Accounts)]
pub struct LoanPaid<'info> {
    #[account(mut)]
    pub loaner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"loan", loaner.key().as_ref()],
        bump = contract.bump,
    )]
    pub contract: Account<'info, Contract>,
    #[account(
        mut,
        seeds = [b"history", loaner.key().as_ref()],
        bump = loaner_history.bump,
    )]
    pub loaner_history: Account<'info, LoanerHistory>,
    #[account(mut)]
    pub owner: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
