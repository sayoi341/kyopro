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
    a: Usize1,
    b: Usize1,
  }

  let mut set: HashSet<usize> = HashSet::from_iter(0..3);

  set.remove(&a);
  set.remove(&b);

  if set.len() == 1 {
    println!("{}", set.iter().next().unwrap() + 1);
  } else {
    println!("-1");
  }
}
