fn main() {
  day1(include_str!("../inputs/1.txt"), false);
  day1(include_str!("../inputs/1.txt"), true);
  day2(include_str!("../inputs/2.txt"), false);
  day2(include_str!("../inputs/2.txt"), true);
  day4(include_str!("../inputs/4.txt"), false);
  day4(include_str!("../inputs/4.txt"), true);
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

fn day2(input: &str, part2: bool) {
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
  println!("{}", total);
}

fn day4(input: &str, part2: bool) {
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
    println!("{}", copies.into_iter().sum::<u32>());
  } else {
    println!("{}", total);
  }
}
