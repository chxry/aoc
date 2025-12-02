use std::collections::{HashSet, VecDeque};
use crate::*;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const SIZE: i32 = 70;

pub fn main(input: &str, part2: bool) -> String {
  let mut bytes = input.lines().map(|s| {
    let p = s.split(",").map(parse::<i32>).to_arr::<2>();
    (p[0], p[1])
  });

  if part2 {
    for i in 1.. {
      if traverse(HashSet::from_iter(bytes.clone().take(i))) == 0 {
        let c = bytes.n(i - 1);
        return format!("{},{}", c.0, c.1);
      }
    }
    ":(".to_string()
  } else {
    format!("{}", traverse(HashSet::from_iter(bytes.clone().take(1024))))
  }
}

fn traverse(mut bytes: HashSet<(i32, i32)>) -> i32 {
  let mut queue = VecDeque::from([(0, 0, 0)]);
  while let Some((x, y, n)) = queue.pop_front() {
    if x == SIZE && y == SIZE {
      return n;
    }
    for (dx, dy) in DIRS {
      let (nx, ny) = (x + dx, y + dy);
      if (0..=SIZE).contains(&nx) && (0..=SIZE).contains(&ny) && !bytes.contains(&(nx, ny)) {
        bytes.insert((nx, ny));
        queue.push_back((nx, ny, n + 1));
      }
    }
  }
  0
}
