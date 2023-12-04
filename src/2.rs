pub fn main(input: &str, part2: bool) -> u32 {
  let mut total = 0;
  for (i, l) in input.lines().enumerate() {
    let mut possible = true;
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for s in l.split(':').nth(1).unwrap().split(';') {
      for c in s.split(',') {
        let mut p = c.trim().split(' ');
        let i = p.next().unwrap().parse().unwrap();
        let color = p.next().unwrap();
        match color {
          "red" if i > 12 => possible = false,
          "green" if i > 13 => possible = false,
          "blue" if i > 14 => possible = false,
          _ => {}
        }
        match color {
          "red" => red = red.max(i),
          "green" => green = green.max(i),
          "blue" => blue = blue.max(i),
          _ => {}
        }
      }
    }
    if !part2 && possible {
      total += i + 1;
    } else if part2 {
      total += red * green * blue;
    }
  }
  total as _
}
