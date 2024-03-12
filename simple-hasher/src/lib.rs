#![no_std]

use sha2::{Digest, Sha256};

pub struct SimpleSha256Hasher {
    hasher: Sha256,
}

impl SimpleSha256Hasher {
    pub fn new() -> Self {
        SimpleSha256Hasher { hasher: Sha256::new() }
    }

    pub fn result(self) -> [u8; 32]  {
        self.hasher.finalize().into()
    }
}

impl core::hash::Hasher for SimpleSha256Hasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }
}