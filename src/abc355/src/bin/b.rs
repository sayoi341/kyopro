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
    n: usize,
    m: usize,
    mut aarr: [usize;n],
    mut barr: [usize;m],
  }

  aarr.sort();
  barr.sort();

  let mut carr = vec![];
  carr.extend_from_slice(&aarr);
  carr.extend_from_slice(&barr);

  carr.sort();

  for i in 0..n + m - 1 {
    if aarr.contains(&carr[i]) && aarr.contains(&carr[i + 1]) {
      println!("Yes");
      return;
    }
  }

  println!("No");
}
