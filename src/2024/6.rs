use std::collections::HashSet;
use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  let visited = check(&grid)
    .unwrap()
    .into_iter()
    .map(|p| (p.0, p.1))
    .dedup();
  if part2 {
    let mut obstructions = 0;
    for (x, y) in visited {
      if grid[y][x as usize] == '.' {
        let mut grid = grid.clone();
        grid[y][x as usize] = '#';
        if check(&grid).is_none() {
          obstructions += 1;
        }
      }
    }
    obstructions
  } else {
    visited.count()
  }
}

fn check(grid: &Grid<char>) -> Option<HashSet<(i32, i32, Dir)>> {
  let (mut x, mut y) = grid.find(|c| *c == '^').unwrap();
  let mut dir = Dir::Up;
  let mut visited = HashSet::new();
  loop {
    let (dx, dy) = dir.xy();
    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
    if grid.valid_x(nx) && grid.valid_y(ny) {
      if grid[ny][nx as usize] == '#' {
        dir = dir.right();
      } else {
        x = nx as _;
        y = ny as _;
        if !visited.insert((nx, ny, dir)) {
          return None;
        }
      }
    } else {
      return Some(visited);
    }
  }
}
