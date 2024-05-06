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
    n: usize,
    m: usize,
    mut s: [Chars; n]
  }

  s.sort();

  while {
    let mut ok = true;

    for i in 0..n - 1 {
      let mut count = 0;
      for j in 0..m {
        if s[i][j] != s[i + 1][j] {
          count += 1;
        }
      }
      if count != 1 {
        ok = false;
      }
    }
    if ok {
      println!("Yes");
      return;
    }
    s.next_permutation()
  } {}

  println!("No");
}
