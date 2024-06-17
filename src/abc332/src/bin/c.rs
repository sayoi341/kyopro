#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  default,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

fn main() {
  input! {
    _n: usize,
    m: isize,
    s: Chars,
  }

  let mut ans = 0;

  // 1 muji or logo
  // 2 logo
  let mut logo = 0isize;
  let mut muji = m;

  for c in s {
    match c {
      '0' => {
        ans = max(ans, (logo).abs() as usize);
        logo = 0;
        muji = m;
      }
      '1' => {
        if muji == 0 {
          logo -= 1;
        } else {
          muji -= 1;
        }
      }
      '2' => {
        logo -= 1;
      }
      _ => {}
    }
  }

  ans = max(ans, (logo).abs() as usize);
  println!("{ans}");
}
