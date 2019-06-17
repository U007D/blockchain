use crate::Clock;
use crate::GeneralHasher;
use std::hash::Hash;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<C: Clock, T> {
    timestamp: C::Output,
    data: T,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl<C: Clock, T: Hash> Block<C, T> {
    fn new<H: GeneralHasher>(clock: &C, hasher: &mut H, data: T, prev_block_hash: &[u8]) -> Self {
        let now = clock.now();
        hasher.reset();
        now.hash(hasher);
        data.hash(hasher);
        prev_block_hash.hash(hasher);
        Self {
            timestamp: clock.now(),
            data,
            prev_block_hash: prev_block_hash.to_vec(),
            hash: hasher.digest(),
        }
    }
}
