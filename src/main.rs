#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]

// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing license files and more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod args;
mod block;
mod blockchain;
mod clock;
mod consts;
mod error;
mod general_hasher;
#[cfg(test)]
mod test_clock;
mod worlds_worst_hasher;

use std::result::Result as StdResult;
use structopt::StructOpt;
pub use {args::Args, block::Block, blockchain::Blockchain, consts::*, error::Error};
use {clock::Clock, general_hasher::GeneralHasher, worlds_worst_hasher::WorldsWorstHasher};
pub type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let _args = Args::from_args();
    Ok(())
}
