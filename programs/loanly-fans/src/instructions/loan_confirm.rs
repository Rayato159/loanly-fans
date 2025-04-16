use anchor_lang::prelude::*;

use crate::states::{contract::Contract, LoanerHistory};

#[derive(Accounts)]
pub struct LoanConfirm<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"loan", contract.loaner.key().as_ref()],
        bump = contract.bump,
    )]
    pub contract: Account<'info, Contract>,
    #[account(
        mut,
        seeds = [b"history", contract.loaner.key().as_ref()],
        bump = loaner_history.bump,
    )]
    pub loaner_history: Account<'info, LoanerHistory>,
    #[account(mut)]
    pub loaner: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}
