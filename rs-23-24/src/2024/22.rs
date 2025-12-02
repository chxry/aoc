use std::collections::HashSet;
use crate::*;

pub fn main(input: &str, part2: bool) -> i64 {
  let mut sum = 0;
  let mut buyers = vec![];
  for l in input.lines() {
    let mut n = parse::<i64>(l);

    let mut nv = vec![];
    let mut dv = vec![];
    let mut last = n % 10;
    for _ in 0..2000 {
      nv.push(last);
      n ^= n * 64;
      n = n.rem_euclid(16777216);
      n ^= n / 32;
      n = n.rem_euclid(16777216);
      n ^= n * 2048;
      n = n.rem_euclid(16777216);

      let price = n % 10;
      dv.push(price - last);
      last = price;
    }
    nv.push(last);
    buyers.push((nv, dv));
    sum += n;
  }

  if part2 {
    let mut highest = 0;
    let mut seen = HashSet::new();

    for b in 0..buyers.len() {
      let (_, dv) = &buyers[b];
      for p in 0..dv.len() - 3 {
        let pat = &dv[p..p + 4];
        if !seen.insert(pat) {
          continue;
        }
        let mut bananas = 0;
        for (nv, dv) in &buyers {
          for p2 in 0..dv.len() - 3 {
            if dv[p2..p2 + 4] == *pat {
              bananas += nv[p2 + 4];
              break;
            }
          }
        }
        if bananas > highest {
          highest = bananas;
        }
      }
    }
    highest
  } else {
    sum
  }
}
