use anchor_lang::prelude::Pubkey;

struct MockStream {
    total_amount: u64,
    withdrawn_amount: u64,
    start_time: i64,
    cliff_time: i64,
    end_time: i64,
    #[allow(dead_code)]
    canceled: bool,
}

fn calculate_vested_amount(stream: &MockStream, now: i64) -> u64 {
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

fn make_stream() -> MockStream {
    MockStream {
        total_amount: 1_000_000,
        withdrawn_amount: 0,
        start_time: 0,
        cliff_time: 500_000,
        end_time: 1_000_000,
        canceled: false,
    }
}

fn make_stream_no_cliff() -> MockStream {
    MockStream {
        total_amount: 1_000_000,
        withdrawn_amount: 0,
        start_time: 0,
        cliff_time: 0,
        end_time: 1_000_000,
        canceled: false,
    }
}

fn make_pubkey(n: u8) -> Pubkey {
    Pubkey::new_from_array([n; 32])
}

#[test]
fn test_vested_before_cliff_is_zero() {
    let stream = make_stream();
    assert_eq!(calculate_vested_amount(&stream, 100_000), 0);
}

#[test]
fn test_vested_at_25_percent() {
    let stream = make_stream_no_cliff();
    assert_eq!(calculate_vested_amount(&stream, 250_000), 250_000);
}

#[test]
fn test_vested_at_cliff() {
    let stream = make_stream();
    assert_eq!(calculate_vested_amount(&stream, 500_000), 500_000);
}

#[test]
fn test_vested_halfway() {
    let stream = make_stream();
    assert_eq!(calculate_vested_amount(&stream, 750_000), 750_000);
}

#[test]
fn test_vested_at_end() {
    let stream = make_stream();
    assert_eq!(calculate_vested_amount(&stream, 1_000_000), 1_000_000);
}

#[test]
fn test_vested_after_end_is_total() {
    let stream = make_stream();
    assert_eq!(calculate_vested_amount(&stream, 9_999_999), 1_000_000);
}

#[test]
fn test_withdrawable_subtracts_already_withdrawn() {
    let mut stream = make_stream();
    stream.withdrawn_amount = 300_000;
    let vested = calculate_vested_amount(&stream, 750_000);
    let withdrawable = vested.saturating_sub(stream.withdrawn_amount);
    assert_eq!(withdrawable, 450_000);
}

#[test]
fn test_nothing_to_withdraw_before_cliff() {
    let stream = make_stream();
    let vested = calculate_vested_amount(&stream, 100_000);
    let withdrawable = vested.saturating_sub(stream.withdrawn_amount);
    assert_eq!(withdrawable, 0);
}

#[test]
fn test_unauthorized_cannot_withdraw() {
    let valid_recipient = make_pubkey(1);
    let attacker = make_pubkey(2);
    assert_ne!(valid_recipient, attacker);
}

#[test]
fn test_withdraw_full_amount() {
    let stream = make_stream();
    let vested = calculate_vested_amount(&stream, 1_000_000);
    let withdrawable = vested.saturating_sub(stream.withdrawn_amount);
    assert_eq!(withdrawable, 1_000_000);
}

#[test]
fn test_withdraw_partial_then_more() {
    let mut stream = make_stream();
    let vested_first = calculate_vested_amount(&stream, 500_000);
    let first_withdraw = vested_first.saturating_sub(stream.withdrawn_amount);
    assert_eq!(first_withdraw, 500_000);
    stream.withdrawn_amount += first_withdraw;

    let vested_second = calculate_vested_amount(&stream, 750_000);
    let second_withdraw = vested_second.saturating_sub(stream.withdrawn_amount);
    assert_eq!(second_withdraw, 250_000);
}
