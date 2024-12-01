pub fn main(input: &str, part2: bool) -> u32 {
  let digits = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
  ];
  let mut total = 0;
  for l in input.lines() {
    let digits: Vec<_> = l
      .chars()
      .enumerate()
      .filter_map(|(i, c)| match c.to_digit(10) {
        Some(d) => Some(d),
        _ if part2 => digits
          .iter()
          .enumerate()
          .find_map(|(j, d)| l[i..].starts_with(d).then_some(j as u32 + 1)),
        _ => None,
      })
      .collect();
    total += digits.first().unwrap() * 10 + digits.last().unwrap();
  }
  total
}
