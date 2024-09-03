use anchor_lang::prelude::*;

// Marketplace Account
#[account]
pub struct Marketplace{
    pub admin: Pubkey,  // Admin key
    pub fee: u16, // fee to list NFT on market
    pub bump: u8, // Bump
}

impl Space for Marketplace{
    const INIT_SPACE:usize = 8+32+2+1;
}