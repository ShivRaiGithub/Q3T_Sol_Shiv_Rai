use anchor_lang::prelude::*;

use crate::state:: {UserAccount, StakeAccount, Ranking};

#[derive(Accounts)]
pub struct Vote<'info>{
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump=user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"stake".as_ref(), stake_account.owner.key().as_ref()],
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds=[b"ranking",ranking.admin.key().as_ref()],
        bump=ranking.bump,
    )]
    ranking: Account<'info, Ranking>,

}

impl<'info> Vote<'info>{
   pub fn vote(&mut self) -> Result<()> {
    // Mark user as having voted
    self.user_account.voted = true;

    // Increase votes for the contestant's stake account
    self.stake_account.votes += 1;

    let contestant_votes = self.stake_account.votes;
    let contestant_key = self.stake_account.owner.key();

    // Adjust ranking based on the new vote count of the contestant
    if contestant_votes > self.ranking.firstVotes {
        // Move down previous rankings
        self.ranking.third = self.ranking.second;
        self.ranking.thirdVotes = self.ranking.secondVotes;

        self.ranking.second = self.ranking.first;
        self.ranking.secondVotes = self.ranking.firstVotes;

        // Update first place with current contestant
        self.ranking.first = contestant_key;
        self.ranking.firstVotes = contestant_votes;
    } else if contestant_votes > self.ranking.secondVotes {
        // Move down previous second place to third
        self.ranking.third = self.ranking.second;
        self.ranking.thirdVotes = self.ranking.secondVotes;

        // Update second place with current contestant
        self.ranking.second = contestant_key;
        self.ranking.secondVotes = contestant_votes;
    } else if contestant_votes > self.ranking.thirdVotes {
        // Update third place with current contestant
        self.ranking.third = contestant_key;
        self.ranking.thirdVotes = contestant_votes;
    }

    Ok(())
}

}