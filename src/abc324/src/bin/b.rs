#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use num_traits::Pow;
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
    mut n: usize,
  }

  loop {
    if n % 2 != 0 {
      break;
    }
    n /= 2;
  }

  loop {
    if n % 3 != 0 {
      break;
    }
    n /= 3;
  }

  println!("{}", if n == 1 { "Yes" } else { "No" });
}
