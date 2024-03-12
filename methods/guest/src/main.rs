#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use revm_primitives::AccountInfo;
use revm_primitives::U256;


use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


use core::hash::Hash;
use simple_hasher::SimpleSha256Hasher;

fn main() {
    let nonce: u32 = env::read();
    let account_info: AccountInfo = env::read();
    let input_result: [u8; 32] = env::read();
    let empty_s_hash_input: [u8; 32] = env::read();

    let mut hasher = SimpleSha256Hasher::new();
    account_info.hash(&mut hasher);
    let result = hasher.result();


    let mut manual = AccountInfo::from_balance(U256::from(100_000));
    manual.nonce = nonce as u64;
    let mut hasher2 = SimpleSha256Hasher::new();
    manual.hash(&mut hasher2);
    let result2 = hasher2.result();

    assert_eq!(manual, account_info, "account info does not match");

    assert_eq!(result, result2);

    // assert_eq!(result, input_result, "hashes do not match");
    assert_eq!(nonce as u64, account_info.nonce, "nonce do not match");

    let s = b"";
    let mut empty_string_hasher = SimpleSha256Hasher::new();
    s.hash(&mut empty_string_hasher);
    let empty_s_hash = empty_string_hasher.result();

    assert_eq!(empty_s_hash, empty_s_hash_input, "empty hashes do not match");

    // write public output to the journal
    env::commit(&nonce);
}
