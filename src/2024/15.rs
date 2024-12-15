use crate::*;

pub fn main(input: &str, part2: bool) -> usize {
  let p = input.split("\n\n").to_arr::<2>();
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
