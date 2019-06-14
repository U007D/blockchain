use super::*;

#[test]
fn hash_hashes_empty_input() {
    // given a hasher and empty input
    let mut hasher = WorldsWorstHasher::new();
    let input = "".as_bytes();
    let expected_hash = "*".as_bytes();

    // when input is hashed
    let result = hasher.hash(input).digest();

    // the result should be as expected
    assert_eq!(Vec::from(expected_hash), result);
}

#[test]
fn hash_hashes_a_single_input() {
    // given a hasher and an input
    let mut hasher = WorldsWorstHasher::new();
    let input = "abc".as_bytes();
    let expected_hash = "abc*".as_bytes();

    // when input is hashed
    let result = hasher.hash(input).digest();

    // the result should be as expected
    assert_eq!(Vec::from(expected_hash), result);
}

#[test]
fn hash_hashes_multiple_inputs() {
    // given a hasher and multiple inputs
    let mut hasher = WorldsWorstHasher::new();
    let input = "abc".as_bytes();
    let input_2 = "def".as_bytes();
    let expected_hash = "abcdef*".as_bytes();

    // when input is hashed
    let result = hasher.hash(input).hash(input_2).digest();

    // the result should be as expected
    assert_eq!(Vec::from(expected_hash), result);
}
