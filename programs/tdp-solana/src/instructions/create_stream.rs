use crate::{error::VestingError, state::stream::Stream};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

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
    ctx: Context<CreateStream>,
    stream_id: u64,
    recipient: Pubkey,
    total_amount: u64,
    start_time: i64,
    cliff_time: i64,
    end_time: i64,
    cancelable: bool,
) -> Result<()> {
    validate_create_stream_params(
        recipient,
        total_amount,
        start_time,
        cliff_time,
        end_time,
        ctx.accounts.creator_token_account.amount,
    )?;

    let stream = &mut ctx.accounts.stream;
    stream.creator = ctx.accounts.creator.key();
    stream.recipient = recipient;
    stream.mint = ctx.accounts.mint.key();
    stream.escrow_token_account = ctx.accounts.escrow_token_account.key();
    stream.stream_id = stream_id;
    stream.total_amount = total_amount;
    stream.withdrawn_amount = 0;
    stream.start_time = start_time;
    stream.cliff_time = cliff_time;
    stream.end_time = end_time;
    stream.cancelable = cancelable;
    stream.canceled = false;
    stream.bump = ctx.bumps.stream;
    stream.escrow_bump = ctx.bumps.escrow_authority;
    stream.created_at = Clock::get()?.unix_timestamp;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            Transfer {
                from: ctx.accounts.creator_token_account.to_account_info(),
                to: ctx.accounts.escrow_token_account.to_account_info(),
                authority: ctx.accounts.creator.to_account_info(),
            },
        ),
        total_amount,
    )?;

    Ok(())
}

pub fn validate_create_stream_params(
    recipient: Pubkey,
    total_amount: u64,
    start_time: i64,
    cliff_time: i64,
    end_time: i64,
    creator_token_balance: u64,
) -> Result<()> {
    require!(total_amount > 0, VestingError::InvalidAmount);
    require!(
        recipient != Pubkey::default(),
        VestingError::InvalidRecipient
    );
    require!(start_time < end_time, VestingError::InvalidSchedule);
    require!(
        cliff_time >= start_time && cliff_time <= end_time,
        VestingError::InvalidCliff
    );
    require!(
        creator_token_balance >= total_amount,
        VestingError::InsufficientFunds
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_anchor_error(result: Result<()>, expected_name: &str) {
        let error = result.unwrap_err();

        match error {
            anchor_lang::error::Error::AnchorError(anchor_error) => {
                assert_eq!(anchor_error.error_name, expected_name);
            }
            _ => panic!("expected AnchorError, got {error:?}"),
        }
    }

    #[test]
    fn validate_create_stream_accepts_valid_params() {
        let result = validate_create_stream_params(Pubkey::new_unique(), 1_000, 10, 20, 110, 1_000);

        assert!(result.is_ok());
    }

    #[test]
    fn validate_create_stream_rejects_zero_amount() {
        let result = validate_create_stream_params(Pubkey::new_unique(), 0, 10, 20, 110, 1_000);

        assert_anchor_error(result, "InvalidAmount");
    }

    #[test]
    fn validate_create_stream_rejects_default_recipient() {
        let result = validate_create_stream_params(Pubkey::default(), 1_000, 10, 20, 110, 1_000);

        assert_anchor_error(result, "InvalidRecipient");
    }

    #[test]
    fn validate_create_stream_rejects_invalid_schedule() {
        let result =
            validate_create_stream_params(Pubkey::new_unique(), 1_000, 110, 110, 10, 1_000);

        assert_anchor_error(result, "InvalidSchedule");
    }

    #[test]
    fn validate_create_stream_rejects_cliff_outside_schedule() {
        let before_start =
            validate_create_stream_params(Pubkey::new_unique(), 1_000, 10, 9, 110, 1_000);
        let after_end =
            validate_create_stream_params(Pubkey::new_unique(), 1_000, 10, 111, 110, 1_000);

        assert_anchor_error(before_start, "InvalidCliff");
        assert_anchor_error(after_end, "InvalidCliff");
    }

    #[test]
    fn validate_create_stream_rejects_insufficient_funds() {
        let result = validate_create_stream_params(Pubkey::new_unique(), 1_001, 10, 20, 110, 1_000);

        assert_anchor_error(result, "InsufficientFunds");
    }
}
