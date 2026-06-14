use crate::{
    error::VestingError,
    state::stream::{Stream, VestingType},
};
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

#[allow(clippy::too_many_arguments)]
pub fn create_stream_handler(
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
    validate_create_stream_params(
        recipient,
        total_amount,
        start_time,
        cliff_time,
        end_time,
        vesting_type,
        milestone_time,
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
    stream.vesting_type = vesting_type;
    stream.milestone_reached = false;
    stream.milestone_time = milestone_time;
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

#[allow(clippy::too_many_arguments)]
pub fn validate_create_stream_params(
    recipient: Pubkey,
    total_amount: u64,
    start_time: i64,
    cliff_time: i64,
    end_time: i64,
    vesting_type: VestingType,
    milestone_time: i64,
    creator_token_balance: u64,
) -> Result<()> {
    require!(total_amount > 0, VestingError::InvalidAmount);
    require!(
        recipient != Pubkey::default(),
        VestingError::InvalidRecipient
    );
    match vesting_type {
        VestingType::Cliff => {
            require!(start_time < end_time, VestingError::InvalidSchedule);
            require!(cliff_time == end_time, VestingError::InvalidCliff);
        }
        VestingType::Linear => {
            require!(start_time < end_time, VestingError::InvalidSchedule);
            require!(cliff_time == start_time, VestingError::InvalidCliff);
        }
        VestingType::Milestone => {
            require!(milestone_time > 0, VestingError::InvalidMilestoneTime);
        }
    }
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
    fn validate_create_stream_accepts_cliff_params() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            109,
            110,
            110,
            VestingType::Cliff,
            0,
            1_000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validate_create_stream_accepts_linear_params() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            10,
            10,
            110,
            VestingType::Linear,
            0,
            1_000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validate_create_stream_accepts_milestone_params() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            0,
            0,
            0,
            VestingType::Milestone,
            110,
            1_000,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn validate_create_stream_rejects_zero_amount() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            0,
            10,
            10,
            110,
            VestingType::Linear,
            0,
            1_000,
        );

        assert_anchor_error(result, "InvalidAmount");
    }

    #[test]
    fn validate_create_stream_rejects_default_recipient() {
        let result = validate_create_stream_params(
            Pubkey::default(),
            1_000,
            10,
            10,
            110,
            VestingType::Linear,
            0,
            1_000,
        );

        assert_anchor_error(result, "InvalidRecipient");
    }

    #[test]
    fn validate_create_stream_rejects_invalid_schedule() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            110,
            110,
            10,
            VestingType::Linear,
            0,
            1_000,
        );

        assert_anchor_error(result, "InvalidSchedule");
    }

    #[test]
    fn validate_create_stream_rejects_wrong_cliff_shape() {
        let cliff_not_at_end = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            10,
            50,
            110,
            VestingType::Cliff,
            0,
            1_000,
        );
        let linear_with_late_cliff = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            10,
            50,
            110,
            VestingType::Linear,
            0,
            1_000,
        );

        assert_anchor_error(cliff_not_at_end, "InvalidCliff");
        assert_anchor_error(linear_with_late_cliff, "InvalidCliff");
    }

    #[test]
    fn validate_create_stream_rejects_insufficient_funds() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_001,
            10,
            10,
            110,
            VestingType::Linear,
            0,
            1_000,
        );

        assert_anchor_error(result, "InsufficientFunds");
    }

    #[test]
    fn validate_create_stream_rejects_missing_milestone_time() {
        let result = validate_create_stream_params(
            Pubkey::new_unique(),
            1_000,
            0,
            0,
            0,
            VestingType::Milestone,
            0,
            1_000,
        );

        assert_anchor_error(result, "InvalidMilestoneTime");
    }
}
