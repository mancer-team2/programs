use {
    anchor_lang::{
        solana_program::instruction::Instruction, InstructionData,
        ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_account::Account,
    solana_clock::Clock,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_program_option::COption,
    solana_program_pack::Pack,
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
    spl_associated_token_account_interface::address::get_associated_token_address,
    spl_token_interface::{
        state::{Account as TokenAccount, AccountState, Mint},
        ID as TOKEN_PROGRAM_ID,
    },
    std::fs,
};

const TOTAL_AMOUNT: u64 = 1_000;
const START_TIME: i64 = 100;
const END_TIME: i64 = 500;

struct TestContext {
    svm: LiteSVM,
    creator: Keypair,
    recipient: Keypair,
    mint: anchor_lang::prelude::Pubkey,
    creator_token_account: anchor_lang::prelude::Pubkey,
    recipient_token_account: anchor_lang::prelude::Pubkey,
    escrow_token_account: Keypair,
    stream: anchor_lang::prelude::Pubkey,
    escrow_authority: anchor_lang::prelude::Pubkey,
}

fn program_bytes() -> Option<Vec<u8>> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../target/deploy/tdp_solana.so"
    );
    fs::read(path).ok()
}

fn setup() -> Option<TestContext> {
    let program_id = tdp_solana::id();
    let creator = Keypair::new();
    let recipient = Keypair::new();
    let mint = anchor_lang::prelude::Pubkey::new_unique();
    let stream_id = 1_u64;

    let creator_token_account = get_associated_token_address(&creator.pubkey(), &mint);
    let recipient_token_account = get_associated_token_address(&recipient.pubkey(), &mint);
    let (stream, _) = anchor_lang::prelude::Pubkey::find_program_address(
        &[
            b"stream",
            creator.pubkey().as_ref(),
            recipient.pubkey().as_ref(),
            &stream_id.to_le_bytes(),
        ],
        &program_id,
    );
    let (escrow_authority, _) = anchor_lang::prelude::Pubkey::find_program_address(
        &[b"escrow_authority", stream.as_ref()],
        &program_id,
    );

    let mut svm = LiteSVM::new();
    let program_bytes = match program_bytes() {
        Some(b) => b,
        None => {
            eprintln!("Skipping LiteSVM edge case test: run `anchor build` to generate target/deploy/tdp_solana.so");
            return None;
        }
    };
    svm.add_program(program_id, &program_bytes).unwrap();
    svm.airdrop(&creator.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&recipient.pubkey(), 10_000_000_000).unwrap();
    set_clock(&mut svm, START_TIME);
    set_mint_account(&mut svm, mint, creator.pubkey(), 0);
    set_token_account(&mut svm, creator_token_account, mint, creator.pubkey(), TOTAL_AMOUNT);
    set_token_account(&mut svm, recipient_token_account, mint, recipient.pubkey(), 0);

    Some(TestContext {
        svm,
        creator,
        recipient,
        mint,
        creator_token_account,
        recipient_token_account,
        escrow_token_account: Keypair::new(),
        stream,
        escrow_authority,
    })
}

fn set_clock(svm: &mut LiteSVM, unix_timestamp: i64) {
    svm.set_sysvar(&Clock {
        unix_timestamp,
        ..Clock::default()
    });
}

fn set_mint_account(
    svm: &mut LiteSVM,
    mint: anchor_lang::prelude::Pubkey,
    mint_authority: anchor_lang::prelude::Pubkey,
    supply: u64,
) {
    let mint_state = Mint {
        mint_authority: COption::Some(mint_authority),
        supply,
        decimals: 6,
        is_initialized: true,
        freeze_authority: COption::None,
    };
    let mut data = [0u8; Mint::LEN];
    Mint::pack(mint_state, &mut data).unwrap();
    svm.set_account(
        mint,
        Account {
            lamports: 1_000_000_000,
            data: data.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn set_token_account(
    svm: &mut LiteSVM,
    address: anchor_lang::prelude::Pubkey,
    mint: anchor_lang::prelude::Pubkey,
    owner: anchor_lang::prelude::Pubkey,
    amount: u64,
) {
    let token_account = TokenAccount {
        mint,
        owner,
        amount,
        delegate: COption::None,
        state: AccountState::Initialized,
        is_native: COption::None,
        delegated_amount: 0,
        close_authority: COption::None,
    };
    let mut data = [0u8; TokenAccount::LEN];
    TokenAccount::pack(token_account, &mut data).unwrap();
    svm.set_account(
        address,
        Account {
            lamports: 1_000_000_000,
            data: data.to_vec(),
            owner: TOKEN_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        },
    )
    .unwrap();
}

fn send_ix(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction, signers: &[&Keypair]) {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let mut tx_signers = vec![payer];
    tx_signers.extend_from_slice(signers);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &tx_signers).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_ok(), "{res:?}");
}

fn send_ix_expect_err(svm: &mut LiteSVM, payer: &Keypair, ix: Instruction, signers: &[&Keypair]) {
    svm.expire_blockhash();
    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[ix], Some(&payer.pubkey()), &blockhash);
    let mut tx_signers = vec![payer];
    tx_signers.extend_from_slice(signers);
    let tx = VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &tx_signers).unwrap();
    let res = svm.send_transaction(tx);
    assert!(res.is_err(), "expected failure, got {res:?}");
}

fn token_amount(svm: &LiteSVM, address: &anchor_lang::prelude::Pubkey) -> u64 {
    let account = svm.get_account(address).unwrap();
    TokenAccount::unpack(&account.data).unwrap().amount
}

fn withdraw_ix(ctx: &TestContext) -> Instruction {
    Instruction::new_with_bytes(
        tdp_solana::id(),
        &tdp_solana::instruction::Withdraw {}.data(),
        tdp_solana::accounts::Withdraw {
            recipient: ctx.recipient.pubkey(),
            stream: ctx.stream,
            mint: ctx.mint,
            escrow_authority: ctx.escrow_authority,
            escrow_token_account: ctx.escrow_token_account.pubkey(),
            recipient_token_account: ctx.recipient_token_account,
            token_program: TOKEN_PROGRAM_ID,
            associated_token_program: spl_associated_token_account_interface::program::ID,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    )
}

fn create_stream_with_cliff(ctx: &mut TestContext, cliff_time: i64) {
    let ix = Instruction::new_with_bytes(
        tdp_solana::id(),
        &tdp_solana::instruction::CreateStream {
            stream_id: 1,
            recipient: ctx.recipient.pubkey(),
            total_amount: TOTAL_AMOUNT,
            start_time: START_TIME,
            cliff_time,
            end_time: END_TIME,
            cancelable: false,
            milestone_based: false,
        }
        .data(),
        tdp_solana::accounts::CreateStream {
            creator: ctx.creator.pubkey(),
            stream: ctx.stream,
            mint: ctx.mint,
            creator_token_account: ctx.creator_token_account,
            escrow_authority: ctx.escrow_authority,
            escrow_token_account: ctx.escrow_token_account.pubkey(),
            token_program: TOKEN_PROGRAM_ID,
            associated_token_program: spl_associated_token_account_interface::program::ID,
            system_program: anchor_lang::system_program::ID,
        }
        .to_account_metas(None),
    );
    send_ix(&mut ctx.svm, &ctx.creator, ix, &[&ctx.escrow_token_account]);
}

// cliff == start → no cliff gate, purely linear from the start.
fn create_stream_no_cliff(ctx: &mut TestContext) {
    create_stream_with_cliff(ctx, START_TIME);
}

// ─── Edge cases ───────────────────────────────────────────────────────────────

// Edge case: withdraw at exactly cliff date.
// Schedule: start=100, cliff=300, end=500, total=1000.
// At t=299: nothing vested (before cliff gate).
// At t=300: cliff catch-up unlocks 50% — elapsed=200, duration=400 → 500 tokens.
#[test]
fn withdraw_at_exactly_cliff_date_unlocks_catch_up_amount() {
    let Some(mut ctx) = setup() else {
        return;
    };

    create_stream_with_cliff(&mut ctx, 300);

    // One tick before cliff: nothing vested.
    set_clock(&mut ctx.svm, 299);
    let ix = withdraw_ix(&ctx);
    send_ix_expect_err(&mut ctx.svm, &ctx.recipient, ix, &[]);
    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 0);

    // At exactly cliff: catch-up unlocks 50%.
    set_clock(&mut ctx.svm, 300);
    let ix = withdraw_ix(&ctx);
    send_ix(&mut ctx.svm, &ctx.recipient, ix, &[]);
    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 500);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        500
    );
}

// Edge case: double withdraw — second call at the same timestamp must fail.
// Verifies the program cannot be drained by replaying the withdraw instruction.
// At t=300 (50%): first withdraw gets 500 tokens; second withdraw at same clock → NothingToWithdraw.
#[test]
fn double_withdraw_same_time_fails_with_nothing_to_withdraw() {
    let Some(mut ctx) = setup() else {
        return;
    };

    create_stream_no_cliff(&mut ctx);

    // First withdraw at 50%: elapsed=200, duration=400 → 500 tokens.
    set_clock(&mut ctx.svm, 300);
    let ix = withdraw_ix(&ctx);
    send_ix(&mut ctx.svm, &ctx.recipient, ix, &[]);
    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 500);

    // Immediate second withdraw at the same timestamp: nothing new vested.
    let ix = withdraw_ix(&ctx);
    send_ix_expect_err(&mut ctx.svm, &ctx.recipient, ix, &[]);

    // Balance must be unchanged — no double-spend.
    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 500);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        500
    );
}

// Edge case: withdraw with nothing available — clock is before cliff → must fail.
// Escrow remains fully funded; recipient balance stays at zero.
#[test]
fn withdraw_with_nothing_available_before_cliff_fails() {
    let Some(mut ctx) = setup() else {
        return;
    };

    // Cliff at 300; test at t=200 — before cliff.
    create_stream_with_cliff(&mut ctx, 300);

    set_clock(&mut ctx.svm, 200);
    let ix = withdraw_ix(&ctx);
    send_ix_expect_err(&mut ctx.svm, &ctx.recipient, ix, &[]);

    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 0);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        TOTAL_AMOUNT
    );
}
