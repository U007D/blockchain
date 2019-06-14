use super::*;

#[test]
fn new_block_initializes_fields() {
    // given a block constructor
    let sut = Block::new;

    let clock =
    let data = "sample data";
    let genesis_block_hash = b"genesis_block_hash";
    let expected_result = Block {
        timestamp: 1,
        data: String::from(data),
        prev_block_hash: Vec::from(genesis_block_hash),
        hash: Vec::from(b"1sample datagenesis_block_hash*"),
    };

    // when constructed
    let result = sut(clock, hasher, data, genesis_block_hash);

    // then the result should be as expected
}