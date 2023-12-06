pub fn main(input: &str, part2: bool) -> u32 {
  // modern cpus are too fast for this challenge
  let nums = if part2 {
    input.replace(' ', "").replace(':', " ")
  } else {
    input.to_string()
  }
  .split_whitespace()
  .filter_map(|s| s.parse().ok())
  .collect::<Vec<usize>>();
  let times = &nums[0..nums.len() / 2];
  let distances = &nums[nums.len() / 2..nums.len()];
  let mut prod = 1;
  for i in 0..nums.len() / 2 {
    let mut n = 0;
    let time = times[i];
    for t in 1..time {
      if t * (time - t) > distances[i] {
        n += 1;
      }
    }
    prod *= n;
  }
  prod
}
