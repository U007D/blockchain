use crate::consts::*;
use crate::general_hasher::GeneralHasher;
use std::hash::Hasher;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct WorldsWorstHasher {
    digest: Vec<u8>,
}

impl WorldsWorstHasher {
    pub fn new() -> Self {
        let mut hasher = Self { digest: Vec::new() };
        hasher.append_terminator();
        hasher
    }

    fn append_terminator(&mut self) -> &mut Self {
        self.digest.push(b'*');
        self
    }
}

impl Hasher for WorldsWorstHasher {
    fn finish(&self) -> u64 {
        panic!(msg::ERR_INTERNAL_CALL_TO_DEPRECATED_FN_FINISH)
    }

    fn write(&mut self, bytes: &[u8]) {
        self.digest.pop();
        self.digest.extend_from_slice(bytes);
        self.append_terminator();
    }
}

impl GeneralHasher for WorldsWorstHasher {
    fn digest(&self) -> Vec<u8> {
        self.digest.clone()
    }

    fn reset(&mut self) -> &mut Self {
        self.digest.clear();
        self.append_terminator();
        self
    }
}
