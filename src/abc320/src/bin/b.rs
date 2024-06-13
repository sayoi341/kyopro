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

fn is_panlindrome(s: &str) -> bool {
  s.chars().eq(s.chars().rev())
}

fn main() {
  input! {
    s: Chars
  }

  let n = s.len();

  let mut ans = 0;

  for left in 0..n {
    for right in left + 1..n + 1 {
      // n以下の2つの数字を選ぶ
      // それが部分文字列

      if is_panlindrome(&s[left..right].iter().collect::<String>()) {
        ans = max(ans, right - left);
      }
    }
  }

  println!("{}", ans);
}
