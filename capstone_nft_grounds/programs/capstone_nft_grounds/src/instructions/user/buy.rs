use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{Listing, Marketplace, UserAccount};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Buy<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        seeds=[b"user".as_ref(), maker.key().as_ref()],
        bump,
    )]
    pub maker_account: Account<'info, UserAccount>,
    #[account(
        seeds=[b"user".as_ref(), taker.key().as_ref()],
        bump,
    )]
    pub taker_account: Account<'info, UserAccount>,


    #[account(
        seeds=[b"marketplace", name.as_bytes()],
        bump
    )]
    pub marketplace: Box<Account<'info, Marketplace>>,
    pub maker_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = taker,
    )]
    pub taker_ata: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        close = maker,
        seeds=[b"listing", maker_mint.key().as_ref()],
        bump,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        mut,
        associated_token::mint=maker_mint,
        associated_token::authority=listing,
        associated_token::token_program=token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,


    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Buy<'info> {
    pub fn buy(&mut self) -> Result<()> {
        self.maker_account.points-=self.listing.price;
        self.taker_account.points+=self.listing.price;
        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        let signer_seeds = &[
            b"stake",
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);

        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)?;

        // close account
        let signer_seeds = &[
            self.marketplace.to_account_info().key.as_ref(),
            self.maker_mint.to_account_info().key.as_ref(),
            &[self.listing.bump],
        ];
        let signer = &[&signer_seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, signer);

        close_account(cpi_ctx)
    }
}