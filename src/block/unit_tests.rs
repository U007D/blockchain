use super::*;
use crate::test_clock::TestClock;
use crate::WorldsWorstHasher;
use time::Duration;

#[test]
fn new_block_initializes_fields() {
    // setup
    let mut hasher = WorldsWorstHasher::new();
    let nano_adjust = 1_u32;
    let clock = TestClock::from((
        hasher.clone(),
        Duration::nanoseconds(i64::from(nano_adjust)),
    ));
    let now = clock.now();
    let data = String::from("sample data");
    let genesis_block_hash = b"genesis_block_hash";

    // Determined using https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=73db745952194a6f28e3f7eed83d5b8b
    let mut expected_hash = vec![26, 64, 246, 0, 0, 0, 0, 0, 1, 0, 0, 0];
    expected_hash.extend_from_slice(data.as_bytes());
    // `str.hash()` terminates the string with 0xff https://doc.rust-lang.org/src/core/hash/mod.rs.html#599-604
    expected_hash.push(0xff);
    // `&[T].hash()` prefixes the elements with the length of the slice.
    // NB: This means 32- and 64-bit Rust platforms will hash identical slices to different values!!
    expected_hash.extend_from_slice(&genesis_block_hash.len().to_le_bytes());
    expected_hash.extend_from_slice(genesis_block_hash);
    expected_hash.push(b'*');

    let expected_retval = Block::<WorldsWorstHasher, TestClock<WorldsWorstHasher>, String> {
        timestamp: now,
        data: data.clone(),
        prev_block_hash: genesis_block_hash.to_vec(),
        digest: expected_hash,
    };

    // given a block constructor
    let sut = Block::new;

    // when constructed
    let retval = sut(
        &mut hasher,
        &clock,
        data.clone(),
        genesis_block_hash.to_vec(),
    );

    // then the result should be as expected
    assert_eq!(retval.digest, expected_retval.digest);
}

#[test]
fn digest_returns_current_digest() {
    // setup
    let mut hasher = WorldsWorstHasher::new();
    let clock = TestClock::from((hasher.clone(), Duration::nanoseconds(3)));
    let block = Block::new(
        &mut hasher,
        &clock,
        &"some data",
        "blah, blah, blah".as_bytes().to_vec(),
    );
    let expected_retval = block.digest.clone();

    // given
    let retval = block.digest();
    // then
    assert_eq!(retval, expected_retval);
}
