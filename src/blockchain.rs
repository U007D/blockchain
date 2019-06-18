use crate::{Block, Clock, GeneralHasher};
use std::hash::Hash;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Blockchain<H, C, T>
where
    H: GeneralHasher,
    C: Clock,
{
    hasher: H,
    clock: C,
    blocks: Vec<Block<H, C, T>>,
}

impl<H, C, T> Blockchain<H, C, T>
where
    H: GeneralHasher,
    C: Clock,
    T: Hash + Default,
{
    pub fn new(mut hasher: H, clock: C) -> Self {
        Self {
            blocks: vec![Self::make_genesis_block(&mut hasher, &clock)],
            clock,
            hasher,
        }
    }

    pub fn make_block(&mut self, data: T) -> &mut Self {
        self.blocks.push(Block::new(
            &mut self.hasher,
            &self.clock,
            data,
            self.blocks[self.blocks.len() - 1].digest(),
        ));
        self
    }

    fn make_genesis_block(hasher: &mut H, clock: &C) -> Block<H, C, T> {
        Block::new(hasher, clock, T::default(), H::Digest::default())
    }
}
