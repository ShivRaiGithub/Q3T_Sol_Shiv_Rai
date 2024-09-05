use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod error;

pub use instructions::*;
pub use state::*;

declare_id!("6Y2pjgcBrVZ19PYsTH5L8iopRZMSaUFUR3YUurMYYHAW");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee:u16, name:String) -> Result<()> {
        ctx.accounts.init(name,fee,&ctx.bumps)?;
        Ok(())
    }
    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_list(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }
    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()
    }
    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.make_payment()?;
        ctx.accounts.transfer_nft()
    }
}
