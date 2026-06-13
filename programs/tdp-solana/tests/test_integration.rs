use {
    anchor_lang::{
        solana_program::instruction::Instruction, AccountDeserialize, InstructionData,
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
            eprintln!("Skipping LiteSVM integration test: run `anchor build` to generate target/deploy/tdp_solana.so");
            return None;
        }
    };
    svm.add_program(program_id, &program_bytes).unwrap();
    svm.airdrop(&creator.pubkey(), 10_000_000_000).unwrap();
    svm.airdrop(&recipient.pubkey(), 10_000_000_000).unwrap();
    set_clock(&mut svm, START_TIME);
    set_mint_account(&mut svm, mint, creator.pubkey(), 0);
    set_token_account(
        &mut svm,
        creator_token_account,
        mint,
        creator.pubkey(),
        TOTAL_AMOUNT,
    );
    set_token_account(
        &mut svm,
        recipient_token_account,
        mint,
        recipient.pubkey(),
        0,
    );

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

fn token_amount(svm: &LiteSVM, address: &anchor_lang::prelude::Pubkey) -> u64 {
    let account = svm.get_account(address).unwrap();
    TokenAccount::unpack(&account.data).unwrap().amount
}

fn stream_state(svm: &LiteSVM, address: &anchor_lang::prelude::Pubkey) -> tdp_solana::Stream {
    let account = svm.get_account(address).unwrap();
    tdp_solana::Stream::try_deserialize(&mut account.data.as_slice()).unwrap()
}

fn create_stream(ctx: &mut TestContext) {
    let ix = Instruction::new_with_bytes(
        tdp_solana::id(),
        &tdp_solana::instruction::CreateStream {
            stream_id: 1,
            recipient: ctx.recipient.pubkey(),
            total_amount: TOTAL_AMOUNT,
            start_time: START_TIME,
            cliff_time: START_TIME,
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

fn withdraw(ctx: &mut TestContext) {
    let ix = Instruction::new_with_bytes(
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
    );
    send_ix(&mut ctx.svm, &ctx.recipient, ix, &[]);
}

// ─── Integration tests ────────────────────────────────────────────────────────

// Acceptance criterion: full flow from create_stream → wait → withdraw → verify balance.
#[test]
fn full_flow_create_wait_withdraw_verify_balance() {
    let Some(mut ctx) = setup() else {
        return;
    };

    // create_stream: tokens move from creator to escrow.
    create_stream(&mut ctx);
    assert_eq!(token_amount(&ctx.svm, &ctx.creator_token_account), 0);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        TOTAL_AMOUNT
    );

    // wait: advance clock to 100% vested.
    set_clock(&mut ctx.svm, END_TIME);

    // withdraw: all tokens move escrow → recipient in a single call.
    withdraw(&mut ctx);

    // verify balance.
    assert_eq!(
        token_amount(&ctx.svm, &ctx.recipient_token_account),
        TOTAL_AMOUNT
    );
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        0
    );
    assert_eq!(
        stream_state(&ctx.svm, &ctx.stream).withdrawn_amount,
        TOTAL_AMOUNT
    );
}

// Linear vesting: partial withdrawal at 75%, then claim the remaining 25%.
// Verifies that withdrawn_amount accounting is correct across multiple withdrawals.
// Schedule: start=100, end=500, duration=400 → 75% at t=400, 100% at t=500.
#[test]
fn full_flow_linear_vest_partial_then_complete() {
    let Some(mut ctx) = setup() else {
        return;
    };

    create_stream(&mut ctx);

    // 75% vested: elapsed=300, duration=400 → 750 tokens.
    set_clock(&mut ctx.svm, 400);
    withdraw(&mut ctx);
    assert_eq!(token_amount(&ctx.svm, &ctx.recipient_token_account), 750);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        250
    );

    // 100% vested: claim the remaining 25%.
    set_clock(&mut ctx.svm, END_TIME);
    withdraw(&mut ctx);
    assert_eq!(
        token_amount(&ctx.svm, &ctx.recipient_token_account),
        TOTAL_AMOUNT
    );
    assert_eq!(
        token_amount(&ctx.svm, &ctx.escrow_token_account.pubkey()),
        0
    );
    assert_eq!(
        stream_state(&ctx.svm, &ctx.stream).withdrawn_amount,
        TOTAL_AMOUNT
    );
}
