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
    n: usize,
  }

  let mut c_a: Vec<(usize, BTreeSet<usize>)> = vec![];

  for _ in 0..n {
    input! {
      c: usize,
      a: [usize; c],
    }

    let set = BTreeSet::from_iter(a.iter().cloned());
    c_a.push((c, set));
  }

  input! {
    x: usize,
  }

  let mut ans: Vec<(usize, usize)> = vec![];

  for (i, (c, a)) in c_a.iter().enumerate() {
    if a.contains(&x) {
      ans.push((i + 1, *c));
    }
  }

  ans.sort_by_key(|x| x.1);

  if ans.is_empty() {
    println!("0");
    return;
  }

  let min = ans[0].1;
  let ans = ans.iter().filter(|x| x.1 == min).map(|x| x.0).collect::<Vec<_>>();
  println!("{}", ans.len());

  for i in ans.iter() {
    print!("{} ", i);
  }
}
