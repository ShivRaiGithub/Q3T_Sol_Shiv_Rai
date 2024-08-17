use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::{ Mint, TokenAccount, TransferChecked, MintTo, transfer_checked, mint_to }
};
use crate::state::Config;

#[derive(Accounts)]

#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    mint_y: Box<InterfaceAccount<'info, Mint>>,
    mint_x: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = maker,
        space =8 + Config::INIT_SPACE,
        seeds = [b"amm", mint_x.key().as_ref(), mint_y.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump
    )]
    config: Account<'info, Config>,
    #[account(
        init_if_needed,
        payer = maker,
        mint::authority = config,
        mint::decimals = 6,
        mint::token_program = token_program,
        seeds=[b"mint",config.key().as_ref()],
        bump,
    )]
    mint_lp: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    vault_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    vault_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_x,
        associated_token::authority = maker,
    )]
    maker_ata_x: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_y,
        associated_token::authority = maker,
    )]
    maker_ata_y: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_lp,
        associated_token::authority = maker,
    )]
    maker_ata_lp: InterfaceAccount<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

impl <'info> Initialize<'info> {
    pub fn save_config ( &mut self, seed: u64, fee: u16, mint_bump: u8 ) -> Result<()> {
        self.config.set_inner(Config{
            seed,
            fee,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            lp_bump: mint_bump,
            bump: mint_bump,
        });
        Ok(())
    }

    pub fn deposit(&mut self, amount_x: u64, is_x: bool) -> Result<()> {
        let(from, to, mint, decimals) = match is_x {
            true => (self.maker_ata_x.to_account_info(), self.vault_x.to_account_info(), self.mint_x.to_account_info(), self.mint_x.decimals),
            false => (self.maker_ata_y.to_account_info(), self.vault_y.to_account_info(), self.mint_y.to_account_info(), self.mint_y.decimals),
        };
        let accounts = TransferChecked{
            from ,
            mint: mint.to_account_info(),
            to,
            authority: self.maker.to_account_info(),
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(),accounts);
        transfer_checked(ctx, amount_x,decimals)
    }

    pub fn mint_lp_tokens(&mut self, amount_x: u64, amount_y: u64) -> Result<()> {
        let amount = amount_x.checked_mul(amount_y).ok_or(ProgramError::ArithmeticOverflow)?;
    
        let mint_x_key = self.mint_x.to_account_info().key;
        let mint_y_key = self.mint_y.to_account_info().key;
        let config_seed = self.config.seed.to_le_bytes();
        let bump = self.config.bump;
    
        // Create a longer-lived value for signer_seeds
        let signer_seeds: &[&[_]] = &[
            b"amm", 
            mint_x_key.as_ref(),
            mint_y_key.as_ref(),
            config_seed.as_ref(),
            &[bump],
        ];
    
        let accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.maker_ata_lp.to_account_info(),
            authority: self.config.to_account_info(),
        };
        
       
        let binding = &[signer_seeds];
    
        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            binding, // Use the binding here
        );
    
        mint_to(ctx, amount)
    }
    
    
}