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
      n: usize,
      mut aa: [usize; n],
  }

  let mut map: BTreeMap<usize, usize> = BTreeMap::new();

  for &a in &aa {
    *map.entry(a).or_insert(0) += 1;
  }

  let mut sum = vec![0; map.last_key_value().unwrap().0 + 1];

  for i in (0..sum.len()).rev() {
    if i == 0 {
      continue;
    }
    sum[i - 1] = sum[i] + map.get(&i).unwrap_or(&0) * i;
  }

  for q in aa {
    print!("{} ", sum[q]);
  }
}
