#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

fn main() {
  input! {
    h: usize,
    w: usize,
    a:[Chars; h],
    b:[Chars; h],
  }

  let mut ans = false;

  for s in 0..w {
    for t in 0..h {
      let mut this_time = true;
      for i in 0..w {
        for j in 0..h {
          if a[(j + h - t) % h][(i + w - s) % w] != b[j][i] {
            this_time = false;
          }
        }
      }
      if this_time {
        ans = true;
      }
    }
  }

  println!("{}", if ans { "Yes" } else { "No" })
}
