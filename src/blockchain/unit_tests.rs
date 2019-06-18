use super::*;
use crate::{test_clock::TestClock, Block, WorldsWorstHasher};
use time::Duration;

#[test]
fn new_creates_blockchain_with_genesis_block() {
    // setup
    let mut hasher = WorldsWorstHasher::new();
    let clock = TestClock::from((hasher.clone(), Duration::nanoseconds(42)));
    let expected_retval = Blockchain {
        blocks: vec![Block::new(&mut hasher, &clock, (), Vec::new())],
        hasher: hasher.clone(),
        clock: clock.clone(),
    };

    // given
    let sut = Blockchain::<WorldsWorstHasher, TestClock<WorldsWorstHasher>, ()>::new;

    // when
    let retval = sut(hasher, clock);

    // then
    assert_eq!(retval, expected_retval);
}
