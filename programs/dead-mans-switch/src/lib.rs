use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::Sysvar;


declare_id!("AMSMud7cW6iTcFkYxgfiUKsrHoNJ2wDS4Gwg9ZwjPpmw");

#[program]
pub mod dead_mans_switch {
    use super::*;

    // Initialize the dead man's switch
    pub fn initialize(ctx: Context<Initialize>, deadline: i64, beneficiary: Pubkey) -> Result<()> {
        let switch = &mut ctx.accounts.switch;
        switch.user = *ctx.accounts.user.key;
        switch.beneficiary = beneficiary;
        switch.deadline = deadline;
        Ok(())
    }

    // Check in to reset the deadline
    pub fn check_in(ctx: Context<CheckIn>, new_deadline: i64) -> Result<()> {
        let switch = &mut ctx.accounts.switch;
        require!(switch.user == *ctx.accounts.user.key, ErrorCode::Unauthorized);
        switch.deadline = new_deadline;
        Ok(())
    }

    // Trigger the switch if the deadline has passed
    pub fn trigger(ctx: Context<Trigger>) -> Result<()> {
        let switch = &mut ctx.accounts.switch;
        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(current_timestamp >= switch.deadline, ErrorCode::DeadlineNotReached);

        // Transfer funds to the beneficiary
        let amount = ctx.accounts.user.lamports();
        **ctx.accounts.user.lamports.borrow_mut() -= amount;
        **ctx.accounts.beneficiary.lamports.borrow_mut() += amount;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 48)]
    pub switch: Account<'info, Switch>,
    /// CHECK: This is the user's account, and they are signing the transaction.
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CheckIn<'info> {
    #[account(mut)]
    pub switch: Account<'info, Switch>,
    /// CHECK: This is the user's account, and they are signing the transaction.
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Trigger<'info> {
    #[account(mut)]
    pub switch: Account<'info, Switch>,
    /// CHECK: This is the user's account, and it is being modified.
    #[account(mut)]
    pub user: AccountInfo<'info>,
    /// CHECK: This is the beneficiary's account, and it is being modified.
    #[account(mut)]
    pub beneficiary: AccountInfo<'info>,
}

#[account]
pub struct Switch {
    pub user: Pubkey,
    pub beneficiary: Pubkey,
    pub deadline: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Deadline not reached")]
    DeadlineNotReached,
}


