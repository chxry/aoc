use regex::Regex;
use std::collections::HashMap;
use crate::*;

pub fn main(input: &str, part2: bool) -> i64 {
  let r1 = Regex::new(r"(\w+): (\d)").unwrap();
  let r2 = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
  let mut wires: HashMap<String, u8> = HashMap::from_iter(
    r1.captures_iter(input)
      .map(|c| (c[1].to_string(), parse(&c[2]))),
  );
  let gates = r2
    .captures_iter(input)
    .map(|c| {
      (
        c[1].to_string(),
        c[2].to_string(),
        c[3].to_string(),
        c[4].to_string(),
      )
    })
    .to_vec();
  if part2 {
    // ended up doing it by hand
    // let mut c = "kvj".to_string(); // inspection
    // let mut swapped = vec![];
    // for i in 1..45 {
    //   let (x, y, z) = (
    //     format!("x{:02}", i),
    //     format!("y{:02}", i),
    //     format!("z{:02}", i),
    //   );
    //   let xor1 = find_gate(&x, &y, "XOR", &gates).unwrap();
    //   let and1 = find_gate(&x, &y, "AND", &gates).unwrap();
    // }
    // println!("{:?}", swapped);
    0
  } else {
    loop {
      let mut complete = true;
      for (a, g, b, t) in &gates {
        if wires.contains_key(t) || !wires.contains_key(a) || !wires.contains_key(b) {
          continue;
        }
        complete = false;
        let (a, b) = (wires[a], wires[b]);
        let v = match g.as_ref() {
          "AND" => a & b,
          "OR" => a | b,
          "XOR" => a ^ b,
          _ => panic!("grr"),
        };
        wires.insert(t.clone(), v);
      }
      if complete {
        break;
      }
    }
    let mut sum = 0;
    for (k, v) in wires {
      if let Some(n) = k.strip_prefix('z') {
        sum += 2i64.pow(parse(n)) * v as i64;
      }
    }
    sum
  }
}

fn find_gate(
  a: &str,
  b: &str,
  gate: &str,
  gates: &[(String, String, String, String)],
) -> Option<String> {
  gates.iter().find_map(|g| {
    (((g.0 == a && g.2 == b) || (g.2 == a && g.0 == b)) && g.1 == gate).then(|| g.3.clone())
  })
}

fn find_other(
  a: &str,
  out: &str,
  gate: &str,
  gates: &[(String, String, String, String)],
) -> Option<String> {
  gates.iter().find_map(|g| {
    (g.1 == gate && g.0 == a && g.3 == out)
      .then(|| g.2.clone())
      .or_else(|| (g.1 == gate && g.2 == a && g.3 == out).then(|| g.0.clone()))
  })
}
