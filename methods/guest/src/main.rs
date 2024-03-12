#![no_main]
// If you want to try std support, also update the guest Cargo.toml file
#![no_std]  // std support is experimental


use revm_primitives::AccountInfo;

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


use core::hash::Hash;
use simple_hasher::SimpleSha256Hasher;

fn main() {
    let nonce: u64 = env::read();
    let account_info: AccountInfo = env::read();
    let input_result: [u8; 32] = env::read();

    let mut hasher = SimpleSha256Hasher::new();
    account_info.hash(&mut hasher);

    let result = hasher.result();

    assert_eq!(result, input_result, "hashes do not match");
    assert_eq!(nonce, account_info.nonce, "nonce do not match");


    // write public output to the journal
    env::commit(&nonce);
}
