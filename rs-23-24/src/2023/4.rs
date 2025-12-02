pub fn main(input: &str, part2: bool) -> u32 {
  let mut total = 0;
  let mut copies = vec![1u32; input.lines().count()];
  for (i, l) in input.lines().enumerate() {
    let nums = l
      .split([':', '|'])
      .map(|s| s.split_whitespace().collect::<Vec<_>>())
      .collect::<Vec<_>>();
    let mut card = 0;
    for w in &nums[1] {
      if nums[2].contains(w) {
        card += 1;
      }
    }
    if part2 {
      for s in 0..card as _ {
        if i + 1 + s < copies.len() {
          copies[i + 1 + s] += copies[i];
        }
      }
    } else {
      total += if card == 0 { 0 } else { 2i32.pow(card - 1) };
    }
  }
  if part2 {
    copies.into_iter().sum()
  } else {
    total as _
  }
}
