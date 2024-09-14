use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
// use state::*;
// use error::*;

declare_id!("75kf2v6x7kSwVjiNbf4qQGjF4jkiLHyjwMmqeK3RWFWs");



#[program]
pub mod nft_stake {

    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.init_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }
    pub fn initialize_account(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)?;
        Ok(())
    }
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }
}

