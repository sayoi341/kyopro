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
    x: isize,
    a: [isize; n -1]
  }

  for i in 0..=100 {
    let mut t_a = a.clone();
    t_a.push(i);
    t_a.sort();
    let sum = t_a.iter().skip(1).take(n - 2).sum::<isize>();

    if sum >= x {
      println!("{}", i);
      return;
    }
  }

  println!("-1");
}
