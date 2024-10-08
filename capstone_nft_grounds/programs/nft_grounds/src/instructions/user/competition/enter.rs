use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata::instructions::
{FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts}, MasterEditionAccount,
Metadata, MetadataAccount}, token::{approve, Approve, Mint, Token, TokenAccount}};

use crate::state:: {UserAccount, StakeAccount,Competition};
use crate::error::{UserError,CompetitionError};

#[derive(Accounts)]
pub struct EnterCompetition<'info>{
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds=[b"user".as_ref(), user.key().as_ref()],
        bump=user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    
    #[account(
        seeds=[b"competition",competition.number.to_le_bytes().as_ref(),competition.admin.key().as_ref()],
        bump
    )]
    pub competition: Box<Account<'info, Competition>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub mint_ata: Account<'info, TokenAccount>,
    #[account(
        seeds= [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata:Account<'info, MetadataAccount>,
    
    #[account(
        seeds= [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
            ],
            seeds::program = metadata_program.key(),
            bump,
        )]
    pub edition:Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"stake".as_ref(), user.key().as_ref()],
        bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program:Program<'info, Metadata>,
}

impl<'info> EnterCompetition<'info>{

    pub fn enter(&mut self) -> Result<()> {
        require!(self.competition.can_register==true, CompetitionError::CantRegister);
        require!(self.user_account.nft_in_competition==false, UserError::NftInCompetition);
        require!(self.user_account.paid_entry_fees==true, UserError::FeesNotPaid);
        // set the user account to be in competition
        self.user_account.nft_in_competition=true;
        self.user_account.paid_entry_fees=false;

        // Approve permission to stake account
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = Approve {
                to: self.mint_ata.to_account_info(),
                delegate: self.stake_account.to_account_info(),
                authority: self.user.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        // Freeze the delegated account
        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts{
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        ).invoke()?;
        self.stake_account.mint = self.mint.key();
        self.stake_account.votes = 0;


        Ok(())
    }
}