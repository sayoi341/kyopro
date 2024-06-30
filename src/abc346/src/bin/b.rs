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
    w: usize,
    b: usize,
  }

  let piano = "wbwbwwbwbwbw".chars().collect::<Vec<_>>();
  let l = w + b;
  for i in 0..12 {
    let mut wcnt = 0;
    let mut bcnt = 0;
    for j in 0..l {
      match piano[(i + j) % 12] {
        'w' => wcnt += 1,
        'b' => bcnt += 1,
        _ => unreachable!(),
      }
    }
    if wcnt == w && bcnt == b {
      println!("Yes");
      return;
    }
  }

  println!("No");
}
