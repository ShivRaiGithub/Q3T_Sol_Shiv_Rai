use anchor_lang::prelude::*;
use crate::state::{Competition,Ranking};

#[derive(Accounts)]
pub struct CompetitionActions<'info>{
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        seeds=[b"competition",admin.key().as_ref()],
        bump,
    )]
    competition: Account<'info, Competition>,

    #[account(
        seeds=[b"ranking",admin.key().as_ref()],
        bump,
    )]
    ranking: Account<'info, Ranking>,

    system_program: Program<'info, System>

}

impl<'info>CompetitionActions<'info>{
    // allow entries
    pub fn start_centry( &mut self )->Result<()>{
    self.competition.can_register = true;
    Ok(())
    
}
    // stop entries, start voting phase
    pub fn start_competition( &mut self )->Result<()>{
    self.competition.can_register=false;
    self.competition.can_vote=true;
    Ok(())
    
}
    // stop competition, start claiming phase
    pub fn stop_competition_start_claim( &mut self )->Result<()>{
    self.competition.can_vote=false;
    self.competition.can_claim=true;
    Ok(())
    }

    // stop claiming phase
    pub fn stop_claim( &mut self )->Result<()>{
        self.competition.can_claim=false;
        Ok(())
    
}

}