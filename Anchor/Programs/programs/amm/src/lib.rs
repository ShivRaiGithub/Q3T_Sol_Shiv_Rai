use anchor_lang::prelude::*;
pub mod context;
pub use context::*;
pub mod state;
pub use state::*;



declare_id!("26QPuoVwHXNWSRsuNj1TJ6M2mRsp1kisBoYKK62VEBkF");
#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, fee: u16, amount_x: u64,amount_y: u64 ) -> Result<()> {
        ctx.accounts.save_config(seed,fee,ctx.bumps.config);
        ctx.accounts.deposit(amount_x, true)?;
        ctx.accounts.deposit(amount_y, false)?;
        ctx.accounts.mint_lp_tokens(amount_x, amount_y)?;
        Ok(())
    }

    // pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y:u64) -> Result<()> {
    //     Ok(())
    // }
    // pub fn withdraw(ctx: Context<Withdraw>, amount: u64, min_x: u64, min_y:u64) -> Result<()> {
    //     Ok(())
    // }
    // pub fn swap(ctx: Context<Swap>, amount: u64, min_receive: u64, is_x:bool) -> Result<()> {
    //     Ok(())
    // }
}

