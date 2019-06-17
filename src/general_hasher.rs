use std::hash::Hasher;

pub trait GeneralHasher: Hasher {
    fn digest(&self) -> Vec<u8>;

    fn hash(&mut self, input: &[u8]) -> &mut Self {
        self.write(input);
        self
    }

    fn reset(&mut self) -> &mut Self;
}
