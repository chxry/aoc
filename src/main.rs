#![feature(array_chunks, let_chains)]
mod utils;
use macros::year;
pub use utils::*;

fn main() {
  year!(2024);
}
