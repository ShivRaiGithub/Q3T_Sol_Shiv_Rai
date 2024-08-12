use anchor_lang::prelude::*;

use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, CloseAccount, close_account}};

use crate::Escrow;


#[derive(Accounts)]
pub struct Take <'info>
{
    #[account(mut)]
    taker: Signer<'info>,
    #[account(mut)]
    maker: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program
    )]
    mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::Token = token_program
    )]
    taker_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::Token = token_program
    )]
    taker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::Token = token_program
    )]
    maker_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close=taker,
        seeds = [b"escrow",maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::Token = token_program
    )]
    vault: InterfaceAccount<'info, TokenAccount>,
    associated_token_a: Account<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}



imp<'info> Take <'info>{
pub fn withdraw_and_close(&mut self)->Result<()>{
    let seed = self.escrow.seed.to_le_bytes();
    let bump = [self.escrow.bump];
    let signer_seeds = [&[b"escrow"], self.maker.to_account_info().key().as_ref(),&seed.as_ref() , &bump][..];

    let accounts: TransferChecked = TransferChecked{
        from: self.vault.to_account_info(),
        mint: self.mint_a.to_account_info(),
        to: self.taker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
    };
    
    let ctx: CpiContext<TransferChecked>=CpiContext::new_with_signer(
        program: self.token_program.to_account_info(),
        accounts,
        signer_seeds
    );

    transfer_checked(ctx, self.vault.amount,self.mint_a.decimals)?;

    let accounts = CloseAccount{
        account: self.vault.to_account_info(),
        destination: self.taker.to_account_info(),
        authority: self.escrow.to_account_info(),
    };

    let ctx = CpiContext::new_with_signer(
        program: self.token_program.to_account_info(),
        accounts,
        &signer_seeds
    );

    close_account(ctx)
};

pub fn transfer_to_maker(&self )->Result<()>{
    let accounts: TransferChecked = TransferChecked{
        from: self.taker_ata_a.to_account_info(),
        mint: self.mint_b.to_account_info(),
        to: self.maker_ata_b.to_account_info(),
        authority: self.taker.to_account_info(),
    };
    
    let ctx: CpiContext<TransferChecked>=CpiContext::new(
        program: self.token_program.to_account_info(),
        accounts
    );

    transfer_checked(ctx, self.escrow.receive,self.mint_b.decimals)?;


    Ok(())
};

} 