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
    use crate::errors::{
        initialize_contract::InitializeContractError, loan_confirm::LoanConfirmError,
        loan_paid::LoanPaidError,
    };

    use super::*;

    pub fn initialize_contract(
        ctx: Context<InitializeContract>,
        owner_pubkey: Pubkey,
        amount: u64,
        due_at: i64,
    ) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        require!(
            amount >= 100_000_000,
            InitializeContractError::NeedMoreAmount
        );

        contract.loaner = ctx.accounts.loaner.key();
        contract.owner = owner_pubkey;
        contract.amount = amount;
        contract.interest_factor = 1.1;
        contract.created_at = Clock::get()?.unix_timestamp;
        contract.due_at = due_at;
        contract.is_confirmed = false;
        contract.is_late_paid = false;
        contract.cashback_claimed = false;
        contract.bump = ctx.bumps.contract;

        let loaner_history = &mut ctx.accounts.loaner_history;

        if loaner_history.total_loans == 0 {
            loaner_history.loaner = ctx.accounts.loaner.key();
            loaner_history.total_loans = 0;
            loaner_history.late_paid_loans = 0;
        }

        Ok(())
    }

    pub fn loan_confirm(ctx: Context<LoanConfirm>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let loaner_history = &mut ctx.accounts.loaner_history;
        let signer = ctx.accounts.owner.key();
        let owner_balance = ctx.accounts.owner.lamports();

        // Check loaner match
        require_keys_eq!(signer, contract.owner);

        // Check loaner history if more than 3 late paid loans
        // Then the loaner is not allowed to confirm the contract
        require!(
            loaner_history.late_paid_loans <= 3,
            LoanConfirmError::BadLoaner
        );

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
        loaner_history.total_loans += 1;

        msg!(
            "Deposit success: {} lamports to {}",
            contract.amount,
            contract.loaner
        );
        Ok(())
    }

    pub fn loan_paid(ctx: Context<LoanPaid>) -> Result<()> {
        let contract = &mut ctx.accounts.contract;
        let loaner_history = &mut ctx.accounts.loaner_history;
        let signer = ctx.accounts.loaner.key();
        let loaner_balance = ctx.accounts.loaner.lamports();

        // Check loaner match
        require_keys_eq!(signer, contract.loaner);

        // Check balance of loaner
        let mut expected_payment = (contract.amount as f64 * contract.interest_factor) as u64;
        require!(
            loaner_balance >= expected_payment,
            LoanPaidError::NotEnoughFunds
        );

        // Check due at
        let now = Clock::get()?.unix_timestamp;
        if now > contract.due_at {
            contract.is_late_paid = true;
            loaner_history.late_paid_loans += 1;
        };

        if !contract.is_late_paid {
            let cashback_factor = 0.02;
            expected_payment =
                (expected_payment as f64 * (contract.interest_factor - cashback_factor)) as u64;
        }

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

        msg!(
            "Loan paid: {} lamports to {}",
            expected_payment,
            contract.owner
        );

        Ok(())
    }
}
