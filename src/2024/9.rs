use crate::*;

pub fn main(input: &str, part2: bool) -> i64 {
  let digits = input.chars().map(|c| c.to_digit(10).unwrap());

  let mut data = vec![];
  if part2 {
    let mut blocks = vec![];
    for (i, n) in digits.enumerate() {
      if i % 2 == 0 {
        blocks.push(((i / 2) as i64, n));
      } else {
        blocks.push((-1, n));
      }
    }
    for i in 0..blocks.len() {
      let i = blocks.len() - i - 1;
      let block = blocks[i];
      if block.0 == -1 {
        continue;
      }
      for j in 0..i {
        let free = blocks[j];
        if free.0 == -1 && free.1 >= block.1 {
          blocks[j].1 = free.1 - block.1;
          blocks[i].0 = -1;
          blocks.insert(j, block);
          break;
        }
      }
    }

    // instead of this i could be smart
    for (n, l) in blocks {
      for _ in 0..l {
        data.push(n as _);
      }
    }
  } else {
    for (i, n) in digits.enumerate() {
      for _ in 0..n {
        if i % 2 == 0 {
          data.push((i / 2) as i64);
        } else {
          data.push(-1);
        }
      }
    }

    let mut free = 0;
    for i in 1..data.len() {
      let i = data.len() - i;
      while data[free] != -1 {
        free += 1;
      }
      if i < free {
        break;
      }
      data.swap(free, i);
    }
  }
  data
    .iter()
    .enumerate()
    .filter_map(|(i, x)| x.is_positive().then_some(i as i64 * x))
    .sum()
}
