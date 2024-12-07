use ordermap::OrderSet;
use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let grid = Grid::from_str(input);
  let (x, y) = grid.find(|c| *c == '^').unwrap();
  let mut visited = OrderSet::new();
  visited.insert((x as _, y as _, Dir::Up));
  check(&grid, &mut visited);

  let seen = visited.iter().map(|p| (p.0, p.1)).dedup();

  if part2 {
    let mut obstructions = 0;
    for (x, y) in seen {
      if grid[y][x as usize] == '.' {
        let mut grid = grid.clone();
        grid[y][x as usize] = '#';
        let mut visited = visited.clone();
        visited.split_off(visited.iter().position(|p| p.0 == x && p.1 == y).unwrap());
        if !check(&grid, &mut visited) {
          obstructions += 1;
        }
      }
    }
    obstructions
  } else {
    seen.count()
  }
}

fn check(grid: &Grid<char>, visited: &mut OrderSet<(i32, i32, Dir)>) -> bool {
  let (mut x, mut y, mut dir) = visited.last().unwrap();
  loop {
    let (dx, dy) = dir.xy();
    let (nx, ny) = (x + dx, y + dy);
    if grid.valid_x(nx) && grid.valid_y(ny) {
      if grid[ny][nx as usize] == '#' {
        dir = dir.right();
      } else {
        x = nx as _;
        y = ny as _;
        if !visited.insert((nx, ny, dir)) {
          return false;
        }
      }
    } else {
      return true;
    }
  }
}
