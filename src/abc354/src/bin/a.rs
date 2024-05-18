#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use num_traits::Pow;
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    h: isize,
  }

  let mut i = 0;

  loop {
    let h2 = 2isize.pow(i);
    if h < h2 - 1 {
      println!("{}", i);
      return;
    }
    i += 1;
  }
}
