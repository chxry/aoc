use std::fmt::Debug;
use std::error::Error;
use std::ops::{Add, Sub, Mul, Div, Rem, Index};

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
  s.parse()
    .unwrap_or_else(|_| panic!("parsing {:?} failed", s))
}

pub struct Grid<T> {
  pub data: Vec<T>,
  width: usize,
}

impl Grid<char> {
  pub fn from_str(s: &str) -> Self {
    let width = s.lines().n(0).chars().count();
    Self {
      data: s.lines().flat_map(|l| l.chars()).to_vec(),
      width,
    }
  }
}

impl<T> Grid<T> {
  pub fn width(&self) -> usize { self.width }

  pub fn height(&self) -> usize { self.data.len() / self.width }

  pub fn valid_x<N: Idx>(&self, x: N) -> bool {
    x.try_into().is_ok_and(|x| (0..self.width()).contains(&x))
  }

  pub fn valid_y<N: Idx>(&self, y: N) -> bool {
    y.try_into().is_ok_and(|y| (0..self.height()).contains(&y))
  }
}

impl<T, N: Idx> Index<N> for Grid<T> {
  type Output = [T];

  fn index(&self, i: N) -> &[T] {
    let start = i.try_into().unwrap() * self.width;
    &self.data[start..start + self.width]
  }
}

pub trait IteratorExt<I> {
  fn n<N: Idx>(&mut self, i: N) -> I;
  fn idx_of<J: PartialEq<I>>(&mut self, item: J) -> Option<usize>;
  fn to_vec(self) -> Vec<I>;
  fn to_arr<const N: usize>(self) -> [I; N];
}

impl<I, T: Iterator<Item = I>> IteratorExt<I> for T {
  fn n<N: Idx>(&mut self, i: N) -> I {
    self
      .nth(i.try_into().unwrap())
      .unwrap_or_else(|| panic!("index {:?} out of bounds", i))
  }

  fn idx_of<J: PartialEq<I>>(&mut self, item: J) -> Option<usize> { self.position(|x| item == x) }

  fn to_vec(self) -> Vec<I> { self.collect() }

  fn to_arr<const N: usize>(mut self) -> [I; N] {
    self
      .next_chunk()
      .unwrap_or_else(|e| panic!("expected {} values, found {}", N, e.count()))
  }
}

pub trait ListExt<I> {
  fn idx_of<J: PartialEq<I>>(&self, item: J) -> Option<usize>;
  fn find_subs<J: PartialEq<I>>(&self, items: &[J]) -> impl Iterator<Item = usize>;
}

impl<I> ListExt<I> for [I] {
  fn idx_of<J: PartialEq<I>>(&self, item: J) -> Option<usize> { self.iter().idx_of(&item) }

  fn find_subs<J: PartialEq<I>>(&self, items: &[J]) -> impl Iterator<Item = usize> {
    self
      .windows(items.len())
      .enumerate()
      .filter_map(move |(i, x)| (items == x).then_some(i))
  }
}

impl<I> ListExt<I> for Vec<I> {
  fn idx_of<J: PartialEq<I>>(&self, item: J) -> Option<usize> { self.as_slice().idx_of(item) }
  fn find_subs<J: PartialEq<I>>(&self, items: &[J]) -> impl Iterator<Item = usize> {
    self.as_slice().find_subs(items)
  }
}

pub trait Idx: TryInto<usize, Error: Debug> + Debug + Copy {}

impl<T: TryInto<usize, Error: Debug> + Debug + Copy> Idx for T {}

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
  fn n(i: isize) -> Self { i.try_into().unwrap() }
}
