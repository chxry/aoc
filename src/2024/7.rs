use crate::*;

pub fn main(input: &str, part2: bool) -> u64 {
  let mut sum = 0;
  for l in input.lines() {
    let parts = l.split(":").to_arr::<2>();
    let target = parse(parts[0]);
    let nums = parts[1].trim().split(" ").map(parse).to_vec();
    if check(target, &nums, part2) {
      sum += target;
    }
  }
  sum
}

fn check(target: u64, nums: &[u64], part2: bool) -> bool {
  if nums[0] > target {
    return false;
  }
  if nums.len() == 1 {
    target == nums[0]
  } else {
    let prod = [&[nums[0] * nums[1]], &nums[2..]].concat();
    let sum = [&[nums[0] + nums[1]], &nums[2..]].concat();
    check(target, &prod, part2)
      || check(target, &sum, part2)
      || (part2 && {
        let concat = [&[parse(&format!("{}{}", nums[0], nums[1]))], &nums[2..]].concat();
        check(target, &concat, part2)
      })
  }
}
