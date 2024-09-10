use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked},
    associated_token::AssociatedToken
};
use crate::{state::{Listing, Marketplace}, UserAccount};
use crate::error::UserError;

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds=[b"marketplace",marketplace.admin.key().as_ref()],
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
    init,
    payer=maker,
    space=Listing::INIT_SPACE,
    seeds=[b"listing",marketplace.key().as_ref(), maker_mint.key().as_ref()],
    bump,
)]
    listing:Box<Account<'info, Listing>>,
    #[account(
        init,
        payer=maker,
        associated_token::authority=listing,
        associated_token::mint=maker_mint,
    )]
    vault: Box<InterfaceAccount<'info, TokenAccount>>,

    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,

}

impl <'info> List<'info> {
    pub fn create_list(&mut self, price: u64, bumps: &ListBumps)->Result<()>{
        require!(self.user_account.nft_in_market == false, UserError::NftInMarket);
        // Update user account
        self.user_account.nft_in_market = true;
        // Create Listing
        self.listing.set_inner(Listing{
            maker:self.maker.key(),
            mint:self.maker_mint.key(),
            price,
            bump:bumps.listing
        });
        // Transfer NFT to vault
        let account = TransferChecked{
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.maker_mint.to_account_info(),
        };
        let cpi_ctx= CpiContext::new(self.token_program.to_account_info(), account);
        transfer_checked(cpi_ctx, 1, self.maker_mint.decimals)
    }
}