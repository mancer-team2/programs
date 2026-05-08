use anchor_lang::prelude::Pubkey;
use std::str::FromStr;

#[test]
fn program_id_is_declared() {
    let expected = Pubkey::from_str("2FUi3XvEWg9N4nMzzqX13EQ7cz2nN7FNn9afRAPWhW1h").unwrap();
    assert_eq!(tdp_solana::id(), expected);
}
