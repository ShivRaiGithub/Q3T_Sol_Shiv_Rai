use anchor_lang::prelude::*;

use crate::state::Ranking;

#[derive(Accounts)]
pub struct InitializeRanking<'info>{
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer=admin,
        space = Ranking::INIT_SPACE,
        seeds=[b"ranking",admin.key().as_ref()],
        bump,
    )]
    ranking: Account<'info, Ranking>,
    system_program: Program<'info, System>,
}

impl<'info>InitializeRanking<'info>{
    pub fn init_ranking(
        &mut self, 
        bumps: &InitializeRankingBumps)->Result<()>{

        let zero_bytes = [0u8; 32];
        let zero_pubkey = Pubkey::from(zero_bytes);


    self.ranking.set_inner(Ranking{
        first: zero_pubkey,
        firstVotes: 0,
        second: zero_pubkey,
        secondVotes: 0,
        third: zero_pubkey,
        thirdVotes: 0,
        hon1: zero_pubkey,
        hon1Votes: 0,
        hon2: zero_pubkey,
        hon2Votes: 0,
        bump: bumps.ranking,
    });
    Ok(())
    
}

}