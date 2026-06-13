use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum VestingType {
    Cliff,
    Linear,
    Milestone,
}

#[account]
#[derive(InitSpace)]
pub struct Stream {
    /// Wallet that created the stream and funded the escrow.
    pub creator: Pubkey,
    /// Wallet entitled to withdraw vested tokens from the stream.
    pub recipient: Pubkey,
    /// SPL token mint being vested.
    pub mint: Pubkey,
    /// Escrow token account (PDA-owned) holding the locked tokens.
    pub escrow_token_account: Pubkey,
    /// Monotonic identifier scoped to the creator, used for PDA derivation.
    pub stream_id: u64,
    /// Total token amount originally deposited into the stream.
    pub total_amount: u64,
    /// Cumulative amount the recipient has already withdrawn.
    pub withdrawn_amount: u64,
    /// Unix timestamp at which vesting begins.
    pub start_time: i64,
    /// Unix timestamp before which no tokens are claimable.
    pub cliff_time: i64,
    /// Unix timestamp at which vesting completes (100% unlocked).
    pub end_time: i64,
    /// Whether the creator is allowed to cancel the stream.
    pub cancelable: bool,
    /// Whether the stream has been canceled.
    pub canceled: bool,
    /// Vesting rule used to calculate the unlocked amount.
    pub vesting_type: VestingType,
    /// Whether the creator has marked the milestone as reached (only meaningful for milestone vesting).
    pub milestone_reached: bool,
    /// Unix timestamp before which a milestone cannot unlock tokens.
    pub milestone_time: i64,
    /// Bump seed for the Stream account PDA.
    pub bump: u8,
    /// Bump seed for the escrow token account PDA.
    pub escrow_bump: u8,
    /// Unix timestamp at which the stream account was created.
    pub created_at: i64,
}
