pub fn main(input: &str, part2: bool) -> i32 {
  let mut total = 0;
  for line in input.lines() {
    let mut nums = vec![line
      .split_whitespace()
      .map(|s| s.parse().unwrap())
      .collect::<Vec<i32>>()];
    while !nums.last().unwrap().iter().all(|n| *n == 0) {
      nums.push(
        nums
          .last()
          .unwrap()
          .windows(2)
          .map(|s| s[1] - s[0])
          .collect(),
      );
    }
    for i in (0..nums.len() - 1).rev() {
      if part2 {
        let n = nums[i][0] - nums[i + 1][0];
        nums[i].insert(0, n);
      } else {
        let n = nums[i + 1].last().unwrap() + nums[i].last().unwrap();
        nums[i].push(n);
      }
    }
    total += if part2 {
      nums[0][0]
    } else {
      *nums[0].last().unwrap()
    };
  }
  total
}
