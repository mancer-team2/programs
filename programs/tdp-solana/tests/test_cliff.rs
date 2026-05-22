//! Cliff vesting behaviour at multiple time points.
//! Schedule: start=1000, cliff=2000, end=5000, total=1_000_000.
//! Before cliff -> 0. At cliff -> linear catch-up since start. After cliff -> linear. At/after end -> total.

use tdp_solana::calculate_vested_amount;

const TOTAL: u64 = 1_000_000;
const START: i64 = 1_000;
const CLIFF: i64 = 2_000;
const END: i64 = 5_000;

#[test]
fn nothing_vested_before_cliff() {
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, START).unwrap(),
        0
    );
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, CLIFF - 1).unwrap(),
        0
    );
}

#[test]
fn cliff_unlocks_catch_up_amount() {
    // At the cliff: elapsed since start = 1000 of 4000 => 25% unlocks at once.
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, CLIFF).unwrap(),
        250_000
    );
}

#[test]
fn vesting_is_linear_after_cliff() {
    // t=3000 -> 2000/4000 = 50%.
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, 3_000).unwrap(),
        500_000
    );
    // t=4000 -> 3000/4000 = 75%.
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, 4_000).unwrap(),
        750_000
    );
}

#[test]
fn fully_vested_at_and_after_end() {
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, END).unwrap(),
        TOTAL
    );
    assert_eq!(
        calculate_vested_amount(TOTAL, START, CLIFF, END, END + 10_000).unwrap(),
        TOTAL
    );
}

#[test]
fn no_cliff_behaves_as_pure_linear() {
    // cliff == start -> no cliff gate; 25% at quarter time (t=2000).
    assert_eq!(
        calculate_vested_amount(TOTAL, START, START, END, 2_000).unwrap(),
        250_000
    );
}
