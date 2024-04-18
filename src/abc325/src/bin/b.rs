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
      mut wx: [(usize, usize); n]
  }

  let mut count = vec![0; 25];

  for (w, x) in wx {
    // x時差で、9時から18時までの参加できる所に、標準時で参加できる人数を足していく
    for t in 9..18 {
      count[(t + x) % 24] += w;
    }
  }

  println!("{}", count.iter().max().unwrap());
}
