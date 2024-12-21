use std::collections::HashMap;
use crate::*;

pub fn main(input: &str, part2: bool) -> i64 {
  let lut = HashMap::from([
    (('A', 'v'), "<vA"),
    (('v', '<'), "<A"),
    (('<', '<'), "A"),
    (('<', 'A'), ">>^A"),
    (('A', '<'), "v<<A"),
    (('<', 'v'), ">A"),
    (('A', '>'), "vA"),
    (('>', '>'), "A"),
    (('>', '^'), "<^A"),
    (('^', 'A'), ">A"),
    (('^', '>'), "v>A"),
    (('>', 'A'), "^A"),
    (('A', '0'), "<A"),
    (('A', 'A'), "A"),
    (('0', '0'), "A"),
    (('v', 'A'), "^>A"),
    (('A', '^'), "<A"),
    (('0', 'A'), ">A"),
    (('<', '^'), ">^A"),
    (('^', '<'), "v<A"),
    (('>', 'v'), "<A"),
    (('v', '>'), ">A"),
    (('0', '1'), "^<A"),
    (('1', 'A'), ">>vA"),
    (('0', '2'), "^A"),
    (('2', 'A'), "v>A"),
    (('0', '3'), "^>A"),
    (('3', 'A'), "vA"),
    (('^', '^'), "A"),
    (('0', '4'), "^^<A"),
    (('v', 'v'), "A"),
    (('4', 'A'), ">>vvA"),
    (('0', '5'), "^^A"),
    (('5', 'A'), "vv>A"),
    (('0', '6'), "^^>A"),
    (('6', 'A'), "vvA"),
    (('0', '7'), "^^^<A"),
    (('7', 'A'), ">>vvvA"),
    (('0', '8'), "^^^A"),
    (('8', 'A'), "vvv>A"),
    (('0', '9'), "^^^>A"),
    (('9', 'A'), "vvvA"),
    (('A', '1'), "^<<A"),
    (('1', '0'), ">vA"),
    (('1', '1'), "A"),
    (('1', '2'), ">A"),
    (('1', '3'), ">>A"),
    (('1', '4'), "^A"),
    (('1', '5'), "^>A"),
    (('1', '6'), "^>>A"),
    (('1', '7'), "^^A"),
    (('1', '8'), "^^>A"),
    (('1', '9'), "^^>>A"),
    (('A', '2'), "<^A"),
    (('2', '0'), "vA"),
    (('2', '1'), "<A"),
    (('2', '2'), "A"),
    (('2', '3'), ">A"),
    (('2', '4'), "<^A"),
    (('2', '5'), "^A"),
    (('2', '6'), "^>A"),
    (('2', '7'), "<^^A"),
    (('2', '8'), "^^A"),
    (('2', '9'), "^^>A"),
    (('A', '3'), "^A"),
    (('3', '0'), "<vA"),
    (('3', '1'), "<<A"),
    (('3', '2'), "<A"),
    (('3', '3'), "A"),
    (('3', '4'), "<<^A"),
    (('3', '5'), "<^A"),
    (('3', '6'), "^A"),
    (('3', '7'), "<<^^A"),
    (('3', '8'), "<^^A"),
    (('3', '9'), "^^A"),
    (('A', '4'), "^^<<A"),
    (('4', '0'), ">vvA"),
    (('4', '1'), "vA"),
    (('4', '2'), "v>A"),
    (('4', '3'), "v>>A"),
    (('4', '4'), "A"),
    (('4', '5'), ">A"),
    (('4', '6'), ">>A"),
    (('4', '7'), "^A"),
    (('4', '8'), "^>A"),
    (('4', '9'), "^>>A"),
    (('A', '5'), "<^^A"),
    (('5', '0'), "vvA"),
    (('5', '1'), "<vA"),
    (('5', '2'), "vA"),
    (('5', '3'), "v>A"),
    (('5', '4'), "<A"),
    (('5', '5'), "A"),
    (('5', '6'), ">A"),
    (('5', '7'), "<^A"),
    (('5', '8'), "^A"),
    (('5', '9'), "^>A"),
    (('A', '6'), "^^A"),
    (('6', '0'), "<vvA"),
    (('6', '1'), "<<vA"),
    (('6', '2'), "<vA"),
    (('6', '3'), "vA"),
    (('6', '4'), "<<A"),
    (('6', '5'), "<A"),
    (('6', '6'), "A"),
    (('6', '7'), "<<^A"),
    (('6', '8'), "<^A"),
    (('6', '9'), "^A"),
    (('A', '7'), "^^^<<A"),
    (('7', '0'), ">vvvA"),
    (('7', '1'), "vvA"),
    (('7', '2'), "vv>A"),
    (('7', '3'), "vv>>A"),
    (('7', '4'), "vA"),
    (('7', '5'), "v>A"),
    (('7', '6'), "v>>A"),
    (('7', '7'), "A"),
    (('7', '8'), ">A"),
    (('7', '9'), ">>A"),
    (('A', '8'), "<^^^A"),
    (('8', '0'), "vvvA"),
    (('8', '1'), "<vvA"),
    (('8', '2'), "vvA"),
    (('8', '3'), "vv>A"),
    (('8', '4'), "<vA"),
    (('8', '5'), "vA"),
    (('8', '6'), "v>A"),
    (('8', '7'), "<A"),
    (('8', '8'), "A"),
    (('8', '9'), ">A"),
    (('A', '9'), "^^^A"),
    (('9', '0'), "<vvvA"),
    (('9', '1'), "<<vvA"),
    (('9', '2'), "<vvA"),
    (('9', '3'), "vvA"),
    (('9', '4'), "<<vA"),
    (('9', '5'), "<vA"),
    (('9', '6'), "vA"),
    (('9', '7'), "<<A"),
    (('9', '8'), "<A"),
    (('9', '9'), "A"),
  ]);
  let mut sum = 0;
  for l in input.lines() {
    sum +=
      parse::<i64>(&l[0..3]) * get_length(&lut, &mut HashMap::new(), l, if part2 { 26 } else { 3 });
  }
  sum
}

fn get_length<'a>(
  lut: &HashMap<(char, char), &'a str>,
  cache: &mut HashMap<(&'a str, i32), i64>,
  target: &'a str,
  depth: i32,
) -> i64 {
  if depth == 0 {
    target.len() as _
  } else if let Some(l) = cache.get(&(target, depth)) {
    *l
  } else {
    let chars = target.chars().to_vec();
    let mut length = get_length(lut, cache, lut[&('A', chars[0])], depth - 1);
    for i in 0..chars.len() - 1 {
      length += get_length(lut, cache, lut[&(chars[i], chars[i + 1])], depth - 1);
    }
    cache.insert((target, depth), length);
    length
  }
}