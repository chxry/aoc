use std::collections::HashMap;
use crate::lcm;

pub fn main(input: &str, part2: bool) -> usize {
  let lines: Vec<_> = input.lines().collect();
  let mut steps = lines[0].chars().cycle();
  let mut map = HashMap::new();
  for line in &lines[2..] {
    let nodes = line.replace("= ", "").replace(['(', ')', ','], "");
    let mut nodes = nodes.split_whitespace().map(|s| s.to_owned());
    map.insert(
      nodes.next().unwrap(),
      (nodes.next().unwrap(), nodes.next().unwrap()),
    );
  }
  let start = if part2 {
    map
      .keys()
      .filter(|s| s.ends_with('A'))
      .map(|s| s.as_ref())
      .collect()
  } else {
    vec!["AAA"]
  };
  lcm(
    &start
      .into_iter()
      .map(|mut c| {
        let mut step = 0;
        while !c.ends_with('Z') {
          let left = steps.next().unwrap() == 'L';
          let choices = map.get(c).unwrap();
          c = if left { &choices.0 } else { &choices.1 };
          step += 1;
        }
        step
      })
      .collect::<Vec<_>>(),
  )
}
