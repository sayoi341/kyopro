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
    d: usize,
    p: usize,
    mut fs: [usize; n],
  }

  // 降順でソート
  fs.sort_by(|a, b| b.cmp(a));
  let mut sum = fs.iter().sum::<usize>();
  let mut i = 0;

  // なんかループ
  loop {
    // d個取る
    let dsum = fs.iter().skip(i).take(d).sum::<usize>();
    // 取ったものとpを比較
    if dsum >= p {
      sum -= dsum;
      sum += p;
      i += d;
    } else {
      break;
    }

    if i > n {
      break;
    }
  }

  println!("{}", sum);
}
