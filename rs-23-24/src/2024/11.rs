use std::collections::HashMap;
use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let mut stones = HashMap::from_iter(input.split(" ").map(|s| (parse::<u64>(s), 1)));
  for _ in 0..if part2 { 75 } else { 25 } {
    let mut new = HashMap::new();
    for (x, n) in stones.iter() {
      for x in blink(*x) {
        *new.entry(x).or_default() += n;
      }
    }
    stones = new;
  }
  stones.values().sum()
}

fn blink(x: u64) -> Vec<u64> {
  if x == 0 {
    return vec![1];
  }
  let str = x.to_string();
  if str.len() % 2 == 0 {
    return vec![parse(&str[..str.len() / 2]), parse(&str[str.len() / 2..])];
  }
  vec![x * 2024]
}
