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
      _n: usize,
      q: usize,
      s: Chars,
      lr: [(usize, usize);q],
  }

  let mut sum_sum = vec![0; s.len()];

  for i in 0..s.len() - 1 {
    if s[i] == s[i + 1] {
      sum_sum[i + 1] = sum_sum[i] + 1;
    } else {
      sum_sum[i + 1] = sum_sum[i];
    }
  }

  for (l, r) in lr {
    println!("{}", sum_sum[r - 1] - sum_sum[l - 1]);
  }
}
