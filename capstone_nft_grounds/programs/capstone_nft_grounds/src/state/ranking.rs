use anchor_lang::prelude::*;

#[account]
pub struct Ranking{
    pub admin : Pubkey,
    pub first: Pubkey,
    pub firstVotes: u64,
    pub second: Pubkey,
    pub secondVotes: u64,
    pub third: Pubkey,
    pub thirdVotes: u64,
    pub bump: u8, // Bump
}

impl Space for Ranking {
    const INIT_SPACE:usize = 8+32*4 + 3*8+ 1;
}