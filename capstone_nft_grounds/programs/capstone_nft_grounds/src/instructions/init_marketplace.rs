use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface };

use crate::state::Marketplace;
// use crate::error::MarketplaceError;


#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info>{
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer=admin,
        space = Marketplace::INIT_SPACE,
        seeds=[b"marketplace",name.as_str().as_bytes()],
        bump,
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        seeds=[b"rewards",marketplace.key().as_ref()],
        payer=admin,
        bump,
        mint::decimals=6,
        mint::authority=marketplace,
    )]
    rewards_mint:InterfaceAccount<'info,Mint>,
    #[account(
        seeds=[b"treasury",marketplace.key().as_ref()],
        bump,
    )]  
    treasury: SystemAccount<'info>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,

}

impl<'info>InitializeMarketplace<'info>{
    pub fn initialize_marketplace(
        &mut self, 
        fee: u16,
        bumps: &InitializeMarketplaceBumps )->Result<()>{
    self.marketplace.set_inner(Marketplace{
    admin:self.admin.key(),
    fee,
    bump:bumps.marketplace,

    });
    Ok(())
    
}

}