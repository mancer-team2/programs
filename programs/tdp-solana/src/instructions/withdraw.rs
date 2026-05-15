use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};
use crate::error::VestingError;
use crate::state::stream::Stream;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(
        mut,
        seeds = [
            b"stream",
            stream.creator.as_ref(),
            recipient.key().as_ref(),
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
        init_if_needed,
        payer = recipient,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

fn calculate_vested_amount(stream: &Stream, now: i64) -> u64 {
    if now < stream.cliff_time {
        return 0;
    }
    if now >= stream.end_time {
        return stream.total_amount;
    }

    let elapsed = (now - stream.start_time) as u128;
    let duration = (stream.end_time - stream.start_time) as u128;
    let total = stream.total_amount as u128;

    ((total * elapsed) / duration) as u64
}

pub fn withdraw_handler(ctx: Context<Withdraw>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;
    let stream_key = ctx.accounts.stream.key();
    let stream = &mut ctx.accounts.stream;

    require!(!stream.canceled, VestingError::StreamAlreadyCanceled);

    let vested = calculate_vested_amount(stream, now);
    let withdrawable = vested.saturating_sub(stream.withdrawn_amount);

    require!(withdrawable > 0, VestingError::NothingToWithdraw);

    stream.withdrawn_amount = stream
        .withdrawn_amount
        .checked_add(withdrawable)
        .ok_or(VestingError::MathOverflow)?;

    let escrow_bump = stream.escrow_bump;
    let seeds: &[&[u8]] = &[b"escrow_authority", stream_key.as_ref(), &[escrow_bump]];
    let signer_seeds = &[seeds];

    let cpi_accounts = Transfer {
        from: ctx.accounts.escrow_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: ctx.accounts.escrow_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        cpi_accounts,
        signer_seeds,
    );
    token::transfer(cpi_ctx, withdrawable)?;

    Ok(())
}
