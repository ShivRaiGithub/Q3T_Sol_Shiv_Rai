
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
    ////////////////////////////////////////
    // ADMIN FUNCTIONS
    ////////////////////////////////////////
    
    // Initialize marketplace
    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee:u16) -> Result<()> {
        ctx.accounts.init_marketplace(fee,&ctx.bumps)?;
        Ok(())
    }
    
    // Initialize competition
    pub fn initialize_competition(ctx: Context<InitializeCompetition>) -> Result<()> {
        ctx.accounts.init_competition(&ctx.bumps)?;
        Ok(())
    }
    // Initialize ranking
    pub fn initialize_ranking(ctx: Context<InitializeRanking>) -> Result<()> {
        ctx.accounts.init_ranking(&ctx.bumps)?;
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
        ctx.accounts.stop_competition_start_claim();
        Ok(())
    }

    // Stop period where users claim their rewards
    pub fn end_rewards_period(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.stop_claim();
        Ok(())
    }



    ////////////////////////////////////////
    // USER FUNCTIONS
    ////////////////////////////////////////


    // HAVE TO WORK FROM HERE

    pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)?;
        Ok(())
    }

    // Competition functions

    // user can enter
    pub fn enter_competition(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    // user can vote
    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        ctx.accounts.vote()?;
        Ok(())
    }

    // user claims points ( 1 per vote, rest per ranking)
    pub fn claim_points(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }


    // Marketplace

    // user can list their nft
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_list(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    // user can delist
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

        // user can buy
    pub fn buy(ctx: Context<Delist>) -> Result<()> {
    ctx.accounts.withdraw_nft()
    }
}
