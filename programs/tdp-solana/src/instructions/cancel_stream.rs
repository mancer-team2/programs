use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use crate::state::stream::Stream;

#[derive(Accounts)]
pub struct CancelStream<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    /// CHECK: address-checked to equal stream.recipient; used only as ATA authority handle.
    #[account(address = stream.recipient)]
    pub recipient_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"stream",
            creator.key().as_ref(),
            stream.recipient.as_ref(),
            &stream.stream_id.to_le_bytes(),
        ],
        bump = stream.bump,
    )]
    pub stream: Account<'info, Stream>,

    pub mint: Account<'info, Mint>,

    /// CHECK: PDA signer only, no private key
    #[account(
        seeds = [b"escrow_authority", stream.key().as_ref()],
        bump = stream.escrow_bump,
    )]
    pub escrow_authority: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = escrow_authority,
    )]
    pub escrow_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = creator,
    )]
    pub creator_token_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = creator,
        associated_token::mint = mint,
        associated_token::authority = recipient_authority,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn cancel_stream_handler(_ctx: Context<CancelStream>) -> Result<()> {
    // TODO Week 5: cek cancelable, split vested vs unvested, CPI transfer keduanya
    Ok(())
}
