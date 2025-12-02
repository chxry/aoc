use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let p = input.split("\n\n").to_arr::<2>();
  if part2 {
    let mut grid = Grid::from_str(
      &p[0]
        .chars()
        .flat_map(|c| match c {
          '#' => ['#', '#'].as_slice(),
          'O' => &['[', ']'],
          '@' => &['@', '.'],
          '\n' => &['\n'],
          _ => &['.', '.'],
        })
        .collect::<String>(),
    );
    let (mut x, mut y) = grid.find(|c| *c == '@').unwrap();
    grid[y][x] = '.';
    for m in p[1].chars() {
      let dir = match m {
        '>' => Dir::Right,
        '^' => Dir::Up,
        '<' => Dir::Left,
        'v' => Dir::Down,
        _ => continue,
      };
      let (dx, dy) = dir.xy();
      let (nx, ny) = (x as i32 + dx, y as i32 + dy);
      match grid[ny][nx as usize] {
        '.' => {
          x = nx as _;
          y = ny as _;
        }
        '[' | ']' => {
          let mut stack = vec![];
          let bx = if grid[ny][nx as usize] == '[' {
            nx
          } else {
            nx - 1
          };
          if !push(&grid, &mut stack, (bx, ny), dir) {
            // 1 pass would be ideal but idk
            for (bx, by) in &stack {
              grid[*by][*bx as usize] = '.';
              grid[*by][*bx as usize + 1] = '.';
            }
            for (bx, by) in &stack {
              let (nx, ny) = (bx + dx, by + dy);
              grid[ny][nx as usize] = '[';
              grid[ny][nx as usize + 1] = ']';
            }
            x = nx as _;
            y = ny as _;
          }
        }
        _ => {}
      }
    }
    grid
      .enumerate()
      .filter(|(_, c)| (**c == '['))
      .map(|((x, y), _)| 100 * y + x)
      .sum()
  } else {
    let mut grid = Grid::from_str(p[0]);
    let (mut x, mut y) = grid.find(|c| *c == '@').unwrap();
    grid[y][x] = '.';
    for m in p[1].chars() {
      let (dx, dy) = match m {
        '>' => Dir::Right,
        '^' => Dir::Up,
        '<' => Dir::Left,
        'v' => Dir::Down,
        _ => continue,
      }
      .xy();
      let (nx, ny) = (x as i32 + dx, y as i32 + dy);
      match grid[ny][nx as usize] {
        '.' => {
          x = nx as _;
          y = ny as _;
        }
        'O' => {
          let (mut n2x, mut n2y) = (nx, ny);
          loop {
            n2x += dx;
            n2y += dy;
            match grid[n2y][n2x as usize] {
              '.' => {
                grid[ny][nx as usize] = '.';
                x = nx as _;
                y = ny as _;
                grid[n2y][n2x as usize] = 'O';
                break;
              }
              '#' => break,
              _ => {}
            }
          }
        }
        _ => {}
      }
    }
    grid
      .enumerate()
      .filter(|(_, c)| (**c == 'O'))
      .map(|((x, y), _)| 100 * y + x)
      .sum()
  }
}

fn push(grid: &Grid<char>, stack: &mut Vec<(i32, i32)>, (x, y): (i32, i32), dir: Dir) -> bool {
  grid[y][x as usize] == '[' && {
    stack.push((x, y));
    match dir {
      Dir::Right => grid[y][x as usize + 2] == '#' || push(grid, stack, (x + 2, y), dir),
      Dir::Left => grid[y][x as usize - 1] == '#' || push(grid, stack, (x - 2, y), dir),
      Dir::Up => {
        grid[y - 1][x as usize] == '#'
          || grid[y - 1][x as usize + 1] == '#'
          || push(grid, stack, (x + 1, y - 1), dir)
          || push(grid, stack, (x, y - 1), dir)
          || push(grid, stack, (x - 1, y - 1), dir)
      }
      Dir::Down => {
        grid[y + 1][x as usize] == '#'
          || grid[y + 1][x as usize + 1] == '#'
          || push(grid, stack, (x + 1, y + 1), dir)
          || push(grid, stack, (x, y + 1), dir)
          || push(grid, stack, (x - 1, y + 1), dir)
      }
    }
  }
}
