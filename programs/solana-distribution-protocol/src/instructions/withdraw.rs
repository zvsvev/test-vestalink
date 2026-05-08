use anchor_lang::prelude::*;

use crate::state::VestingState;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub recipient: Signer<'info>,
    #[account(
        mut,
        has_one = recipient,
        has_one = vault_token_account
    )]
    pub vesting_state: Account<'info, VestingState>,
    /// CHECK: Validated by the vesting_state has_one constraint.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    /// CHECK: Recipient token account validation is added with the token transfer implementation.
    #[account(mut)]
    pub recipient_token_account: UncheckedAccount<'info>,
}

pub fn withdraw(_context: Context<Withdraw>) -> Result<()> {
    Ok(())
}
