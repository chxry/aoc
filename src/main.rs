#![feature(array_chunks)]
use macros::day;

fn main() {
  // day!(1);
  // day!(2);
  // day!(4);
  // day!(5);
  // day!(6);
  // day!(7);
  day!(8);
}

pub fn lcm(n: &[usize]) -> usize {
  if n.len() == 1 {
    return n[0];
  }
  let a = n[0];
  let b = lcm(&n[1..]);
  a * b / hcf(a, b)
}

pub fn hcf(a: usize, b: usize) -> usize {
  if b == 0 {
    return a;
  }
  hcf(b, a % b)
}
