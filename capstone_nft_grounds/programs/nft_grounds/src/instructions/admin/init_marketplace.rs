use anchor_lang::prelude::*;

use crate::state::Marketplace;

#[derive(Accounts)]
pub struct InitializeMarketplace<'info>{
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer=admin,
        space = Marketplace::INIT_SPACE,
        seeds=[b"marketplace",admin.key().as_ref()],
        bump,
    )]
    marketplace: Account<'info, Marketplace>,
    system_program: Program<'info, System>,
}

impl<'info>InitializeMarketplace<'info>{
    pub fn init_marketplace(
        &mut self, 
        bumps: &InitializeMarketplaceBumps )->Result<()>{
    self.marketplace.set_inner(Marketplace{
    admin:self.admin.key(),
    bump:bumps.marketplace,
    });
    Ok(())
    
}

}