use super::*;

#[test]
fn hash_hashes_empty_input() {
    // setup
    let input = b"";
    let expected_hash = b"*";

    // given a `GeneralHasher` and empty input
    let mut hasher = WorldsWorstHasher::new();

    // when input is hashed
    let result = hasher.hash(input).digest();

    // the result should be as expected
    assert_eq!(expected_hash.to_vec(), result);
}

#[test]
fn hash_hashes_a_single_input() {
    // setup
    let input = b"abc";
    let expected_hash = b"abc*";

    // given a `GeneralHasher` and an input
    let mut hasher = WorldsWorstHasher::new();

    // when input is hashed
    let result = hasher.hash(input).digest();

    // the result should be as expected
    assert_eq!(expected_hash.to_vec(), result);
}

#[test]
fn hash_hashes_multiple_inputs() {
    // setup
    let input = b"abc";
    let input_2 = b"def";
    let expected_hash = b"abcdef*";

    // given a `GeneralHasher` and multiple inputs
    let mut hasher = WorldsWorstHasher::new();

    // when input is hashed
    let result = hasher.hash(input).hash(input_2).digest();

    // the result should be as expected
    assert_eq!(expected_hash.to_vec(), result);
}

#[test]
fn reset_clears_hash_state() {
    // setup
    let input = b"abc";
    let expected_hash = b"*";

    // given a `GeneralHasher` in non-default state
    let mut hasher = WorldsWorstHasher::new();
    hasher.hash(input);

    // when hasher state is reset and then hash is retrieved
    let retval = hasher.reset().digest();

    // then the hash should be as expected
    assert_eq!(expected_hash.to_vec(), retval);
}
