use anchor_lang::prelude::*;

use crate::states::{contract::Contract, LoanerHistory};

#[derive(Accounts)]
pub struct InitializeContract<'info> {
    #[account(mut)]
    pub loaner: Signer<'info>,
    #[account(
        init,
        seeds = [b"loan", loaner.key().as_ref()],
        bump,
        payer = loaner,
        space = 8 + Contract::INIT_SPACE,
    )]
    pub contract: Account<'info, Contract>,
    #[account(
        init,
        seeds = [b"history", loaner.key().as_ref()],
        bump,
        payer = loaner,
        space = 8 + Contract::INIT_SPACE,
    )]
    pub loaner_history: Account<'info, LoanerHistory>,
    pub system_program: Program<'info, System>,
}
