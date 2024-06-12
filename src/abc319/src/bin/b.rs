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
    n: usize
  }

  'outl: for i in 0..=n {
    for j in 1..10 {
      if n % j == 0 && i % (n / j) == 0 {
        print!("{}", j);
        continue 'outl;
      }
    }
    print!("-");
  }
}
