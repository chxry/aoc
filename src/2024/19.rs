use std::collections::HashMap;
use crate::*;

pub fn main(input: &str, part2: bool) -> u64 {
  let s = input.split("\n\n").to_arr::<2>();
  let towels = s[0].split(", ").to_vec();
  let mut subpatterns = HashMap::new();
  let mut sum = 0;
  for p in s[1].lines() {
    let n = check(p, &towels, &mut subpatterns);
    if part2 {
      sum += n;
    } else if n > 0 {
      sum += 1;
    }
  }
  sum
}

fn check(pat: &str, towels: &[&str], subpatterns: &mut HashMap<String, u64>) -> u64 {
  let mut sum = 0;
  for t in towels {
    if let Some(rest) = pat.strip_prefix(t) {
      if rest.is_empty() {
        sum += 1;
      } else if let Some(x) = subpatterns.get(rest) {
        sum += x;
      } else {
        let x = check(rest, towels, subpatterns);
        subpatterns.insert(rest.to_string(), x);
        sum += x;
      }
    }
  }
  sum
}
