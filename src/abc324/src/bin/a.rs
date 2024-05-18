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
    n: usize,
    a: [usize; n]
  }

  let first = a[0];

  for aa in a.iter().skip(1) {
    if aa != &first {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
