use crate::*;

pub fn main(input: &str, _: bool) -> i32 {
  let mut locks = vec![];
  let mut keys = vec![];
  for p in input.split("\n\n") {
    let lines = p.lines().to_arr::<7>();
    let mut pat = [0, 0, 0, 0, 0];
    for i in 1..6 {
      let chars = lines[i].chars().to_vec();
      for j in 0..5 {
        if chars[j] == '#' {
          pat[j] += 1;
        }
      }
    }
    if lines[0] == "#####" {
      locks.push(pat);
    } else {
      keys.push(pat);
    }
  }
  let mut sum = 0;
  for l in &locks {
    for k in &keys {
      let mut fits = true;
      for i in 0..5 {
        if l[i] + k[i] > 5 {
          fits = false;
          break;
        }
      }
      if fits {
        sum += 1;
      }
    }
  }
  sum
}
