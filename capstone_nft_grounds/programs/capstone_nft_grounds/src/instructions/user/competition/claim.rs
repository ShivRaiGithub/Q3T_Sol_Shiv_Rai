use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token};
use crate::{state::UserAccount, Ranking, StakeAccount};

#[derive(Accounts)]
pub struct Claim<'info>{
#[account(mut)]
pub user: Signer<'info>,

#[account(
    mut,
    seeds=[b"user".as_ref(), user.key().as_ref()],
    bump,
)]
pub user_account: Account<'info, UserAccount>,

#[account(
    seeds = [b"stake".as_ref(), user.key().as_ref()],
    bump,
)]
pub stake_account: Account<'info, StakeAccount>,
#[account(
    seeds=[b"ranking",ranking.admin.key().as_ref()],
    bump,
)]
ranking: Account<'info, Ranking>,

pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
pub token_program: Program<'info, Token>,
}

impl<'info>Claim<'info>{
    pub fn claim(&mut self)->Result<()>{
        // 1 Point to all voters
        if self.user_account.voted == true{
            self.user_account. points += 1;
            self.user_account.voted = false;
        }
        
        // Assign extra points to the top 3 users
        let user_key = self.user.key();

        if user_key ==  self.ranking.first {
            self.user_account.points += 5;
        } else if user_key == self.ranking.second {
            self.user_account.points += 4;
        } else if user_key == self.ranking.third {
            self.user_account.points += 3;
        }

        Ok(())

    }

}

