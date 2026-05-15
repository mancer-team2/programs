use crate::{error::VestingError, state::stream::Stream};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub recipient: Signer<'info>,

    #[account(
        mut,
        constraint = recipient.key() == stream.recipient @ VestingError::Unauthorized,
        seeds = [
            b"stream",
            stream.creator.as_ref(),
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

pub fn withdraw_handler(ctx: Context<Withdraw>) -> Result<()> {
    let stream_key = ctx.accounts.stream.key();
    let escrow_bump = ctx.accounts.stream.escrow_bump;

    validate_withdraw_recipient(ctx.accounts.recipient.key(), ctx.accounts.stream.recipient)?;
    require!(
        !ctx.accounts.stream.canceled,
        VestingError::StreamAlreadyCanceled
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
    let vested_amount = calculate_vested_amount(
        ctx.accounts.stream.total_amount,
        ctx.accounts.stream.start_time,
        ctx.accounts.stream.cliff_time,
        ctx.accounts.stream.end_time,
        now,
    )?;
    let withdrawable_amount =
        calculate_withdrawable_amount(vested_amount, ctx.accounts.stream.withdrawn_amount)?;

    require!(withdrawable_amount > 0, VestingError::NothingToWithdraw);

    ctx.accounts.stream.withdrawn_amount = ctx
        .accounts
        .stream
        .withdrawn_amount
        .checked_add(withdrawable_amount)
        .ok_or(VestingError::MathOverflow)?;

    let escrow_bump_seed = [escrow_bump];
    let signer_seeds: &[&[u8]] = &[
        b"escrow_authority",
        stream_key.as_ref(),
        escrow_bump_seed.as_ref(),
    ];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            Transfer {
                from: ctx.accounts.escrow_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.escrow_authority.to_account_info(),
            },
            &[signer_seeds],
        ),
        withdrawable_amount,
    )?;

    Ok(())
}

pub fn validate_withdraw_recipient(signer: Pubkey, recipient: Pubkey) -> Result<()> {
    require_keys_eq!(signer, recipient, VestingError::Unauthorized);

    Ok(())
}

pub fn calculate_vested_amount(
    total_amount: u64,
    start_time: i64,
    cliff_time: i64,
    end_time: i64,
    current_time: i64,
) -> Result<u64> {
    require!(start_time < end_time, VestingError::InvalidSchedule);
    require!(
        cliff_time >= start_time && cliff_time <= end_time,
        VestingError::InvalidCliff
    );

    if current_time < cliff_time {
        return Ok(0);
    }

    if current_time >= end_time {
        return Ok(total_amount);
    }

    let elapsed = current_time
        .checked_sub(start_time)
        .ok_or(VestingError::MathOverflow)? as u128;
    let duration = end_time
        .checked_sub(start_time)
        .ok_or(VestingError::MathOverflow)? as u128;
    let vested = (total_amount as u128)
        .checked_mul(elapsed)
        .ok_or(VestingError::MathOverflow)?
        .checked_div(duration)
        .ok_or(VestingError::MathOverflow)?;

    u64::try_from(vested).map_err(|_| VestingError::MathOverflow.into())
}

pub fn calculate_withdrawable_amount(vested_amount: u64, withdrawn_amount: u64) -> Result<u64> {
    vested_amount
        .checked_sub(withdrawn_amount)
        .ok_or_else(|| VestingError::MathOverflow.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_anchor_error<T: std::fmt::Debug>(result: Result<T>, expected_name: &str) {
        let error = result.unwrap_err();

        match error {
            anchor_lang::error::Error::AnchorError(anchor_error) => {
                assert_eq!(anchor_error.error_name, expected_name);
            }
            _ => panic!("expected AnchorError, got {error:?}"),
        }
    }

    #[test]
    fn calculate_vested_amount_returns_zero_before_cliff() {
        let vested = calculate_vested_amount(1_000, 100, 200, 500, 199).unwrap();

        assert_eq!(vested, 0);
    }

    #[test]
    fn calculate_vested_amount_returns_twenty_five_percent() {
        let vested = calculate_vested_amount(1_000, 100, 100, 500, 200).unwrap();

        assert_eq!(vested, 250);
    }

    #[test]
    fn calculate_vested_amount_returns_fifty_percent() {
        let vested = calculate_vested_amount(1_000, 100, 100, 500, 300).unwrap();

        assert_eq!(vested, 500);
    }

    #[test]
    fn calculate_vested_amount_caps_at_total_amount() {
        let vested = calculate_vested_amount(1_000, 100, 100, 500, 500).unwrap();

        assert_eq!(vested, 1_000);
    }

    #[test]
    fn calculate_withdrawable_amount_supports_partial_withdrawals() {
        let first_claim = calculate_withdrawable_amount(250, 0).unwrap();
        let second_claim = calculate_withdrawable_amount(500, first_claim).unwrap();

        assert_eq!(first_claim, 250);
        assert_eq!(second_claim, 250);
    }

    #[test]
    fn calculate_withdrawable_amount_supports_full_withdrawal() {
        let amount = calculate_withdrawable_amount(1_000, 250).unwrap();

        assert_eq!(amount, 750);
    }

    #[test]
    fn calculate_withdrawable_amount_rejects_overdrawn_state() {
        let result = calculate_withdrawable_amount(250, 500);

        assert_anchor_error(result, "MathOverflow");
    }

    #[test]
    fn validate_withdraw_recipient_rejects_unauthorized_signer() {
        let result = validate_withdraw_recipient(Pubkey::new_unique(), Pubkey::new_unique());

        assert_anchor_error(result, "Unauthorized");
    }
}
