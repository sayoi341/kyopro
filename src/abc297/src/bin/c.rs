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
    h: usize,
    w: usize,
  }

  for _i in 0..h {
    input! {
      mut s: Chars,
    }

    for i in 1..w {
      if s[i - 1] == 'T' && s[i] == 'T' {
        s[i - 1] = 'P';
        s[i] = 'C';
      }
    }

    println!("{}", s.into_iter().collect::<String>());
  }
}
