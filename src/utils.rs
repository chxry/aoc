use std::str::FromStr;
use std::hash::Hash;
use std::fmt::Debug;
use std::error::Error;
use std::ops::{Add, Sub, Mul, Div, Rem, Index, IndexMut};
use ordermap::OrderSet;

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

pub fn parse<F: FromStr>(s: &str) -> F {
  s.parse()
    .unwrap_or_else(|_| panic!("parsing {:?} failed", s))
}

#[derive(Clone)]
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

impl<T: Clone> Grid<T> {
  pub fn new(default: T, width: usize, height: usize) -> Self {
    Self {
      data: vec![default; width * height],
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

  pub fn valid_coord<N: Idx>(&self, (x, y): (N, N)) -> bool { self.valid_x(x) && self.valid_y(y) }

  pub fn enumerate(&self) -> impl Iterator<Item = ((usize, usize), &T)> {
    self
      .data
      .iter()
      .enumerate()
      .map(|(i, t)| ((i % self.width, i / self.width), t))
  }

  pub fn find<P: Fn(&T) -> bool>(&self, predicate: P) -> Option<(usize, usize)> {
    self
      .data
      .iter()
      .position(predicate)
      .map(|i| (i % self.width, i / self.width))
  }

  pub fn map<U, F: Fn(T) -> U>(self, f: F) -> Grid<U> {
    Grid {
      data: self.data.into_iter().map(f).to_vec(),
      width: self.width,
    }
  }
}

impl<T, N: Idx> Index<N> for Grid<T> {
  type Output = [T];

  fn index(&self, i: N) -> &[T] {
    let start = i.try_into().unwrap() * self.width;
    &self.data[start..start + self.width]
  }
}

impl<T, N: Idx> IndexMut<N> for Grid<T> {
  fn index_mut(&mut self, i: N) -> &mut [T] {
    let start = i.try_into().unwrap() * self.width;
    &mut self.data[start..start + self.width]
  }
}

pub trait IteratorExt<I> {
  fn n<N: Idx>(&mut self, i: N) -> I;
  fn idx_of<J: PartialEq<I>>(&mut self, item: J) -> Option<usize>;
  fn to_vec(self) -> Vec<I>;
  fn to_arr<const N: usize>(self) -> [I; N];
  fn dedup(self) -> impl Iterator<Item = I>
  where
    I: Eq + Hash;
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

  fn dedup(self) -> impl Iterator<Item = I>
  where
    I: PartialEq + Eq + Hash,
  {
    self.collect::<OrderSet<I>>().into_iter()
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

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Dir {
  Up,
  Right,
  Down,
  Left,
}

impl Dir {
  pub fn xy(&self) -> (i32, i32) {
    match self {
      Dir::Up => (0, -1),
      Dir::Right => (1, 0),
      Dir::Down => (0, 1),
      Dir::Left => (-1, 0),
    }
  }

  pub fn right(&self) -> Self {
    match self {
      Dir::Up => Dir::Right,
      Dir::Right => Dir::Down,
      Dir::Down => Dir::Left,
      Dir::Left => Dir::Up,
    }
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
