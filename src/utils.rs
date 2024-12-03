use std::fmt::Debug;
use std::error::Error;
use std::ops::{Add, Sub, Mul, Div, Rem};

pub fn lcm<N: Num>(n: &[N]) -> N {
  if n.len() == 1 {
    return n[0];
  }
  let a = n[0];
  let b = lcm(&n[1..]);
  a * b / hcf(a, b)
}

pub fn hcf<N: Num>(a: N, b: N) -> N {
  if b == N::n(0) {
    return a;
  }
  hcf(b, a % b)
}

pub fn parse(s: &str) -> i32 {
  s.parse().expect("parse failed")
}

pub trait IteratorExt<I> {
  fn n<N: TryInto<usize, Error = E> + Debug + Copy, E: Error>(&mut self, i: N) -> I;
  fn to_vec(self) -> Vec<I>;
}

impl<I, T: Iterator<Item = I> + Clone> IteratorExt<I> for T {
  fn n<N: TryInto<usize, Error = E> + Debug + Copy, E: Error>(&mut self, i: N) -> I {
    self
      .nth(i.try_into().unwrap())
      .expect(&format!("index {:?} out of bounds", i))
  }

  fn to_vec(self) -> Vec<I> {
    self.collect()
  }
}

pub trait Num:
  Eq
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
  + Rem<Output = Self>
  + Copy
{
  fn n(i: isize) -> Self;
}

impl<
    T: Eq
      + Add<Output = Self>
      + Sub<Output = Self>
      + Mul<Output = Self>
      + Div<Output = Self>
      + Rem<Output = Self>
      + Copy
      + TryFrom<isize, Error = E>,
    E: Error,
  > Num for T
{
  fn n(i: isize) -> Self {
    i.try_into().unwrap()
  }
}
