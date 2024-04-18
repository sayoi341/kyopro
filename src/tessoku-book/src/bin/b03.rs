#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
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

fn main() {
  input! {
      n: usize,
      a: [usize; n],
  }

  for i in 0..n {
    for j in 0..n {
      for k in 0..n {
        if a[i] + a[j] + a[k] == 1000 {
          println!("Yes");
          return;
        }
      }
    }
  }

  println!("No");
}
