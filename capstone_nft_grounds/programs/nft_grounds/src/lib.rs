
declare_id!("DuqczLjqGbTENegYpNSrT8j4PWNdqLetcQH5KGiM8wdo");

use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;
use state::*;

#[program]
pub mod nft_grounds {

    use super::*;

    ////////////////////////////////////////
    // ADMIN FUNCTIONS
    ////////////////////////////////////////

    // Initialize marketplace
    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>) -> Result<()> {
        ctx.accounts.init_marketplace(&ctx.bumps)
    }
    
    // Initialize competition
    pub fn initialize_competition(ctx: Context<InitializeCompetition>, num:u64, fee:u16) -> Result<()> {
        ctx.accounts.init_competition(num,fee,&ctx.bumps)
    }
    // Initialize ranking
    pub fn initialize_ranking(ctx: Context<InitializeRanking>) -> Result<()> {
        ctx.accounts.init_ranking(&ctx.bumps)
    }

    // Start a competition entry of users
    pub fn start_entry(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.start_centry()
    }
    
    // start compeition
    pub fn start_competition(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.start_competition()
    }
    
    // Stop a competition
    pub fn stop_competition(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.stop_competition_start_claim()
    }

    // Stop period where users claim their rewards
    pub fn end_rewards_period(ctx: Context<CompetitionActions>) -> Result<()> {
        ctx.accounts.stop_claim()
    }


    ////////////////////////////////////////
    // USER FUNCTIONS
    ////////////////////////////////////////

    
    pub fn initialize_user_account(ctx: Context<InitializeUserAccount>) -> Result<()> {
        ctx.accounts.init_user_account(&ctx.bumps)
    }
    
    // Competition functions

    // user pays entry fee
    pub fn pay_entry(ctx: Context<PayEntry>) -> Result<()> {
        ctx.accounts.pay_entry_fees()
    }
    
    // user can enter
    pub fn enter_competition(ctx: Context<EnterCompetition>) -> Result<()> {
        ctx.accounts.enter()
    }

    // user votes
    pub fn vote(ctx: Context<Vote>) -> Result<()> {
        ctx.accounts.vote()
    }

    // user claims points ( 1 per vote, rest per ranking)
    pub fn claim_points(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
    // user claim back their nft
    pub fn claim_back_nft(ctx: Context<Exit>) -> Result<()> {
        ctx.accounts.exit()
    }

    // Marketplace
    // user can list their nft
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_list(price, &ctx.bumps)
    }

    // user can delist
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }

    // user can buy
    pub fn buy(ctx: Context<Buy>) -> Result<()> {
    ctx.accounts.buy()?;
    ctx.accounts.transfer_nft()
    }
}
