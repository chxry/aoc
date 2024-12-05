use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let parts = input.split("\n\n").to_arr::<2>();
  let rules = parts[0]
    .lines()
    .map(|l| l.split("|").map(parse).to_arr())
    .to_vec();
  let mut updates = parts[1]
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

fn check(update: &mut Vec<i32>, rules: &[[i32; 2]], fix: bool) -> bool {
  for r in rules {
    if let (Some(a), Some(b)) = (update.idx_of(r[0]), update.idx_of(r[1]))
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
