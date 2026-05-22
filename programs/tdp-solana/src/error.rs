use anchor_lang::prelude::*;

#[error_code]
pub enum VestingError {
    #[msg("Amount must be greater than zero")]
    InvalidAmount,

    #[msg("Recipient cannot be the default pubkey")]
    InvalidRecipient,

    #[msg("start_time must be before end_time")]
    InvalidSchedule,

    #[msg("cliff_time must be between start_time and end_time")]
    InvalidCliff,

    #[msg("Cliff period has not been reached yet")]
    CliffNotReached,

    #[msg("No tokens available to withdraw")]
    NothingToWithdraw,

    #[msg("Signer is not authorized for this action")]
    Unauthorized,

    #[msg("Stream has already been canceled")]
    AlreadyCancelled,

    #[msg("Stream is not cancelable")]
    StreamNotCancelable,

    #[msg("Stream is already fully vested")]
    FullyVested,

    #[msg("Stream is not configured for milestone unlocking")]
    NotMilestoneStream,

    #[msg("Stream schedule has already ended")]
    StreamExpired,

    #[msg("Token account mint does not match stream mint")]
    InvalidTokenAccount,

    #[msg("Insufficient token balance to fund stream")]
    InsufficientFunds,

    #[msg("PDA derivation does not match expected address")]
    InvalidPda,

    #[msg("Arithmetic overflow or underflow")]
    MathOverflow,
}
