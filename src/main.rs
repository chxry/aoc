#![feature(array_chunks)]
use macros::year;

fn main() {
  year!(2024);
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

pub fn parse(s: &str) -> i32 {
  s.parse().expect("parse failed")
}
