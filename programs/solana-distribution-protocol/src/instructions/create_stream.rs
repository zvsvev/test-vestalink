use anchor_lang::prelude::*;

use crate::state::{DistributionKind, VestingState};

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateStreamParameters {
    pub distribution_kind: DistributionKind,
    pub total_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub authority_revoker: Pubkey,
    pub authority_milestone: Pubkey,
    pub treasury_return_address: Pubkey,
}

#[derive(Accounts)]
#[instruction(parameters: CreateStreamParameters)]
pub struct CreateStream<'info> {
    #[account(mut)]
    pub authority_funder: Signer<'info>,
    /// CHECK: Stored as the designated recipient for this stream.
    pub recipient: UncheckedAccount<'info>,
    /// CHECK: Stored as the SPL token mint for this stream.
    pub token_mint: UncheckedAccount<'info>,
    /// CHECK: Placeholder for the isolated vault token account from the Week 2 architecture.
    pub vault_token_account: UncheckedAccount<'info>,
    #[account(
        init,
        payer = authority_funder,
        space = VestingState::SPACE,
        seeds = [
            b"vesting-state",
            authority_funder.key().as_ref(),
            recipient.key().as_ref(),
            token_mint.key().as_ref(),
        ],
        bump
    )]
    pub vesting_state: Account<'info, VestingState>,
    pub system_program: Program<'info, System>,
}

pub fn create_stream(_context: Context<CreateStream>, _parameters: CreateStreamParameters) -> Result<()> {
    Ok(())
}
