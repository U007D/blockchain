use std::fmt::Debug;
use std::hash::{Hash, Hasher};

pub trait GeneralHasher: Hasher + PartialEq {
    type Digest: IntoIterator<Item = u8> + Clone + Debug + Default + Hash + PartialEq;
    fn digest(&self) -> Self::Digest;

    fn hash(&mut self, input: &[u8]) -> &mut Self {
        self.write(input);
        self
    }

    fn reset(&mut self) -> &mut Self;
}
