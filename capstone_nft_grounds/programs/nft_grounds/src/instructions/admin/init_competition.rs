use anchor_lang::prelude::*;

use crate::state::Competition;

#[derive(Accounts)]
#[instruction(num: u64)]
pub struct InitializeCompetition<'info>{
    #[account(mut)]
    admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        space = Competition::INIT_SPACE,
        seeds=[b"competition",num.to_le_bytes().as_ref(),admin.key().as_ref()],
        bump,
    )]
    competition: Account<'info, Competition>,
    system_program: Program<'info, System>

}

impl<'info>InitializeCompetition<'info>{
    pub fn init_competition(
        &mut self,
        num:u64,
        fee: u16,
        bumps: &InitializeCompetitionBumps )->Result<()>{
    
    self.competition.set_inner(Competition{
    admin:self.admin.key(),
    number:num,
    can_register:false,
    can_vote:false,
    can_claim:false,
    fee,
    bump:bumps.competition,
    });
    Ok(())
    
}

}