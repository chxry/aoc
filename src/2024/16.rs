use std::collections::{HashMap, HashSet};
use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let grid = Grid::from_str(input);
  let (x, y) = grid.find(|c| *c == 'S').unwrap();
  let mut queue = vec![(x as i32, y as i32, Dir::Right, 0)];
  let mut scores = HashMap::new();
  let mut pre: HashMap<_, Vec<_>> = HashMap::new();

  while !queue.is_empty() {
    let (i, &(x, y, dir, score)) = queue.iter().enumerate().min_by_key(|(_, x)| x.3).unwrap();
    queue.remove(i);

    if part2 {
      if score > *scores.get(&(x, y, dir)).unwrap_or(&i32::MAX) {
        continue;
      }
    } else if grid[y][x as usize] == 'E' {
      return score;
    }

    let ns = score + 1000;
    for nd in [dir.left(), dir.right()] {
      if scores.get(&(x, y, nd)).is_none_or(|s| *s >= ns) {
        if scores.insert((x, y, nd), ns).is_none() {
          queue.push((x, y, nd, ns));
        }
        pre.entry((x, y, nd)).or_default().push((x, y, dir));
      }
    }

    let (dx, dy) = dir.xy();
    let (nx, ny) = (x + dx, y + dy);
    if grid.valid_coord((nx, ny)) && grid[ny][nx as usize] != '#' {
      let ns = score + 1;
      if scores.get(&(nx, ny, dir)).is_none_or(|s| *s >= ns) {
        if scores.insert((nx, ny, dir), ns).is_none() {
          queue.push((nx, ny, dir, ns));
        }
        pre.entry((nx, ny, dir)).or_default().push((x, y, dir));
      }
    }
  }

  let (ex, ey) = grid.find(|c| *c == 'E').unwrap();
  let mut end_states = vec![(ex as i32, ey as i32, Dir::Right)];
  let mut tiles = HashSet::new();
  while let Some(current) = end_states.pop() {
    tiles.insert((current.0, current.1));

    for prev in pre.get(&current).unwrap() {
      if (scores.get(prev).unwrap() + if prev.2 != current.2 { 1000 } else { 1 })
        == *scores.get(&current).unwrap()
      {
        end_states.push(*prev);
      }
    }
  }
  tiles.len() as _
}
