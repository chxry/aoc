use std::collections::{HashMap, HashSet};
use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
  for (c, a) in grid.enumerate() {
    if *a == '.' {
      continue;
    }
    antennas.entry(*a).or_default().push((c.0 as _, c.1 as _));
  }

  let mut antinodes = HashSet::new();
  for c in antennas.values() {
    // write combinations/permutations helpers
    for i in 0..c.len() {
      for j in (i + 1)..c.len() {
        // tuple helpers too
        let d = (c[j].0 - c[i].0, c[j].1 - c[i].1);
        let mut a = c[i];
        let mut b = c[j];
        // this can be collapsed into something nice later
        if part2 {
          while grid.valid_coord(a) {
            antinodes.insert(a);
            a = (a.0 - d.0, a.1 - d.1);
          }
          while grid.valid_coord(b) {
            antinodes.insert(b);
            b = (b.0 + d.0, b.1 + d.1);
          }
        } else {
          a = (a.0 - d.0, a.1 - d.1);
          if grid.valid_coord(a) {
            antinodes.insert(a);
          }
          b = (b.0 + d.0, b.1 + d.1);
          if grid.valid_coord(b) {
            antinodes.insert(b);
          }
        }
      }
    }
  }
  antinodes.len()
}
