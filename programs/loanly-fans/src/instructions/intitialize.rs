use anchor_lang::prelude::*;

use crate::states::contract::Contract;

#[derive(Accounts)]
pub struct Initialize<'info> {
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
    pub system_program: Program<'info, System>,
}
