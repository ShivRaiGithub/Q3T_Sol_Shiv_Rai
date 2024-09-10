use anchor_lang::error_code;

#[error_code]
pub enum CompetitionError{
    #[msg("Cannot Register right now")]
    CantRegister,
    #[msg("Cannot Vote right now")]
    CantVote,
    #[msg("Cannot Claim right now")]
    CantClaim
}

#[error_code]
pub enum UserError{
    #[msg("Entry Fees already paid")]
    FeesPaid,
    #[msg("NFT already in competition")]
    NftInCompetition,
    #[msg("NFT already in market")]
    NftInMarket,
    #[msg("Voted already")]
    CantClaim
}