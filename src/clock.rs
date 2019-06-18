use std::fmt::Debug;
use std::hash::Hash;

pub trait Clock: PartialEq {
    type Output: Hash + Clone + Debug + PartialEq;

    fn now(&self) -> Self::Output;
}
