use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::stream::Stream;

#[derive(Accounts)]
#[instruction(stream_id: u64, recipient: Pubkey)]
pub struct CreateStream<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = 8 + Stream::INIT_SPACE,
        seeds = [
            b"stream",
            creator.key().as_ref(),
            recipient.as_ref(),
            &stream_id.to_le_bytes(),
        ],
        bump,
    )]
    pub stream: Account<'info, Stream>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = creator,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,

    /// CHECK: PDA signer only, no private key
    #[account(
        seeds = [b"escrow_authority", stream.key().as_ref()],
        bump,
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    #[account(
        init,
        payer = creator,
        token::mint = mint,
        token::authority = escrow_authority,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn create_stream_handler(
    _ctx: Context<CreateStream>,
    _stream_id: u64,
    _recipient: Pubkey,
    _total_amount: u64,
    _start_time: i64,
    _cliff_time: i64,
    _end_time: i64,
    _cancelable: bool,
) -> Result<()> {
    // TODO Week 4: validasi + init Stream fields + CPI transfer ke escrow
    Ok(())
}
