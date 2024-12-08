use crate::*;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(i32, i32); 8] = [
  (1, 0),
  (1, 1),
  (0, 1),
  (-1, 1),
  (-1, 0),
  (-1, -1),
  (0, -1),
  (1, -1),
];

pub fn main(input: &str, part2: bool) -> i32 {
  let grid = Grid::from_str(input);
  let mut count = 0;
  if part2 {
    for y in 1..grid.height() - 1 {
      for x in 1..grid.width() - 1 {
        if grid[y][x] == 'A' {
          let a = (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
            || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M');
          let b = (grid[y + 1][x - 1] == 'M' && grid[y - 1][x + 1] == 'S')
            || (grid[y + 1][x - 1] == 'S' && grid[y - 1][x + 1] == 'M');
          if a && b {
            count += 1;
          }
        }
      }
    }
  } else {
    for y in 0..grid.height() {
      for x in 0..grid.width() {
        for (dx, dy) in DIRS {
          if check(&grid, x as _, y as _, dx, dy) {
            count += 1;
          }
        }
      }
    }
  }
  count
}

fn check(grid: &Grid<char>, mut x: i32, mut y: i32, dx: i32, dy: i32) -> bool {
  for c in XMAS {
    if grid.valid_coord(x, y) && grid[y][x as usize] == c {
      x += dx;
      y += dy;
    } else {
      return false;
    }
  }
  true
}
