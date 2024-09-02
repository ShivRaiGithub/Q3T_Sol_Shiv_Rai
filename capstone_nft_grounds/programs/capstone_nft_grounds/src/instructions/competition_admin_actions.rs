use anchor_lang::prelude::*;

use crate::state::Competition;

#[derive(Accounts)]
pub struct CompetitionActions<'info>{
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

impl<'info>CompetitionActions<'info>{
    pub fn start_centry( &mut self )->Result<()>{
    self.competition.can_register = true;

    Ok(())
    
}
    pub fn start_competition( &mut self )->Result<()>{
    self.competition.can_register=false;
    self.competition.can_vote=true;
    Ok(())
    
}
    pub fn stop_competition( &mut self )->Result<()>{
    self.competition.can_vote=false;

    // Add rewards giving part
    Ok(())
    
}

}