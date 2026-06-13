use crate::{
    error::VestingError,
    state::stream::{Stream, VestingType},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct SetMilestone<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        constraint = creator.key() == stream.creator @ VestingError::Unauthorized,
        seeds = [
            b"stream",
            creator.key().as_ref(),
            stream.recipient.as_ref(),
            &stream.stream_id.to_le_bytes(),
        ],
        bump = stream.bump,
    )]
    pub stream: Account<'info, Stream>,
}

pub fn set_milestone_handler(ctx: Context<SetMilestone>) -> Result<()> {
    let creator_key = ctx.accounts.creator.key();
    let stream = &mut ctx.accounts.stream;

    require_keys_eq!(creator_key, stream.creator, VestingError::Unauthorized);
    require!(
        stream.vesting_type == VestingType::Milestone,
        VestingError::NotMilestoneStream
    );
    require!(!stream.canceled, VestingError::AlreadyCancelled);
    require!(!stream.milestone_reached, VestingError::FullyVested);

    stream.milestone_reached = true;

    Ok(())
}
