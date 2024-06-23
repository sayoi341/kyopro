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
    n: usize, // 鍵の数
    m: usize, // テスト
    k: usize, // 正しい鍵をk回以上使う
  }

  let mut test = vec![(vec![false; n], false); m];

  for t in &mut test {
    input! {
      c: usize,
      aarr: [Usize1; c],
      r: char,
    }

    if r == 'o' {
      t.1 = true;
    }

    for a in aarr {
      t.0[a] = true;
    }
  }

  let mut ans = 0;

  'keyloop: for keys in 0usize..1 << n {
    for (a, r) in &test {
      let mut cnt = 0;
      for key in 0..n {
        if keys >> key & 1 == 1 && a[key] {
          cnt += 1;
        }
      }
      if *r && cnt < k || !*r && cnt >= k {
        continue 'keyloop;
      }
    }
    ans += 1;
  }

  println!("{}", ans);
}
