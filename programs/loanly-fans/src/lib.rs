#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use anchor_lang::system_program;

declare_id!("7rCAqh5G1nFCYzrt3y8xnZDhtYFCX9V2rSkZu37sTRrR");

pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

#[program]
pub mod loanly_fans {
    use crate::errors::{loan_confirm::LoanConfirmError, loan_paid::LoanPaidError};

    use super::*;

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
        contract.interest_factor = 1.1;
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
        let owner_balance = ctx.accounts.owner.lamports();

        // Check loaner match
        require_keys_eq!(signer, contract.owner);

        require!(
            owner_balance >= contract.amount,
            LoanConfirmError::NotEnoughFunds
        );

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.loaner.to_account_info(),
                },
            ),
            contract.amount,
        )?;

        contract.is_confirmed = true;

        msg!(
            "Deposit success: {} lamports to {}",
            contract.amount,
            contract.loaner
        );
        Ok(())
    }

    pub fn loan_paid(ctx: Context<LoanPaid>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let signer = ctx.accounts.loaner.key();
        let loaner_balance = ctx.accounts.loaner.lamports();

        // Check loaner match
        require_keys_eq!(signer, contract.loaner);

        // Check balance of loaner
        let expected_payment = (contract.amount as f64 * contract.interest_factor) as u64;
        require!(
            loaner_balance >= expected_payment,
            LoanPaidError::NotEnoughFunds
        );

        // Check due at
        let now = Clock::get()?.unix_timestamp;
        require!(contract.due_at > now, LoanPaidError::LoanDueAtPassed);

        system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                system_program::Transfer {
                    from: ctx.accounts.loaner.to_account_info(),
                    to: ctx.accounts.owner.to_account_info(),
                },
            ),
            expected_payment,
        )?;

        contract.is_paid = true;

        msg!(
            "Loan paid: {} lamports to {}",
            expected_payment,
            contract.owner
        );

        Ok(())
    }
}
