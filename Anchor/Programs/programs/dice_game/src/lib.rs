pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("DEcMDjGz9FMS9bbtHm1CRLP1mYaicvkeUXG6GxBK5jos");

#[program]
pub mod dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount:u64) -> Result<()> {
        ctx.accounts.init(amount)?;
        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed:u64, amount:u64, roll:u8) -> Result<()> {
        ctx.accounts.create_bet(seed,&ctx.bumps,amount, roll)?;
        ctx.accounts.deposit(amount)?;
        Ok(())
    }

}
