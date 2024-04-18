#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use num_traits::Pow;
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
      mut s: Chars,
  }

  s.sort();

  let mut ans = 0usize;

  for p in (0..).take_while(|&x| x * x <= 10_u64.pow(n as u32)) {
    let mut s_s = (p * p).to_string().chars().collect_vec();

    s_s.resize(n, '0');
    s_s.sort();

    if s_s == s {
      ans += 1;
    }
  }

  println!("{}", ans);
}
