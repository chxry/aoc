#![feature(array_chunks, let_chains, iter_next_chunk)]
mod utils;
use macros::year;
pub use utils::*;

fn main() {
  year!(2024);
}
