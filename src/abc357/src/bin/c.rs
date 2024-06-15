#![allow(unused_imports)]
use ac_library::*;
use itertools::{iproduct, Itertools, MinMaxResult};
use num_traits::Pow;
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

fn car(k: usize, ans: &mut [Vec<char>], xs: usize, ys: usize) {
  if k == 0 {
    return;
  }

  for (x, y) in iproduct!(0..3, 0..3) {
    if x == 1 && y == 1 {
      for (dx, dy) in iproduct!(0..3usize.pow((k - 1) as u32), 0..3usize.pow((k - 1) as u32)) {
        ans[xs + dx + 3usize.pow((k - 1) as u32) * x][ys + dy + 3usize.pow((k - 1) as u32) * y] = '.';
      }
    } else {
      car(k - 1, ans, xs + x * 3usize.pow((k - 1) as u32), ys + y * 3usize.pow((k - 1) as u32));
    }
  }
}

fn main() {
  input! {
    n: usize,
  }

  let mut ans = vec![vec!['#'; 3usize.pow(n as u32)]; 3usize.pow(n as u32)];

  car(n, &mut ans, 0, 0);

  println!("{}", ans.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
}
