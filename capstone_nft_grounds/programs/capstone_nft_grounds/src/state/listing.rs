use anchor_lang::prelude::*;

// User lists their NFT for sale on the market
#[account]
pub struct Listing{
    pub maker: Pubkey, // User
    pub mint: Pubkey, // NFT Mint
    pub price: u64, // Price in points
    pub bump: u8 // Bump
}

impl Space for Listing {
    const INIT_SPACE:usize = 8+32*2+8+1;
}