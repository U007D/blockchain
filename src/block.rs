use crate::Clock;
use crate::GeneralHasher;
use std::hash::Hash;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Block<H, C, T>
where
    H: GeneralHasher,
    C: Clock,
{
    timestamp: C::Output,
    data: T,
    prev_block_hash: H::Digest,
    digest: H::Digest,
}

impl<H, C, T> Block<H, C, T>
where
    H: GeneralHasher,
    C: Clock,
    T: Hash,
{
    pub fn new(hasher: &mut H, clock: &C, data: T, prev_block_hash: H::Digest) -> Self {
        let timestamp = clock.now();
        Self {
            digest: {
                hasher.reset();
                timestamp.hash(hasher);
                data.hash(hasher);
                prev_block_hash.hash(hasher);
                hasher.digest()
            },
            timestamp,
            data,
            prev_block_hash: prev_block_hash.clone(),
        }
    }

    pub fn digest(&self) -> H::Digest {
        self.digest.clone()
    }
}
