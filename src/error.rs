use crate::consts::msg;
use chrono::{DateTime, Utc};
use derive_more::*;
use time::Duration;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}: {:?}", "msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8", "_0")]
    ArgNotConvertibleToUtf8(std::ffi::OsString),
    #[display(
        fmt = "{}: {:?} + {:?}",
        "msg::ERR_INTERNAL_CLOCK_OVERFLOW",
        "_0",
        "_1"
    )]
    ClockOverflow(DateTime<Utc>, Duration),
}
