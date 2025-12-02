use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let mut safe_num = 0;
  for l in input.lines() {
    let nums = l.split(" ").map(parse).to_vec();
    if part2 {
      for n in 0..nums.len() {
        let mut nums = nums.clone();
        nums.remove(n);
        if safe(&nums) {
          safe_num += 1;
          break;
        }
      }
    } else if safe(&nums) {
      safe_num += 1;
    }
  }
  safe_num
}

fn safe(nums: &[i32]) -> bool {
  let dir = nums[0] < nums[nums.len() - 1];
  for i in 1..nums.len() {
    if !((1..=3).contains(&nums[i].abs_diff(nums[i - 1])) && (nums[i - 1] < nums[i]) == dir) {
      return false;
    }
  }
  true
}
