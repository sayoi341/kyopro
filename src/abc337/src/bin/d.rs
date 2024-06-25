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
  vec,
};
use superslice::Ext;

fn main() {
  input! {
    h: usize,
    w: usize,
    k: usize,
    ss: [Chars; h],
  }

  let mut ans = usize::MAX;

  for i in 0..h {
    let line = &ss[i];
    let mut xsum = vec![0; w + 1];
    let mut dotsum = vec![0; w + 1];
    //println!("{}", line.iter().join(""));
    for j in 0..w {
      xsum[j + 1] = xsum[j] + if line[j] == 'x' { 1 } else { 0 };
      dotsum[j + 1] = dotsum[j] + if line[j] == '.' { 1 } else { 0 };
    }
    //println!("x: {:?}", xsum);
    //println!(".: {:?}", dotsum);
    if w < k {
      continue;
    }
    for j in 0..=w - k {
      //println!("{}: {}:{}", j, xsum[j + k] - xsum[j], dotsum[j + k] - dotsum[j]);
      if xsum[j + k] - xsum[j] == 0 {
        ans = min(ans, dotsum[j + k] - dotsum[j]);
      }
    }
  }

  for i in 0..w {
    let line = &ss.iter().map(|x| x[i]).collect::<Vec<_>>();
    let mut xsum = vec![0; h + 1];
    let mut dotsum = vec![0; h + 1];
    //println!("{}", line.iter().join(""));
    for j in 0..h {
      xsum[j + 1] = xsum[j] + if line[j] == 'x' { 1 } else { 0 };
      dotsum[j + 1] = dotsum[j] + if line[j] == '.' { 1 } else { 0 };
    }
    //println!("x: {:?}", xsum);
    //println!(".: {:?}", dotsum);
    if h < k {
      continue;
    }
    for j in 0..=h - k {
      //println!("{}: {}:{}", j, xsum[j + k] - xsum[j], dotsum[j + k] - dotsum[j]);
      if xsum[j + k] - xsum[j] == 0 {
        ans = min(ans, dotsum[j + k] - dotsum[j]);
      }
    }
  }

  println!("{}", if ans == usize::MAX { -1 } else { ans as isize });
}
