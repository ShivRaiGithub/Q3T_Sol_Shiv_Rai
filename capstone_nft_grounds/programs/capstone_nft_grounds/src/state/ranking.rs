use anchor_lang::prelude::*;

#[account]
pub struct Ranking{
    pub admin : Pubkey,
    pub first: Pubkey,
    pub first_votes: u64,
    pub second: Pubkey,
    pub second_votes: u64,
    pub third: Pubkey,
    pub third_votes: u64,
    pub bump: u8, // Bump
}

impl Space for Ranking {
    const INIT_SPACE:usize = 8+32*4 + 3*8+ 1;
}