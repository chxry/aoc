use regex::Regex;
use crate::*;

pub fn main(input: &str, part2: bool) -> i64 {
  let re = Regex::new(
    r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
  )
  .unwrap();
  let mut sum = 0;
  for c in re.captures_iter(input) {
    let [xa, ya, xb, yb, mut xp, mut yp] = c.extract().1.map(parse::<i64>);
    if part2 {
      xp += 10000000000000;
      yp += 10000000000000;
    }
    // write helper for matrix slop
    let det = xa * yb - ya * xb;
    let a = yb * xp - xb * yp;
    let b = xa * yp - ya * xp;
    if a % det != 0 || b % det != 0 {
      continue;
    }
    sum += 3 * a / det + b / det;
  }
  sum
}
