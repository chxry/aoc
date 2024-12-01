pub fn main(input: &str, part2: bool) -> u32 {
  let maze: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
  let (mut x, mut y) = maze
    .iter()
    .enumerate()
    .find_map(|(y, l)| l.iter().position(|c| *c == 'S').map(|x| (x, y)))
    .unwrap();
  let mut steps = 0;
  let mut dir = Dir::Left; // set by inspection :3
  loop {
    steps += 1;
    match dir {
      Dir::Up => y -= 1,
      Dir::Down => y += 1,
      Dir::Left => x -= 1,
      Dir::Right => x += 1,
    };
    match maze[y][x] {
      '|' | '-' => {}
      '7' => match dir {
        Dir::Right => dir = Dir::Down,
        Dir::Up => dir = Dir::Left,
        _ => unreachable!(),
      },
      'L' => match dir {
        Dir::Left => dir = Dir::Up,
        Dir::Down => dir = Dir::Right,
        _ => unreachable!(),
      },
      'J' => match dir {
        Dir::Right => dir = Dir::Up,
        Dir::Down => dir = Dir::Left,
        _ => unreachable!(),
      },
      'F' => match dir {
        Dir::Left => dir = Dir::Down,
        Dir::Up => dir = Dir::Right,
        _ => unreachable!(),
      },
      'S' => break,
      _ => unreachable!(),
    };
  }
  steps / 2
}

enum Dir {
  Up,
  Down,
  Left,
  Right,
}
