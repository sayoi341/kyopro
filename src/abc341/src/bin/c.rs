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
    h: usize,
    w: usize,
    _n: usize,
    t: Chars,
    g: [Chars; h]
  }

  let mut ans = 0;

  'sp: for (mut x, mut y) in iproduct!(0..w, 0..h) {
    if g[y][x] == '#' {
      continue 'sp;
    }

    for c in &t {
      match c {
        'U' => {
          y = y.wrapping_sub(1);
        }
        'D' => {
          y = y.wrapping_add(1);
        }
        'L' => {
          x = x.wrapping_sub(1);
        }
        'R' => {
          x = x.wrapping_add(1);
        }
        _ => unreachable!(),
      }

      if x >= w || y >= h || g[y][x] == '#' {
        continue 'sp;
      }
    }

    ans += 1;
  }

  println!("{}", ans);
}
