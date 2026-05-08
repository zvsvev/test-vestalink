use anchor_lang::prelude::*;

#[account]
pub struct VestingState {
    pub authority_funder: Pubkey,
    pub recipient: Pubkey,
    pub token_mint: Pubkey,
    pub vault_token_account: Pubkey,
    pub authority_revoker: Pubkey,
    pub authority_milestone: Pubkey,
    pub treasury_return_address: Pubkey,
    pub distribution_kind: DistributionKind,
    pub total_amount: u64,
    pub unlocked_amount: u64,
    pub claimed_amount: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub cliff_time: i64,
    pub is_revoked: bool,
    pub bump: u8,
}

impl VestingState {
    pub const ACCOUNT_DISCRIMINATOR_SIZE: usize = 8;
    pub const SPACE: usize = Self::ACCOUNT_DISCRIMINATOR_SIZE
        + (32 * 7)
        + DistributionKind::SPACE
        + (8 * 6)
        + 1
        + 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum DistributionKind {
    Stream,
    LinearVesting,
    CliffVesting,
    Milestone,
}

impl DistributionKind {
    pub const SPACE: usize = 1;
}
