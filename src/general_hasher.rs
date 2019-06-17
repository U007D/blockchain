use std::hash::{Hash, Hasher};

pub trait GeneralHasher: Hasher {
    type Digest: IntoIterator<Item = u8> + Clone + Hash;
    fn digest(&self) -> Self::Digest;

    fn hash(&mut self, input: &[u8]) -> &mut Self {
        self.write(input);
        self
    }

    fn reset(&mut self) -> &mut Self;
}
