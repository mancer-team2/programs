use crate::{error::VestingError, state::stream::Stream};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Token, TokenAccount};

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

pub fn close_stream_handler(ctx: Context<CloseStream>) -> Result<()> {
    // The escrow must be empty (enforced by the account constraint). On top of that,
    // only allow closing a stream that is fully settled: cancelled or fully withdrawn.
    validate_stream_settled(
        ctx.accounts.stream.canceled,
        ctx.accounts.stream.withdrawn_amount,
        ctx.accounts.stream.total_amount,
    )?;

    let stream_key = ctx.accounts.stream.key();
    let escrow_bump_seed = [ctx.accounts.stream.escrow_bump];
    let signer_seeds: &[&[u8]] = &[
        b"escrow_authority",
        stream_key.as_ref(),
        escrow_bump_seed.as_ref(),
    ];

    // Close the (empty) escrow token account, returning its rent to the creator.
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.key(),
        CloseAccount {
            account: ctx.accounts.escrow_token_account.to_account_info(),
            destination: ctx.accounts.creator.to_account_info(),
            authority: ctx.accounts.escrow_authority.to_account_info(),
        },
        &[signer_seeds],
    ))?;

    // The Stream account itself is closed by the `close = creator` constraint.
    Ok(())
}

/// A stream may be closed only once it is fully settled: either cancelled, or the
/// recipient has withdrawn the entire amount.
pub fn validate_stream_settled(
    canceled: bool,
    withdrawn_amount: u64,
    total_amount: u64,
) -> Result<()> {
    require!(
        canceled || withdrawn_amount == total_amount,
        VestingError::StreamNotSettled
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settled_when_fully_withdrawn() {
        assert!(validate_stream_settled(false, 1_000, 1_000).is_ok());
    }

    #[test]
    fn settled_when_canceled() {
        assert!(validate_stream_settled(true, 250, 1_000).is_ok());
    }

    #[test]
    fn rejects_active_stream() {
        let result = validate_stream_settled(false, 250, 1_000);

        match result.unwrap_err() {
            anchor_lang::error::Error::AnchorError(error) => {
                assert_eq!(error.error_name, "StreamNotSettled");
            }
            other => panic!("expected AnchorError, got {other:?}"),
        }
    }
}
