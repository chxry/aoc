use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let mut parts = input.split("\n\n");
  let rules = parts
    .n(0)
    .lines()
    .map(|l| {
      let mut s = l.split("|").map(parse);
      (s.n(0), s.n(0))
    })
    .to_vec();
  let mut updates = parts
    .n(0)
    .lines()
    .map(|l| l.split(",").map(parse).to_vec())
    .to_vec();
  let mut sum = 0;
  if part2 {
    for u in &mut updates {
      let mut incorrect = false;
      while !check(u, &rules, true) {
        incorrect = true;
      }
      if incorrect {
        sum += u[u.len() / 2];
      }
    }
  } else {
    for u in &mut updates {
      if check(u, &rules, false) {
        sum += u[u.len() / 2]
      }
    }
  }
  sum
}

fn check(update: &mut Vec<i32>, rules: &[(i32, i32)], fix: bool) -> bool {
  for r in rules {
    if let (Some(a), Some(b)) = (idx_of(update, &r.0), idx_of(update, &r.1))
      && a > b
    {
      if fix {
        update.swap(a, b);
      }
      return false;
    }
  }
  return true;
}
