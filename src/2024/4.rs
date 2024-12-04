use crate::*;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const DIRS: [(isize, isize); 8] = [
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
  let grid = input.lines().map(|l| l.chars().to_vec()).to_vec();
  let (w, h) = (grid[0].len(), grid.len());
  let mut count = 0;
  if part2 {
    for y in 1..h - 1 {
      for x in 1..w - 1 {
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
    for y in 0..h {
      for x in 0..w {
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

fn check(grid: &[Vec<char>], mut x: isize, mut y: isize, dx: isize, dy: isize) -> bool {
  let (w, h) = (grid[0].len() as isize, grid.len() as isize);
  for c in XMAS {
    if (0..w).contains(&x) && (0..h).contains(&y) && grid[y as usize][x as usize] == c {
      x += dx;
      y += dy;
    } else {
      return false;
    }
  }
  true
}
