use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode{
    #[msg("Maximum number of stakes reached")]
    MaxStakes,
    #[msg("Staking has not matured")]
    UnstakeFreezeDurationInvalid,
}
