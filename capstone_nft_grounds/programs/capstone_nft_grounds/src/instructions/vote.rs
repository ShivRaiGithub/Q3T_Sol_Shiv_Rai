use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state:: {UserAccount, StakeAccount};


#[derive(Accounts)]
pub struct Vote<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump=user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    pub mint: Account<'info, Mint>,
    #[account(
        seeds = [b"stake".as_ref(), mint.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

}

impl<'info> Vote<'info>{
    pub fn vote(&mut self) -> Result<()> {
        self.user_account.voted = true;
        self.stake_account.votes += 1;
        Ok(())
    }
}