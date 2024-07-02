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
    graph: [[usize;n];n],
  }

  for line in graph {
    for (idx, v) in line.iter().enumerate() {
      if v == &1 {
        print!("{} ", idx + 1);
      }
    }
    println!();
  }
}
