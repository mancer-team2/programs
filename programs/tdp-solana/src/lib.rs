pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("BiwY71TrdBzgv2yfa6KfUxUMY8UCpeiUMGnwmCMTsfs9");

#[program]
pub mod tdp_solana {
    use super::*;

    pub fn create_stream(
        ctx: Context<CreateStream>,
        stream_id: u64,
        recipient: Pubkey,
        total_amount: u64,
        start_time: i64,
        cliff_time: i64,
        end_time: i64,
        cancelable: bool,
        vesting_type: VestingType,
        milestone_time: i64,
    ) -> Result<()> {
        instructions::create_stream::create_stream_handler(
            ctx,
            stream_id,
            recipient,
            total_amount,
            start_time,
            cliff_time,
            end_time,
            cancelable,
            vesting_type,
            milestone_time,
        )
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        instructions::withdraw::withdraw_handler(ctx)
    }

    pub fn cancel_stream(ctx: Context<CancelStream>) -> Result<()> {
        instructions::cancel_stream::cancel_stream_handler(ctx)
    }

    pub fn close_stream(ctx: Context<CloseStream>) -> Result<()> {
        instructions::close_stream::close_stream_handler(ctx)
    }

    pub fn set_milestone(ctx: Context<SetMilestone>) -> Result<()> {
        instructions::set_milestone::set_milestone_handler(ctx)
    }
}
