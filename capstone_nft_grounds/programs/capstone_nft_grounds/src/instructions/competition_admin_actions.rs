use anchor_lang::prelude::*;

use crate::state::Competition;

#[derive(Accounts)]
pub struct CompetitionActions<'info>{
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        seeds=[b"competition",admin.key().as_ref()],
        bump,
    )]
    competition: Account<'info, Competition>,

    system_program: Program<'info, System>

}

impl<'info>CompetitionActions<'info>{
    // allow entries
    pub fn start_centry( &mut self )->Result<()>{
    self.competition.can_register = true;
    Ok(())
    
}
    // stop entries
    pub fn start_competition( &mut self )->Result<()>{
    self.competition.can_register=false;
    self.competition.can_vote=true;
    Ok(())
    
}
    // stop competition
    pub fn stop_competition_start_claim( &mut self )->Result<()>{
    self.competition.can_vote=false;
    self.competition.can_claim=true;
    Ok(())
    }

    pub fn stop_claim( &mut self )->Result<()>{
        self.competition.can_claim=false;
        // TODO
        // give rewards to winners
        Ok(())
    
}

}