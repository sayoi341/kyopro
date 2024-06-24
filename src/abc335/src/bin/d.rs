#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use libm::y0;
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
  }

  let mut ans_g = vec![vec!["T".to_string(); n]; n];

  // 0 右 1 下　2 左 3 上
  let mut range = 0;

  // 0 <= rank <= (n-1)/2
  let mut rank = 0;

  let mut x = 0;
  let mut y = 0;

  for i in 1..n * n {
    ans_g[y][x] = i.to_string();

    match range {
      0 => {
        if x == n - 1 - rank {
          range = 1;
          y += 1;
        } else {
          x += 1;
        }
      }
      1 => {
        if y == n - 1 - rank {
          range = 2;
          x -= 1;
        } else {
          y += 1;
        }
      }
      2 => {
        if x == rank {
          range = 3;
          y -= 1;
        } else {
          x -= 1;
        }
      }
      3 => {
        if y == rank + 1 {
          range = 0;
          x += 1;
          rank += 1;
        } else {
          y -= 1;
        }
      }
      _ => unreachable!(),
    }
  }

  for i in 0..n {
    println!("{}", ans_g[i].join(" "));
  }
}
