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
  vec,
};
use superslice::Ext;

#[fastout]
fn main() {
  input! {
    n: usize,
    q: usize,
  }

  let mut vec = VecDeque::new();
  for i in 1..=n {
    vec.push_back(i);
  }

  let mut called: BTreeSet<usize> = BTreeSet::new();

  for _i in 0..q {
    input! {
      e: usize,
    }

    if e == 1 {
      //受付に呼ばれていない人のうち、最も小さい番号の人が受付に呼ばれる。
      let x = vec.pop_front().unwrap();
      called.insert(x);
    } else if e == 2 {
      //人 x が初めて受付に行く。(ここで、人 x はすでに 1 回以上受付に呼ばれている。)
      input! {
        x: usize,
      }

      called.remove(&x);
    } else {
      //すでに受付に呼ばれているが受付に行っていない人のうち、最も小さい番号の人が再度呼ばれる。
      println!("{}", called.first().unwrap());
    }
  }
}
