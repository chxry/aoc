use regex::Regex;

pub fn main(input: &str, part2: bool) -> u32 {
  let res = [
    Regex::new(r"mul\((\d+),(\d+)\)").unwrap(),
    Regex::new(r"do\(\)").unwrap(),
    Regex::new(r"don't\(\)").unwrap(),
  ];
  let mut matches = vec![];
  for (i, re) in res.iter().enumerate() {
    for c in re.captures_iter(input) {
      matches.push((i, c.get(0).unwrap().start(), c));
    }
  }
  matches.sort_by_key(|m| m.1);
  let mut enabled = true;
  let mut sum = 0;
  for (i, _, c) in matches {
    match i {
      0 if enabled => sum += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap(),
      1 if part2 => enabled = true,
      2 if part2 => enabled = false,
      _ => {}
    }
  }
  sum
}
