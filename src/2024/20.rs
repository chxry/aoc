use crate::*;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  let path = traverse(&grid);
  let mut sum = 0;
  let d = if part2 { 20 } else { 2 };
  for i in 0..path.len() {
    let (x, y) = path[i];
    for dy in -d..=d {
      for dx in -d..=d {
        let (nx, ny) = (x + dx, y + dy);
        let len = dx.abs() + dy.abs();
        if len <= d {
          if let Some(skip) = path.iter().position(|c| *c == (nx, ny)) {
            let save = skip as i32 - i as i32 - len;
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

fn traverse(grid: &Grid<char>) -> Vec<(i32, i32)> {
  let s = grid.find(|c| *c == 'S').unwrap();
  let (mut x, mut y) = (s.0 as _, s.1 as _);
  let mut path = vec![];
  loop {
    path.push((x, y));
    if grid[y][x as usize] == 'E' {
      break;
    }
    for (dx, dy) in DIRS {
      let (nx, ny) = (x + dx, y + dy);
      if grid[ny][nx as usize] != '#' && !path.iter().any(|c| *c == (nx, ny)) {
        x = nx;
        y = ny;
        break;
      }
    }
  }
  path
}
