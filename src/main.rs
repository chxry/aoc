#![feature(array_chunks)]
mod utils;
use macros::year;
pub use utils::*;

fn main() {
  year!(2024);
}
