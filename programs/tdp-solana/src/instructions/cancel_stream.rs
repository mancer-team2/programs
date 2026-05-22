use crate::instructions::withdraw::calculate_vested_amount;
use crate::{error::VestingError, state::stream::Stream};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

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

pub fn cancel_stream_handler(ctx: Context<CancelStream>) -> Result<()> {
    let stream_key = ctx.accounts.stream.key();
    let escrow_bump = ctx.accounts.stream.escrow_bump;

    // Only the creator can cancel (the seeds already enforce this; this is defense-in-depth).
    require_keys_eq!(
        ctx.accounts.creator.key(),
        ctx.accounts.stream.creator,
        VestingError::Unauthorized
    );
    require!(
        ctx.accounts.stream.cancelable,
        VestingError::StreamNotCancelable
    );
    require!(
        !ctx.accounts.stream.canceled,
        VestingError::AlreadyCancelled
    );
    require_keys_eq!(
        ctx.accounts.mint.key(),
        ctx.accounts.stream.mint,
        VestingError::InvalidTokenAccount
    );
    require_keys_eq!(
        ctx.accounts.escrow_token_account.key(),
        ctx.accounts.stream.escrow_token_account,
        VestingError::InvalidTokenAccount
    );

    let now = Clock::get()?.unix_timestamp;
    let total_amount = ctx.accounts.stream.total_amount;

    // Reject cancelling a stream that is already fully unlocked, per mode.
    let vested_amount = if ctx.accounts.stream.milestone_based {
        require!(
            !ctx.accounts.stream.milestone_reached,
            VestingError::FullyVested
        );
        0
    } else {
        require!(
            now < ctx.accounts.stream.end_time,
            VestingError::StreamExpired
        );
        calculate_vested_amount(
            total_amount,
            ctx.accounts.stream.start_time,
            ctx.accounts.stream.cliff_time,
            ctx.accounts.stream.end_time,
            now,
        )?
    };

    let withdrawn_amount = ctx.accounts.stream.withdrawn_amount;
    let (to_recipient, to_creator) =
        split_cancel_amounts(total_amount, vested_amount, withdrawn_amount)?;

    // Effects before interactions.
    ctx.accounts.stream.canceled = true;

    let escrow_bump_seed = [escrow_bump];
    let signer_seeds: &[&[u8]] = &[
        b"escrow_authority",
        stream_key.as_ref(),
        escrow_bump_seed.as_ref(),
    ];

    // Vested-but-unclaimed tokens go to the recipient.
    if to_recipient > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.key(),
                Transfer {
                    from: ctx.accounts.escrow_token_account.to_account_info(),
                    to: ctx.accounts.recipient_token_account.to_account_info(),
                    authority: ctx.accounts.escrow_authority.to_account_info(),
                },
                &[signer_seeds],
            ),
            to_recipient,
        )?;
    }

    // Still-locked tokens return to the creator.
    if to_creator > 0 {
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.key(),
                Transfer {
                    from: ctx.accounts.escrow_token_account.to_account_info(),
                    to: ctx.accounts.creator_token_account.to_account_info(),
                    authority: ctx.accounts.escrow_authority.to_account_info(),
                },
                &[signer_seeds],
            ),
            to_creator,
        )?;
    }

    Ok(())
}

/// Split the escrow balance when a stream is cancelled:
/// - to_recipient = vested - withdrawn (unlocked but not yet claimed)
/// - to_creator   = total - vested     (still locked)
pub fn split_cancel_amounts(
    total_amount: u64,
    vested_amount: u64,
    withdrawn_amount: u64,
) -> Result<(u64, u64)> {
    let to_recipient = vested_amount
        .checked_sub(withdrawn_amount)
        .ok_or(VestingError::MathOverflow)?;
    let to_creator = total_amount
        .checked_sub(vested_amount)
        .ok_or(VestingError::MathOverflow)?;

    Ok((to_recipient, to_creator))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_before_cliff_returns_all_to_creator() {
        let (to_recipient, to_creator) = split_cancel_amounts(1_000, 0, 0).unwrap();

        assert_eq!(to_recipient, 0);
        assert_eq!(to_creator, 1_000);
    }

    #[test]
    fn split_mid_stream_splits_vested_and_locked() {
        let (to_recipient, to_creator) = split_cancel_amounts(1_000, 600, 0).unwrap();

        assert_eq!(to_recipient, 600);
        assert_eq!(to_creator, 400);
    }

    #[test]
    fn split_accounts_for_already_withdrawn() {
        let (to_recipient, to_creator) = split_cancel_amounts(1_000, 600, 250).unwrap();

        assert_eq!(to_recipient, 350);
        assert_eq!(to_creator, 400);
    }

    #[test]
    fn split_rejects_overdrawn_state() {
        let result = split_cancel_amounts(1_000, 200, 500);

        assert!(result.is_err());
    }
}
