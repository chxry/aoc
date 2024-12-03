use crate::parse;

pub fn main(input: &str, part2: bool) -> i32 {
  let (mut left, mut right): (Vec<_>, Vec<_>) = input
    .lines()
    .map(|l| {
      let mut s = l.split_whitespace().map(parse);
      (s.next().unwrap(), s.next().unwrap())
    })
    .unzip();

  if part2 {
    left
      .iter()
      .map(|n| n * right.iter().filter(|m| n == *m).count() as i32)
      .sum()
  } else {
    left.sort();
    right.sort();

    left
      .iter()
      .zip(right.iter())
      .fold(0, |x, (l, r)| x + (l - r).abs())
  }
}
