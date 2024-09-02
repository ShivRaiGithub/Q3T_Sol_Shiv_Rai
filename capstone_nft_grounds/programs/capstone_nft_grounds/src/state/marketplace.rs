use anchor_lang::prelude::*;

// Marketplace Account
#[account]
pub struct Marketplace{
    pub admin: Pubkey,  // Admin key
    pub fee: u16,
    pub bump: u8, // Bump
    // pub rewards_bump: u8,
    // pub treasury_bump: u8,
}

impl Space for Marketplace{
    const INIT_SPACE:usize = 8+32+2+1;
}