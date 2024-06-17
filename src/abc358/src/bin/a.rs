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
    s: Chars,
    t: Chars,
  }

  if s.iter().collect::<String>() == "AtCoder" && "Land" == t.iter().collect::<String>() {
    println!("Yes");
  } else {
    println!("No");
  }
}
