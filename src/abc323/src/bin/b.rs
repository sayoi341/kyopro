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
    s: [Chars; n]
  }

  let mut ans = (0..n).map(|i| (i, 0)).collect::<Vec<_>>();

  for (i, line) in s.iter().enumerate() {
    for c in line {
      if *c == 'o' {
        ans[i].1 += 1;
      }
    }
  }

  ans.sort_by(|a, b| b.1.cmp(&a.1));

  println!("{}", ans.iter().map(|(i, _)| i + 1).join(" "));
}
