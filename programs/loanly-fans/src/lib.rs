#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("7rCAqh5G1nFCYzrt3y8xnZDhtYFCX9V2rSkZu37sTRrR");

pub mod error;

#[program]
pub mod loanly_fans {
    use super::*;
    use anchor_lang::solana_program::program::invoke;
    use error;

    pub fn initialize(
        ctx: Context<Initialize>,
        owner_pubkey: Pubkey,
        amount: u64,
        due_at: i64,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;

        contract.loaner = ctx.accounts.loaner.key();
        contract.owner = owner_pubkey;
        contract.amount = amount;
        contract.interest_rate = 1.1;
        contract.created_at = Clock::get()?.unix_timestamp;
        contract.due_at = due_at;
        contract.is_confirmed = false;
        contract.is_paid = false;
        contract.cashback_claimed = false;
        contract.bump = ctx.bumps.contract;

        Ok(())
    }

    pub fn loan_confirm(ctx: Context<LoanConfirm>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let signer = ctx.accounts.owner.key();

        // Check owner match
        require_keys_eq!(signer, contract.owner);

        contract.is_confirmed = true;

        msg!("Loan confirmed!");
        msg!(
            "Loaner pubkey: {}, Owner pubkey: {}",
            contract.loaner,
            contract.owner
        );

        Ok(())
    }

    pub fn loan_deposit(ctx: Context<LoanDeposit>) -> Result<()> {
        let contract = &ctx.accounts.contract;

        let expected_payment = (contract.amount as f64 * contract.interest_rate) as u64;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.loaner.key(),
            &ctx.accounts.vault_account.key(),
            expected_payment,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.loaner.to_account_info(),
                ctx.accounts.vault_account.to_account_info(),
            ],
        )?;

        msg!("Deposit success: {} lamports", expected_payment);
        Ok(())
    }

    pub fn loan_paid(ctx: Context<LoanPaid>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let signer = ctx.accounts.loaner.key();
        let vault_balance = ctx.accounts.vault_account.lamports();

        // Check loaner match
        require_keys_eq!(signer, contract.loaner);

        // Check if the loan is already paid
        let expected_payment = (contract.amount as f64 * contract.interest_rate) as u64;
        require!(
            vault_balance >= expected_payment,
            error::LoanPaidError::NotEnoughFunds
        );

        contract.is_paid = true;

        msg!("Loan paid!");
        msg!(
            "Loaner pubkey: {}, Owner pubkey: {}",
            contract.loaner,
            contract.owner
        );

        Ok(())
    }
}

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

#[account]
#[derive(InitSpace)]
pub struct Contract {
    pub loaner: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub interest_rate: f64,
    pub created_at: i64,
    pub due_at: i64,
    pub is_confirmed: bool,
    pub is_paid: bool,
    pub cashback_claimed: bool,
    pub bump: u8,
}

#[derive(Accounts)]
pub struct LoanConfirm<'info> {
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"loan", contract.loaner.key().as_ref()],
        bump = contract.bump,
    )]
    pub contract: Account<'info, Contract>,
}

#[derive(Accounts)]
pub struct LoanPaid<'info> {
    pub loaner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"loan", contract.loaner.key().as_ref()],
        bump = contract.bump,
    )]
    pub contract: Account<'info, Contract>,

    /// CHECK: This is a vault PDA, verified via seeds. No manual checks needed.
    #[account(
        mut,
        seeds = [b"vault", contract.key().as_ref()],
        bump,
    )]
    pub vault_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct LoanDeposit<'info> {
    #[account(mut)]
    pub loaner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"loan", contract.key().as_ref()],
        bump = contract.bump,
    )]
    pub contract: Account<'info, Contract>,

    /// CHECK: This is a vault PDA, verified via seeds. No manual checks needed.
    #[account(
        mut,
        seeds = [b"vault", contract.key().as_ref()],
        bump,
    )]
    pub vault_account: AccountInfo<'info>,
}
