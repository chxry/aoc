use std::collections::HashMap;
use crate::*;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  let path = traverse(&grid);
  let mut sum = 0;
  let d = if part2 { 20 } else { 2 };
  for ((x, y), i) in &path {
    for dy in -d..=d {
      for dx in -d..=d {
        let (nx, ny) = (x + dx, y + dy);
        let len = dx.abs() + dy.abs();
        if len <= d {
          if let Some(skip) = path.get(&(nx, ny)) {
            let save = skip - i - len;
            if save >= 100 {
              sum += 1;
            }
          }
        }
      }
    }
  }
  sum
}

fn traverse(grid: &Grid<char>) -> HashMap<(i32, i32), i32> {
  let s = grid.find(|c| *c == 'S').unwrap();
  let (mut x, mut y) = (s.0 as _, s.1 as _);
  let mut path = HashMap::new();
  loop {
    path.insert((x, y), path.len() as _);
    if grid[y][x as usize] == 'E' {
      break;
    }
    for (dx, dy) in DIRS {
      let (nx, ny) = (x + dx, y + dy);
      if grid[ny][nx as usize] != '#' && !path.contains_key(&(nx, ny)) {
        x = nx;
        y = ny;
        break;
      }
    }
  }
  path
}
