use std::collections::HashSet;
use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let grid = Grid::from_str(input).map(|c| c.to_digit(10).unwrap());
  let mut sum = 0;
  for ((x, y), d) in grid.enumerate() {
    if *d == 0 {
      sum += check(&grid, 0, (x as _, y as _), &mut (!part2).then(HashSet::new));
    }
  }
  sum
}

fn check(
  grid: &Grid<u32>,
  n: u32,
  (x, y): (i32, i32),
  visited: &mut Option<HashSet<(i32, i32)>>,
) -> i32 {
  if !grid.valid_coord((x, y))
    || grid[y][x as usize] != n
    || visited.as_mut().is_some_and(|v| !v.insert((x, y)))
  {
    return 0;
  }
  if grid[y][x as usize] == 9 {
    return 1;
  }
  check(grid, n + 1, (x + 1, y), visited)
    + check(grid, n + 1, (x, y + 1), visited)
    + check(grid, n + 1, (x - 1, y), visited)
    + check(grid, n + 1, (x, y - 1), visited)
}
