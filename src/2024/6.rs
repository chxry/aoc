use std::collections::HashSet;
use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  if part2 {
    let mut obstructions = 0;
    for y in 0..grid.height() {
      for x in 0..grid.width() {
        if grid[y][x] == '.' {
          let mut grid = grid.clone();
          grid[y][x] = '#';
          if check(&grid).is_none() {
            obstructions += 1;
          }
        }
      }
    }
    obstructions
  } else {
    check(&grid).unwrap()
  }
}

fn check(grid: &Grid<char>) -> Option<usize> {
  let (mut x, mut y) = grid.find(|c| *c == '^').unwrap();
  let mut dir = Dir::Up;
  let mut positions = HashSet::new();
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
        positions.insert((nx, ny));
        if !visited.insert((nx, ny, dir)) {
          return None;
        }
      }
    } else {
      return Some(positions.len());
    }
  }
}
