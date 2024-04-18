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
      s: Chars,
  }

  let mut ans = "".to_string();

  for &c in &s {
    if c == 'C' && ans.len() >= 2 && &ans[ans.len() - 2..ans.len()] == "AB" {
      ans.pop();
      ans.pop();
    } else {
      ans.push(c);
    }
  }

  println!("{}", ans);
}
