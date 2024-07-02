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
  let mut vec = vec![];

  loop {
    input! { a: usize }
    vec.push(a);
    if a == 0 {
      break;
    }
  }

  for a in vec.iter().rev() {
    println!("{}", a);
  }
}
