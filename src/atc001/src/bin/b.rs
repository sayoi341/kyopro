#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
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
use superslice::Ext;

fn main() {
  input! {
      n: usize,
      q: usize,
  }

  let mut uf: UnionFind<usize> = UnionFind::new(n);

  for _i in 0..q {
    input! {
        p: usize,
        a: usize,
        b: usize,
    }
    if p == 0 {
      uf.union(a, b);
    } else {
      println!("{}", if uf.equiv(a, b) { "Yes" } else { "No" });
    }
  }
}
