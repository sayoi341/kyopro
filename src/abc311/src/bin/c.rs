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

fn main() {
  input! {
    n: usize,
    g: [Usize1; n],
  }

  let mut visited = vec![false; n];
  let mut looped = vec![];
  let mut queue = VecDeque::new();
  queue.push_back(0);

  let mut v = 0; //start
  while !visited[v] {
    visited[v] = true;
    looped.push(v);
    v = g[v];
  }

  let loop_start = looped.iter().position(|&x| x == v).unwrap();
  println!("{}", looped.len() - loop_start);
  println!("{}", looped[loop_start..].iter().map(|&x| x + 1).join(" "));
}
