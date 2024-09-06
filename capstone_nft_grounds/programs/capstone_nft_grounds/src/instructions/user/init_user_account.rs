use anchor_lang::prelude::*;
use crate::{state::UserAccount, StakeAccount};

#[derive(Accounts)]
pub struct InitializeUserAccount<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump,
        space = UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        init,
        payer=user,
        space=StakeAccount::INIT_SPACE,
        // Taking user key in stake instead of mint
        seeds = [b"stake".as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info>InitializeUserAccount<'info>{
    pub fn init_user_account(&mut self, bumps: &InitializeUserAccountBumps) -> Result<()>{
        self.user_account.set_inner(UserAccount{
            points: 0,
            nft_in_market: false,
            paid_entry_fees: false,
            registered: false,
            nft_in_competition: false,
            voted: false,
            bump: bumps.user_account,
        });

        // set stake account
        self.stake_account.owner=self.user.key();
        self.stake_account.bump=bumps.stake_account;


        Ok(())
    }
}