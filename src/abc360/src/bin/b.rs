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

  for w in 0..s.len() {
    for c in 0..w {
      let mut vec = vec![];
      let mut i = c;
      loop {
        vec.push(s[i]);
        i += w;
        if i >= s.len() {
          break;
        }
      }
      if vec == t {
        println!("Yes");
        return;
      }
    }
  }

  println!("No");
}
