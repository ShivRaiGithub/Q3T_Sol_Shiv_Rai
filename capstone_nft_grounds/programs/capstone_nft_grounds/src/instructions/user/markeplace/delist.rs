use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use crate::{state::{Listing,Marketplace}, UserAccount};
use crate::error::UserError;

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds=[b"marketplace",marketplace.admin.as_ref()],
        bump=marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), maker.key().as_ref()],
        bump=user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    maker_mint:Box<InterfaceAccount<'info, Mint>>,
    #[account(
        mut,
        associated_token::authority=maker,
        associated_token::mint=maker_mint,
    )]
    maker_ata:Box<InterfaceAccount<'info, TokenAccount>>,

#[account(
    mut,
    close = maker,
    seeds=[marketplace.key().as_ref(), maker_mint.key().as_ref()],
    bump,
)]
    listing:Box<Account<'info, Listing>>,
    #[account(
        mut,
        associated_token::authority=listing,
        associated_token::mint=maker_mint,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,

    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,

}

impl <'info> Delist<'info> {

    pub fn withdraw_nft(&mut self) ->Result<()>{
        require!(self.user_account.nft_in_market==true,UserError::NftNotInMarket);
        // Update user account
        self.user_account.nft_in_market = false;

        let seeds=&[
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let account = TransferChecked{
            from: self.vault.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };
        let cpi_ctx= CpiContext::new_with_signer(self.token_program.to_account_info(), account,signer_seeds);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals) 
    }
}