pub fn main(input: &str, part2: bool) -> usize {
  let mut blocks = input.split("\n\n");
  let seeds = blocks
    .next()
    .unwrap()
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect::<Vec<usize>>();
  let mut maps = vec![];
  for b in blocks {
    let mut map = vec![];
    for l in &b.lines().collect::<Vec<_>>()[1..] {
      let nums = l
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>();
      map.push(nums);
    }
    maps.push(map);
  }

  let mut lowest = usize::MAX;
  if part2 {
    // really slow
    for [s, l] in seeds.array_chunks::<2>() {
      check_lowest(*s..s + l, &maps, &mut lowest);
    }
  } else {
    check_lowest(seeds.into_iter(), &maps, &mut lowest)
  }
  lowest
}

fn check_lowest<I: Iterator<Item = usize>>(
  seeds: I,
  maps: &Vec<Vec<Vec<usize>>>,
  lowest: &mut usize,
) {
  for mut s in seeds {
    for m in maps {
      for r in m {
        if s >= r[1] && s < r[1] + r[2] {
          s = r[0] + s - r[1];
          break;
        }
      }
    }
    *lowest = (*lowest).min(s);
  }
}
