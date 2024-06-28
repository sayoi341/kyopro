#![allow(unused_imports)]
use ac_library::*;
use itertools::{iproduct, Itertools, MinMaxResult};
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
    s: Chars
  }

  let mut ans = 0;
  for length in 0..s.len() {
    let mut set: BTreeSet<String> = BTreeSet::new();
    for start in 0..s.len() - length {
      if !set.contains(&s[start..start + length + 1].iter().collect::<String>()) {
        ans += 1;
        set.insert(s[start..start + length + 1].iter().collect::<String>());
      }
    }
  }

  println!("{}", ans);
}
