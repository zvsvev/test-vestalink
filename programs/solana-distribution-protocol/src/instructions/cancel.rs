use anchor_lang::prelude::*;

use crate::state::VestingState;

#[derive(Accounts)]
pub struct Cancel<'info> {
    pub authority_revoker: Signer<'info>,
    #[account(
        mut,
        has_one = authority_revoker,
        has_one = vault_token_account,
        has_one = treasury_return_address
    )]
    pub vesting_state: Account<'info, VestingState>,
    /// CHECK: Validated by the vesting_state has_one constraint.
    #[account(mut)]
    pub vault_token_account: UncheckedAccount<'info>,
    /// CHECK: Validated by the vesting_state has_one constraint.
    #[account(mut)]
    pub treasury_return_address: UncheckedAccount<'info>,
}

pub fn cancel(_context: Context<Cancel>) -> Result<()> {
    Ok(())
}
