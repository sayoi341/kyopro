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
    ss: Chars,
    ts: Chars,
  }

  let doko = |f: char, t: char| {
    let fnum = match f {
      'A' => 0usize,
      'B' => 1,
      'C' => 2,
      'D' => 3,
      'E' => 4,
      _ => unreachable!(),
    };
    let tnum = match t {
      'A' => 0usize,
      'B' => 1,
      'C' => 2,
      'D' => 3,
      'E' => 4,
      _ => unreachable!(),
    };
    fnum.abs_diff(tnum)
  };

  let ds = doko(ss[0], ss[1]);
  let dt = doko(ts[0], ts[1]);

  if ds == dt || 5 - ds == dt {
    println!("Yes");
  } else {
    println!("No");
  }
}
