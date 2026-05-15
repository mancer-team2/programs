use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

#[test]
fn program_id_is_declared() {
    let expected = Pubkey::from_str("BiwY71TrdBzgv2yfa6KfUxUMY8UCpeiUMGnwmCMTsfs9").unwrap();

    assert_eq!(tdp_solana::id(), expected);
}
