#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

#[fastout]
fn main() {
  input! {
    n: isize,
    s: Chars,
  }

  let mut ans = -1;
  let mut count = 0;

  for c in s {
    if c == 'o' {
      count += 1;
      ans = max(ans, count);
    } else {
      count = 0;
    }
  }

  println!("{}", if ans == n { -1 } else { ans });
}
