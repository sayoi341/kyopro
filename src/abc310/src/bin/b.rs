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
    _m: usize,
  }

  let mut ps: Vec<usize> = Vec::new();
  let mut cs: Vec<usize> = Vec::new();
  let mut fs: Vec<BTreeSet<usize>> = Vec::new();

  for _ in 0..n {
    input! {
      p: usize,
      c: usize,
      f: [Usize1; c],
    }

    let f = BTreeSet::from_iter(f.into_iter());

    ps.push(p);
    cs.push(c);
    fs.push(f);
  }

  let mut ans = false;

  for (i, j) in iproduct!(0..n, 0..n) {
    // p[i] >= p[j]
    // f[i] ⊆ f[j]
    // p[i] > p[j] || |f[i]| < |f[j]|

    ans |= ps[i] >= ps[j] && fs[j].is_superset(&fs[i]) && (ps[i] > ps[j] || cs[i] < cs[j]);
  }

  println!("{}", if ans { "Yes" } else { "No" });
}
