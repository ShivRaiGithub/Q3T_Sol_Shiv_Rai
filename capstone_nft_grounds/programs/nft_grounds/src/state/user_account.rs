use anchor_lang::prelude::*;

// User account
#[account]
pub struct UserAccount{
    pub points: u64, // Points balance
    pub nft_in_market: bool, // NFT in market
    pub paid_entry_fees: bool, // entry fees for competition
    pub nft_in_competition: bool, // NFT in competition
    pub voted: bool, // Voted in competition
    pub bump: u8, // Bump
}

impl Space for UserAccount{
    const INIT_SPACE: usize = 8 + 8 + 1  + 1 + 1 + 1;
}