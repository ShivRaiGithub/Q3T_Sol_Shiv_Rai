use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface, TransferChecked, transfer_checked},
    metadata::{Metadata, MetadataAccount, MasterEditionAccount},
    associated_token::AssociatedToken
};
use crate::state::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds=[b"marketplace",marketplace.admin.key().as_ref()],
        bump=marketplace.bump,
    )]
    marketplace: Box<Account<'info, Marketplace>>,
    maker_mint:Box<InterfaceAccount<'info, Mint>>,
    collection_mint:Box<InterfaceAccount<'info, TokenAccount>>,
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
    seeds=[marketplace.key().as_ref(), maker_mint.key().as_ref()],
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

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program=metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref()==collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified==true,

    )]
    metadata: Box<Account<'info, MetadataAccount>>,

    #[account(
        seeds=[
            b"edition",
            metadata_program.key().as_ref(),
            maker_mint.key().as_ref()
        ],
        seeds::program=metadata_program.key(),
        bump
    )]
    master_edition: Box<Account<'info, MasterEditionAccount>>,

    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,

}

impl <'info> List<'info> {
    pub fn create_list(&mut self, price: u64, bumps: &ListBumps)->Result<()>{
        self.listing.set_inner(Listing{
            maker:self.maker.key(),
            mint:self.maker_mint.key(),
            price,
            bump:bumps.listing
        });
        Ok(())
    }

    pub fn deposit_nft(&mut self) ->Result<()>{
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