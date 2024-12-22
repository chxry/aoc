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

    for a in -9..=9 {
      for b in -9..=9 {
        for c in -9..=9 {
          for d in -9..=9 {
            let p = [a, b, c, d];
            let mut bananas = 0;
            for (nv, dv) in &buyers {
              for i in 0..(dv.len() - 3) {
                if dv[i..i + 4] == p {
                  bananas += nv[i + 4];
                  break;
                }
              }
            }
            if bananas > highest {
              highest = bananas;
            }
          }
        }
      }
    }

    highest
  } else {
    sum
  }
}
