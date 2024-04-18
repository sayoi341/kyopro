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
      a: [[usize; 9]; 9],
  }

  for i in 0..9 {
    let mut check = vec![false; 9];

    for j in 0..9 {
      if check[a[i][j] - 1] {
        println!("No");
        return;
      }
      check[a[i][j] - 1] = true;
    }
  }

  for i in 0..9 {
    let mut check = vec![false; 9];

    for j in 0..9 {
      if check[a[j][i] - 1] {
        println!("No");
        return;
      }

      check[a[j][i] - 1] = true;
    }
  }

  for bx in 0..3 {
    for by in 0..3 {
      let mut check = vec![false; 9];

      for i in 0..3 {
        for j in 0..3 {
          if check[a[bx * 3 + i][by * 3 + j] - 1] {
            println!("No");
            return;
          }

          check[a[bx * 3 + i][by * 3 + j] - 1] = true;
        }
      }
    }
  }

  println!("Yes");
}
