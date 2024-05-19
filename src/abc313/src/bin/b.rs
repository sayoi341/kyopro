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
    ab: [(Usize1, Usize1); m]
  }

  let mut set = (0..n).collect::<BTreeSet<_>>();

  for (_, b) in ab {
    set.remove(&b);
  }

  if set.len() == 1 {
    println!("{}", set.iter().next().unwrap() + 1);
  } else {
    println!("-1");
  }
}
