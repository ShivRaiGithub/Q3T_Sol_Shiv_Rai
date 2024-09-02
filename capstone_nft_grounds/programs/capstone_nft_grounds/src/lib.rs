
declare_id!("DuqczLjqGbTENegYpNSrT8j4PWNdqLetcQH5KGiM8wdo");

use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;
use error::*;


#[program]
pub mod nft_grounds {

    use super::*;

    // ADMIN FUNCTIONS
    
    // Initialize marketplace
    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee:u16) -> Result<()> {
        ctx.accounts.initialize_marketplace(fee,&ctx.bumps)?;
        Ok(())
    }
    
    // Initialize compeition
    pub fn initialize_competition(ctx: Context<InitializeCompetition>) -> Result<()> {
        ctx.accounts.init_competition(&ctx.bumps)?;
        Ok(())
    }



    // Start a competition entry of users
    pub fn start_entry(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.start_centry();
        Ok(())
    }
    
    // start compeition
    pub fn start_competition(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.start_competition();
        Ok(())
    }
    
    // Stop a competition
    pub fn stop_competition(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.stop_competition();
        Ok(())
    }
    
    
    // USER FUNCTIONS

    pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<()> {
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

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_list(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }
}
