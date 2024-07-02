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
    s: Chars,
    q: usize,
    cdarr: [(char, char); q],
  }

  let mut alphabet = "abcdefghijklmnopqrstuvwxyz".chars().collect_vec();

  for (c, d) in cdarr {
    // map[c] = d;
    for ab in alphabet.iter_mut() {
      if *ab == c {
        *ab = d;
      }
    }
  }

  //println!("{}", alphabet.iter().collect::<String>());

  for c in s {
    print!("{}", alphabet[c as usize - 'a' as usize]);
  }
}
