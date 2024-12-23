use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use crate::*;

pub fn main(input: &str, part2: bool) -> String {
  let mut nodes = HashMap::new();
  for l in input.lines() {
    let [a, b] = l.split("-").to_arr();
    nodes.entry(a).or_insert(HashSet::new()).insert(b);
    nodes.entry(b).or_insert(HashSet::new()).insert(a);
  }
  if part2 {
    let mut max = HashSet::new();
    let p: HashSet<_> = nodes.keys().copied().collect();
    let r = HashSet::new();
    let x = HashSet::new();
    bron(&nodes, r, p, x, &mut max);

    let mut v = max.into_iter().to_vec();
    v.sort();
    v.join(",")
  } else {
    let mut sum = 0;
    // maybe rewrite with bron kerbosch instead of bruteforce
    for s in nodes.keys().combinations(3) {
      let [a, b, c] = s.iter().to_arr();
      if &a[..1] == "t" || &b[..1] == "t" || &c[..1] == "t" {
        if connected(a, b, &nodes) && connected(b, c, &nodes) && connected(a, c, &nodes) {
          sum += 1;
        }
      }
    }
    sum.to_string()
  }
}
fn bron<'a>(
  nodes: &'a HashMap<&'a str, HashSet<&'a str>>,
  r: HashSet<&'a str>,
  p: HashSet<&'a str>,
  x: HashSet<&'a str>,
  max: &mut HashSet<&'a str>,
) {
  if p.is_empty() && x.is_empty() {
    if r.len() > max.len() {
      *max = r;
    }
    return;
  }

  let union_px: HashSet<_> = p.union(&x).copied().collect();
  let pivot = union_px
    .iter()
    .max_by_key(|&&v| nodes.get(v).map_or(0, |neighbors| neighbors.len()));

  let pivot_neighbours = pivot.and_then(|&u| nodes.get(u)).unwrap();
  let p_minus_neighbors = p.difference(pivot_neighbours).copied().to_vec();

  for &v in &p_minus_neighbors {
    let mut new_r = r.clone();
    new_r.insert(v);

    let neighbors = nodes.get(v).unwrap();
    let new_p: HashSet<_> = p.intersection(neighbors).copied().collect();
    let new_x: HashSet<_> = x.intersection(neighbors).copied().collect();

    bron(nodes, new_r, new_p, new_x, max);
  }
}

fn connected(a: &str, b: &str, nodes: &HashMap<&str, HashSet<&str>>) -> bool {
  nodes.get(a).unwrap().contains(b)
}
