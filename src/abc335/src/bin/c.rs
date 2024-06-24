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
  vec,
};
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    q: usize,
  }

  let mut dragon: VecDeque<(isize, isize)> = VecDeque::with_capacity(n);
  for i in 0..n {
    dragon.push_back((i as isize + 1, 0));
  }

  for _ in 0..q {
    input! {
      t: usize,
    }

    if t == 1 {
      input! {
        c: char,
      }

      dragon.pop_back();

      let first_val = dragon.front().unwrap();

      match c {
        'R' => dragon.push_front((first_val.0 + 1, first_val.1)),
        'L' => dragon.push_front((first_val.0 - 1, first_val.1)),
        'U' => dragon.push_front((first_val.0, first_val.1 + 1)),
        'D' => dragon.push_front((first_val.0, first_val.1 - 1)),
        _ => unreachable!(),
      }
    } else if t == 2 {
      input! {
        p: usize,
      }

      println!("{} {}", dragon[p - 1].0, dragon[p - 1].1);
    }
  }
}
