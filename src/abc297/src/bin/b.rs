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
    s:Chars,
  }

  let mut b_pos: Vec<usize> = Vec::new();
  let mut k_pos = 0usize;
  let mut r_pos: Vec<usize> = Vec::new();

  for (i, &c) in s.iter().enumerate() {
    match c {
      'B' => b_pos.push(i),
      'K' => k_pos = i,
      'R' => r_pos.push(i),
      _ => (),
    }
  }

  if b_pos[0] % 2 != b_pos[1] % 2 && r_pos[0] < k_pos && k_pos < r_pos[1] {
    println!("Yes");
  } else {
    println!("No");
  }
}
