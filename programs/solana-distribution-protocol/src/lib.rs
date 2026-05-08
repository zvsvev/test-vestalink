use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("6a32KtC9vdynb89TRSXxPe369nMnsZW7MuKAdxJ9iNT7");

#[program]
pub mod solana_distribution_protocol {
    use super::*;

    pub fn create_stream(context: Context<CreateStream>, parameters: CreateStreamParameters) -> Result<()> {
        instructions::create_stream(context, parameters)
    }

    pub fn withdraw(context: Context<Withdraw>) -> Result<()> {
        instructions::withdraw(context)
    }

    pub fn cancel(context: Context<Cancel>) -> Result<()> {
        instructions::cancel(context)
    }
}
