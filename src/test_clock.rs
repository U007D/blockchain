use crate::{msg, Clock, Error, GeneralHasher, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use std::hash::Hash;
use time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TestClock<H: GeneralHasher> {
    date_time: DateTime<Utc>,
    hasher: H,
}

impl<H: GeneralHasher> TestClock<H> {
    pub fn new(hasher: H) -> Self {
        Self {
            date_time: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            hasher,
        }
    }

    pub fn adjust_by(&mut self, duration: Duration) -> Result<&mut Self> {
        self.date_time
            .checked_add_signed(duration)
            .ok_or_else(|| Error::ClockOverflow(self.date_time, duration))
            .and_then(|dt| {
                self.date_time = dt;
                Ok(self)
            })
    }

    pub fn digest(&mut self) -> H::Digest {
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
