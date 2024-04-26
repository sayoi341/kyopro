#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::*;
use rand_core::le;
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

// インタラクティブ問はfastoutダメ
fn main() {
  input_interactive! {
    n: usize,
  }

  let mut from_zero = 0;
  let mut to_one = n;

  // 0(from_zero),0,0 ... mid ... 1,1,1(to_one)
  // の場合を考える

  // midが0の場合
  // 0,0,0 ... 0(from_zero), ... 1,1,1(to_one)

  // midが1の場合
  // 0(from_zero),0,0 ... 1(to_one) ... 1,1,1

  while to_one.abs_diff(from_zero) > 1 {
    let mid = (from_zero + to_one) / 2;

    println!("? {}", mid);
    input_interactive! {
      midnum: usize,
    }

    if midnum == 1 {
      to_one = mid;
    } else {
      from_zero = mid;
    }
  }

  println!("! {}", from_zero);
}
