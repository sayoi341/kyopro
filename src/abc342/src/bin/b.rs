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
    parr: [usize; n],
    q: usize,
    abarr : [(usize, usize); q]
  }

  for (a, b) in abarr {
    let mut aidx = 0;
    let mut bidx = 0;
    for (idx, p) in parr.iter().enumerate() {
      if *p == a {
        aidx = idx;
      }
      if *p == b {
        bidx = idx;
      }
    }

    println!("{}", if aidx > bidx { b } else { a });
  }
}
