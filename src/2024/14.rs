use std::collections::HashSet;
use regex::Regex;
use crate::*;

pub fn main(input: &str, part2: bool) -> i32 {
  let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
  let mut robots = re
    .captures_iter(input)
    .map(|c| c.extract().1.map(parse::<i32>))
    .to_vec();
  if part2 {
    let mut i = 0;
    loop {
      for [x, y, dx, dy] in &mut robots {
        *x = (*x + *dx).rem_euclid(101);
        *y = (*y + *dy).rem_euclid(103);
      }
      i += 1;
      // for y in 0..103 {
      //   for x in 0..101 {
      //     if robots.iter().any(|[rx, ry, _, _]| *rx == x && *ry == y) {
      //       print!("#");
      //     } else {
      //       print!(" ")
      //     }
      //   }
      //   println!();
      // }
      // checking for no overlaps seems to work
      let mut positions = HashSet::new();
      if !robots
        .iter()
        .any(|[rx, ry, _, _]| !positions.insert((rx, ry)))
      {
        break;
      }
    }
    i
  } else {
    let (mut tl, mut tr, mut bl, mut br) = (0, 0, 0, 0);
    for [mut x, mut y, dx, dy] in robots {
      x = (x + dx * 100).rem_euclid(101);
      y = (y + dy * 100).rem_euclid(103);
      match (x, y) {
        (..=49, ..=50) => tl += 1,
        (51.., ..=50) => tr += 1,
        (..=49, 52..) => bl += 1,
        (51.., 52..) => br += 1,
        _ => {}
      }
    }
    tl * tr * bl * br
  }
}
