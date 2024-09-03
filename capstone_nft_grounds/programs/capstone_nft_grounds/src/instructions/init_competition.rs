use anchor_lang::prelude::*;

use crate::state::Competition;

#[derive(Accounts)]
pub struct InitializeCompetition<'info>{
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space = Competition::INIT_SPACE,
        seeds=[b"competition",admin.key().as_ref()],
        bump,
    )]
    competition: Account<'info, Competition>,

    system_program: Program<'info, System>

}

impl<'info>InitializeCompetition<'info>{
    pub fn init_competition(
        &mut self,
        bumps: &InitializeCompetitionBumps )->Result<()>{
    
    self.competition.set_inner(Competition{
    admin:self.admin.key(),
    can_register:false,
    can_vote:false,
    can_claim:false,
    bump:bumps.competition,
    });
    Ok(())
    
}

}