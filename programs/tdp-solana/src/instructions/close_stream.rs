use crate::state::stream::Stream;
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
pub struct CloseStream<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"stream",
            creator.key().as_ref(),
            stream.recipient.as_ref(),
            &stream.stream_id.to_le_bytes(),
        ],
        bump = stream.bump,
        close = creator,
    )]
    pub stream: Account<'info, Stream>,

    #[account(
        mut,
        token::authority = escrow_authority,
        constraint = escrow_token_account.amount == 0,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA signer only, no private key
    #[account(
        seeds = [b"escrow_authority", stream.key().as_ref()],
        bump = stream.escrow_bump,
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn close_stream_handler(_ctx: Context<CloseStream>) -> Result<()> {
    // TODO Week 5: validasi escrow kosong, tutup accounts, kembalikan rent ke creator
    Ok(())
}
