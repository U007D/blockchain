use std::hash::Hash;

pub trait Clock {
    type Output: Hash;

    fn now(&self) -> Self::Output;
}
