#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [usize; n]
  }

  let mut a = VecDeque::from_iter(a);

  let mut ans = 0;

  while !a.is_empty() {
    let mut seat = k;
    while let Some(team) = a.pop_front() {
      if seat >= team {
        seat -= team;
      } else {
        a.push_front(team);
        break;
      }
    }
    ans += 1;
  }

  println!("{}", ans);
}
