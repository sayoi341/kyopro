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

fn main() {
  input! {
    n: usize,
    m: usize,
    s: [Chars; n],
  }

  let mut ans = vec![];

  for (dx_i, dy_j) in iproduct!(0..=m - 9, 0..=n - 9) {
    let mut is_ok = true;
    for (x, y) in iproduct!(0..9, 0..9) {
      if (x < 3 && y < 3) && s[y + dy_j][x + dx_i] != '#' {
        is_ok = false;
      }
      if (x < 4 && y < 4) && (x == 3 || y == 3) && s[y + dy_j][x + dx_i] != '.' {
        is_ok = false;
      }

      if (5 < x && 5 < y) && s[y + dy_j][x + dx_i] != '#' {
        is_ok = false;
      }
      if (4 < x && 4 < y) && (x == 5 || y == 5) && s[y + dy_j][x + dx_i] != '.' {
        is_ok = false;
      }
    }
    if is_ok {
      ans.push((dy_j, dx_i));
    }
  }

  ans.sort();
  for (dy, dx) in ans {
    println!("{} {}", dy + 1, dx + 1);
  }
}
