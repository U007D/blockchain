use crate::Clock;
use crate::GeneralHasher;
use std::hash::Hash;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<C, H, T>
where
    C: Clock,
    H: GeneralHasher,
{
    timestamp: C::Output,
    data: T,
    prev_block_hash: H::Digest,
    hash: H::Digest,
}

impl<C, H, T> Block<C, H, T>
where
    C: Clock,
    H: GeneralHasher,
    T: Hash,
{
    fn new(clock: &C, hasher: &mut H, data: T, prev_block_hash: &H::Digest) -> Self {
        let now = clock.now();
        hasher.reset();
        now.hash(hasher);
        data.hash(hasher);
        prev_block_hash.hash(hasher);
        Self {
            timestamp: clock.now(),
            data,
            prev_block_hash: prev_block_hash.clone(),
            hash: hasher.digest(),
        }
    }
}
