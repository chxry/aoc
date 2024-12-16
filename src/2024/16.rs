use std::collections::HashMap;
use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let grid = Grid::from_str(input);
  let (x, y) = grid.find(|c| *c == 'S').unwrap();
  let mut queue = vec![(x as i32, y as i32, Dir::Right, 0)];
  let mut scores: HashMap<(i32, i32, Dir), i32> = HashMap::new();
  while !queue.is_empty() {
    let (i, &(x, y, dir, score)) = queue.iter().enumerate().min_by_key(|(_, x)| x.3).unwrap();
    queue.remove(i);
    if grid[y][x as usize] == 'E' {
      return score;
    }

    let nd = dir.left();
    let ns = score + 1000;
    if scores.get(&(x, y, nd)).is_none_or(|s| *s < ns) && scores.insert((x, y, nd), ns).is_none() {
      queue.push((x, y, nd, ns));
    }

    let nd = dir.right();
    if scores.get(&(x, y, nd)).is_none_or(|s| *s < ns) && scores.insert((x, y, nd), ns).is_none() {
      queue.push((x, y, nd, ns));
    }

    let (dx, dy) = dir.xy();
    let (nx, ny) = (x + dx, y + dy);
    if grid.valid_coord((nx, ny)) && grid[ny][nx as usize] != '#' {
      let ns = score + 1;
      if scores.get(&(nx, ny, dir)).is_none_or(|s| *s < ns)
        && scores.insert((nx, ny, dir), ns).is_none()
      {
        queue.push((nx, ny, dir, ns))
      }
    }
  }
  0
}
