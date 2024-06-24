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
    s: Chars
  }

  let mut alphabet = vec![0usize; 26];
  for &c in &s {
    alphabet[c as usize - 'a' as usize] += 1;
  }

  let mut count = vec![0usize; 101];
  for &a in &alphabet {
    count[a] += 1;
  }

  count.remove(0);

  for i in count {
    if i == 0 || i == 2 {
      continue;
    } else {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
