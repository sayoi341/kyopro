#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use proconio::{input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
  vec,
};

fn main() {
  input! {
      n: usize,
      t: Chars,
      sa: [Chars; n]
  }

  let mut ans: Vec<usize> = Vec::new();

  for (i, s) in sa.iter().enumerate().take(n) {
    if t.len().abs_diff(s.len()) >= 2 {
      continue;
    }

    let mut pre = 0usize;
    let mut suf = 0usize;

    for l in 0..min(t.len(), s.len()) {
      if t[l] == s[l] {
        pre += 1;
      } else {
        break;
      }
    }

    for r in 0..min(t.len(), s.len()) {
      if t[t.len() - r - 1] == s[s.len() - r - 1] {
        suf += 1;
      } else {
        break;
      }
    }

    if (pre + suf) >= min(t.len(), s.len()) || (t.len() == s.len() && (pre + suf + 1) == s.len()) {
      ans.push(i);
    }
  }

  println!("{}", ans.len());
  println!("{}", ans.into_iter().map(|x| x + 1).join(" "));
}
