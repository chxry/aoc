fn main() {
  day1(include_str!("../inputs/1.txt"), false);
  day1(include_str!("../inputs/1.txt"), true);
}

fn day1(input: &str, part2: bool) {
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
  println!("{}", total);
}
