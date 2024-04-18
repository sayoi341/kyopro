#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use proconio::{input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
  vec,
};

fn main() {
  input! {
      a: usize,
      b: usize,
  }

  let mut count: usize = 0;

  for i in a..=b {
    if 100 % i == 0 {
      count += 1;
    }
  }

  println!("{}", if count == 0 { "No" } else { "Yes" });
}
