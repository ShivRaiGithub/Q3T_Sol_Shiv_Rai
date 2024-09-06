use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Claim<'info>{
#[account(mut)]
pub user: Signer<'info>,

#[account(
    mut,
    seeds=[b"user".as_ref(), user.key().as_ref()],
    bump=user_account.bump,
)]
pub user_account: Account<'info, UserAccount>,

pub associated_token_program: Program<'info, AssociatedToken>,
pub system_program: Program<'info, System>,
pub token_program: Program<'info, Token>,
}

impl<'info>Claim<'info>{
    pub fn claim(&mut self)->Result<()>{
        if self.user_account.voted == true{
            self.user_account. points += 1;
        }
        
        // TODO assign points based on claimer and reset the required
        Ok(())

    }

}

