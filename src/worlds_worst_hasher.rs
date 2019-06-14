#[cfg(test)]
mod unit_tests;

pub struct WorldsWorstHasher {
    digest: Vec<u8>,
}

impl WorldsWorstHasher {
    pub fn new() -> Self {
        Self { digest: Vec::new() }
    }

    pub fn hash(&mut self, input: &[u8]) -> &mut Self {
        self.digest.pop();
        self.digest.extend_from_slice(input);
        self.digest.push(b'*');
        self
    }

    pub fn digest(&self) -> Vec<u8> {
        self.digest.clone()
    }
}
