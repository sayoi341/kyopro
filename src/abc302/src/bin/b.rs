#![allow(unused_imports)]
use ac_library::*;
use itertools::*;
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};

fn main() {
  input! {
    h: usize,
    w: usize,
    s: [Chars; h],
  }

  for (x, y) in iproduct!(0..w, 0..h) {
    if s[y][x] != 's' {
      continue;
    }
    'out: for (dx, dy) in iproduct!([!0, 0, 1], [!0, 0, 1]) {
      let (mut tmpx, mut tmpy) = (x, y);
      let mut ans = vec![(x, y)];
      for c in "nuke".chars().collect::<Vec<char>>() {
        tmpx = tmpx.wrapping_add(dx);
        tmpy = tmpy.wrapping_add(dy);
        if tmpx >= w || tmpy >= h || s[tmpy][tmpx] != c {
          continue 'out;
        }
        ans.push((tmpx, tmpy));
      }
      for (x, y) in ans {
        println!("{} {}", y + 1, x + 1);
      }
      return;
    }
  }
}
