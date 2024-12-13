use std::collections::{HashMap, HashSet};
use crate::*;

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIRS2: [(i32, i32); 8] = [
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
  let mut seen = HashMap::new();
  let mut sum = 0;
  for ((x, y), _) in grid.enumerate() {
    if !seen.contains_key(&(x as _, y as _)) {
      let mut perimeter = 4;
      let mut region_seen = HashSet::new();
      region_seen.insert((x as _, y as _));
      silly(&grid, (x as _, y as _), &mut region_seen, &mut perimeter);
      let area = region_seen.len() as i32;
      if !part2 {
        sum += area * perimeter;
      }
      for c in region_seen {
        seen.insert(c, area);
      }
    }
  }
  if part2 {
    for ((x, y), c) in grid.enumerate() {
      let adjs = DIRS2.map(|(dx, dy)| {
        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
        if grid.valid_coord((nx, ny)) {
          grid[ny][nx as usize]
        } else {
          '\0'
        }
      });
      let mut corners = 0;
      // outwards facing
      // ..
      // A.
      if adjs[0] != *c && adjs[2] != *c {
        corners += 1;
      }
      if adjs[2] != *c && adjs[4] != *c {
        corners += 1;
      }
      if adjs[4] != *c && adjs[6] != *c {
        corners += 1;
      }
      if adjs[6] != *c && adjs[0] != *c {
        corners += 1;
      }
      // inwards facing
      // A.
      // AA
      if adjs[1] != *c && adjs[0] == *c && adjs[2] == *c {
        corners += 1;
      }
      if adjs[3] != *c && adjs[2] == *c && adjs[4] == *c {
        corners += 1;
      }
      if adjs[5] != *c && adjs[4] == *c && adjs[6] == *c {
        corners += 1;
      }
      if adjs[7] != *c && adjs[6] == *c && adjs[0] == *c {
        corners += 1;
      }
      sum += seen.get(&(x as _, y as _)).unwrap() * corners;
    }
  }
  sum
}

fn silly(
  grid: &Grid<char>,
  (x, y): (i32, i32),
  seen: &mut HashSet<(i32, i32)>,
  perimeter: &mut i32,
) {
  let c = grid[y][x as usize];
  for (dx, dy) in DIRS {
    let (nx, ny) = (x + dx, y + dy);
    if grid.valid_coord((nx, ny)) && grid[ny][nx as usize] == c && seen.insert((nx, ny)) {
      let mut adj = 0;
      for (d2x, d2y) in DIRS {
        let (nx, ny) = (nx + d2x, ny + d2y);
        if grid.valid_coord((nx, ny)) && grid[ny][nx as usize] == c && seen.contains(&(nx, ny)) {
          adj += 1;
        }
      }
      *perimeter += match adj {
        1 => 2,
        2 => 0,
        3 => -2,
        4 => -4,
        _ => panic!("wtf"),
      };
      silly(grid, (nx, ny), seen, perimeter);
    }
  }
}
