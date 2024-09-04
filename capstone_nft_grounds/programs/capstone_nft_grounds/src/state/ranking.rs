use anchor_lang::prelude::*;

#[account]
pub struct Ranking{
    pub first: Pubkey,
    pub second: Pubkey,
    pub third: Pubkey,
    pub hon1: Pubkey,
    pub hon2: Pubkey,
    pub bump: u8, // Bump

}

impl Space for Ranking {
    const INIT_SPACE:usize = 8+32*5 + 1;
}