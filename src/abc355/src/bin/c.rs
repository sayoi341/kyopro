#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

fn main() {
  input! {
    n:usize,
    t:usize,
    turna:[Usize1;t],
  }

  let mut row = vec![0; n];
  let mut col = vec![0; n];
  let mut diag = vec![0; 2];

  for (i, t) in turna.iter().enumerate() {
    let x = t / n;
    let y = t % n;
    row[x] += 1;
    col[y] += 1;

    // check
    // row
    if row[x] == n {
      println!("{}", i + 1);
      return;
    }

    // col
    if col[y] == n {
      println!("{}", i + 1);
      return;
    }

    // diag
    if x == y {
      diag[0] += 1;
      if diag[0] == n {
        println!("{}", i + 1);
        return;
      }
    }

    if x + y == n - 1 {
      diag[1] += 1;
      if diag[1] == n {
        println!("{}", i + 1);
        return;
      }
    }
  }

  println!("-1");
}
