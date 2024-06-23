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
    l: Usize1,
    r: Usize1,
  }

  let mut a = (1..=n).collect::<Vec<_>>();
  // lからrまでの間を反転
  a[l..=r].reverse();

  println!("{}", a.iter().join(" "));
}
