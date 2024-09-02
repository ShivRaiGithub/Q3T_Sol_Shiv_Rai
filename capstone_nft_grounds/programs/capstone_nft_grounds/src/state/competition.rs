use anchor_lang::prelude::*;
// Competition account
#[account]
pub struct Competition{
    pub admin: Pubkey, // Admin key
    pub can_register: bool, // Users can register or not
    pub can_vote: bool, // Users can vote or not
    pub bump: u8 // bump
}

impl Space for Competition {
    const INIT_SPACE:usize = 8+32+1+1+1;
}