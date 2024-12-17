use regex::Regex;
use crate::*;

pub fn main(input: &str, part2: bool) -> String {
  let re = Regex::new(r"(\d+,?)+")
    .unwrap()
    .captures_iter(input)
    .to_arr::<4>();
  let mut regs = re[0..3].iter().map(|c| parse::<u64>(&c[0])).to_arr::<3>();
  let program = re[3][0].split(",").map(parse).to_vec();
  if part2 {
    // from observation:
    // - the number of digits is log_8(A)+1
    // - the 'period' of each digit is 8^n
    regs[0] = 8u64.pow(program.len() as u32 - 1);
    'o: loop {
      let out = compute(regs, &program);
      let mut i = program.len();
      while i > 0 {
        i -= 1;
        if out[i] != program[i] {
          break;
        }
        if i == 0 {
          break 'o;
        }
      }
      regs[0] += 8u64.pow(i as _);
    }
    regs[0].to_string()
  } else {
    compute(regs, &program)
      .iter()
      .map(|n| n.to_string())
      .to_vec()
      .join(",")
  }
}

fn compute([mut a, mut b, mut c]: [u64; 3], program: &[u64]) -> Vec<u64> {
  let mut ip = 0;
  let mut out = vec![];
  while ip < program.len() {
    let op = match program[ip + 1] {
      x if x <= 3 => x,
      4 => a,
      5 => b,
      6 => c,
      _ => panic!("grr"),
    };
    match program[ip] {
      0 => a /= 2u64.pow(op as _),
      1 => b ^= program[ip + 1],
      2 => b = op % 8,
      3 if a != 0 => {
        ip = program[ip + 1] as _;
        continue;
      }
      4 => b ^= c,
      5 => out.push(op % 8),
      6 => b = a / 2u64.pow(op as _),
      7 => c = a / 2u64.pow(op as _),
      _ => {}
    }
    ip += 2;
  }
  out
}
