use anchor_lang::prelude::*;

#[account]
pub struct Ranking{
    pub first: Pubkey,
    pub firstVotes: u64,
    pub second: Pubkey,
    pub secondVotes: u64,
    pub third: Pubkey,
    pub thirdVotes: u64,
    pub hon1: Pubkey,
    pub hon1Votes: u64,
    pub hon2: Pubkey,
    pub hon2Votes: u64,
    pub bump: u8, // Bump
}

impl Space for Ranking {
    const INIT_SPACE:usize = 8+32*5 + 5*8+ 1;
}