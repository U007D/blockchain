use super::*;
use crate::general_hasher::GeneralHasher;
use crate::{consts::*, Clock, WorldsWorstHasher};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::mem;
use time::Duration;

#[derive(Clone, Debug, PartialEq)]
struct TestClock<H: GeneralHasher> {
    date_time: DateTime<Utc>,
    hasher: H,
}

impl<H: GeneralHasher> TestClock<H> {
    fn new(hasher: H) -> Self {
        Self {
            date_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            hasher,
        }
    }

    #[must_use]
    fn adjust_by(&mut self, duration: Duration) -> Option<&mut Self> {
        self.date_time.checked_add_signed(duration).and_then(|dt| {
            self.date_time = dt;
            Some(self)
        })
    }

    fn digest(&mut self) -> Vec<u8> {
        self.hasher.reset();
        self.date_time.hash(&mut self.hasher);
        self.hasher.digest()
    }
}

impl<H: GeneralHasher> From<(H, Duration)> for TestClock<H> {
    fn from((hasher, duration): (H, Duration)) -> Self {
        let mut clock = Self::new(hasher);
        clock
            .adjust_by(duration)
            .expect(msg::ERR_INTERNAL_CLOCK_OVERFLOW);
        clock
    }
}

impl<H: GeneralHasher> Clock for TestClock<H> {
    type Output = DateTime<Utc>;

    fn now(&self) -> Self::Output {
        self.date_time
    }
}

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

    let expected_retval = Block::<TestClock<WorldsWorstHasher>, String> {
        timestamp: now,
        data: data.clone(),
        prev_block_hash: genesis_block_hash.to_vec(),
        hash: expected_hash,
    };

    // given a block constructor
    let sut = Block::new;

    // when constructed
    let retval = sut(&clock, &mut hasher, data, genesis_block_hash);

    // then the result should be as expected
    assert_eq!(retval.hash, expected_retval.hash);
}
