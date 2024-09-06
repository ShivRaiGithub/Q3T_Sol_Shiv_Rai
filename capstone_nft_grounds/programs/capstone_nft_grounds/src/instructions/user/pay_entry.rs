use anchor_lang::{
    prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}
};

use crate::{Competition, UserAccount};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct PayEntry<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    /// CHECK: This is safe because we are only transferring funds to the admin's public key.
    pub admin: UncheckedAccount<'info>,  // Admin is just a public key, not a data account

    #[account(
        seeds = [b"marketplace", name.as_bytes()],
        bump
    )]
    pub competition: Box<Account<'info, Competition>>,

    pub system_program: Program<'info, System>,
}

impl<'info> PayEntry<'info> {
    // Make payment for entry fees
    pub fn pay_entry_fees(&mut self) -> Result<()> {
        // Transfer amount
        let accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.admin.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        let amount = LAMPORTS_PER_SOL * self.competition.fee as u64;
        transfer(cpi_ctx, amount)?;

        // Set user account to have paid entry fees
        self.user_account.paid_entry_fees = true;

        Ok(())
    }
}
