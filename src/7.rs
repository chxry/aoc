use std::collections::HashMap;
use std::cmp::Ordering;

pub fn main(input: &str, part2: bool) -> u32 {
  let mut hands = vec![];
  for c in input.lines() {
    let mut s = c.split_whitespace();
    let cards = s.next().unwrap().chars().map(Card::parse).collect();
    let bid = s.next().unwrap().parse().unwrap();
    hands.push(Hand { cards, bid });
  }
  hands.sort_by(|a, b| {
    if a.ty(part2) > b.ty(part2) || a.ty(part2) == b.ty(part2) && a.cmp(b, part2) {
      Ordering::Greater
    } else {
      Ordering::Less
    }
  });
  let mut total = 0;
  for (i, h) in hands.iter().rev().enumerate() {
    total += h.bid * (i + 1) as u32;
  }
  total
}

#[derive(Debug)]
struct Hand {
  cards: Vec<Card>,
  bid: u32,
}

impl Hand {
  fn ty(&self, part2: bool) -> u32 {
    let mut counts = HashMap::new();
    for c in &self.cards {
      *counts.entry(c).or_insert(0) += 1;
    }
    let jokers = if part2 {
      counts.remove(&Card::J).unwrap_or(0)
    } else {
      0
    };
    if jokers == 5 {
      return 0;
    }
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by_key(|c| c.1);
    sorted.reverse();
    match sorted[0].1 + jokers {
      5 => 0, // five of a kind
      4 => 1, // four of a kind
      3 => match sorted[1].1 {
        2 => 2, // full house
        1 => 3, // three of a kind
        _ => unreachable!(),
      },
      2 => match sorted[1].1 {
        2 => 4, // two pair
        1 => 5, // one pair
        _ => unreachable!(),
      },
      1 => 6, // high card
      _ => unreachable!(),
    }
  }

  fn cmp(&self, other: &Self, part2: bool) -> bool {
    for i in 0..5 {
      if self.cards[i] != other.cards[i] {
        if part2 {
          if self.cards[i] == Card::J {
            return true;
          }
          if other.cards[i] == Card::J {
            return false;
          }
        }
        return self.cards[i] < other.cards[i];
      }
    }
    false
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
  N2,
  N3,
  N4,
  N5,
  N6,
  N7,
  N8,
  N9,
  T,
  J,
  Q,
  K,
  A,
}

impl Card {
  fn parse(s: char) -> Self {
    match s {
      'A' => Self::A,
      'K' => Self::K,
      'Q' => Self::Q,
      'J' => Self::J,
      'T' => Self::T,
      '9' => Self::N9,
      '8' => Self::N8,
      '7' => Self::N7,
      '6' => Self::N6,
      '5' => Self::N5,
      '4' => Self::N4,
      '3' => Self::N3,
      '2' => Self::N2,
      _ => unreachable!(),
    }
  }
}
