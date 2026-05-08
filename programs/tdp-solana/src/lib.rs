pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use instructions::*;
pub use state::*;

declare_id!("2FUi3XvEWg9N4nMzzqX13EQ7cz2nN7FNn9afRAPWhW1h");

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

    pub fn create_stream(
        ctx: Context<CreateStream>,
        stream_id: u64,
        recipient: Pubkey,
        total_amount: u64,
        start_time: i64,
        cliff_time: i64,
        end_time: i64,
        cancelable: bool,
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
}
