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
      mut x: usize,
  }

  loop {
    let hnd = x / 100;
    let ten = (x - hnd * 100) / 10;
    let one = x - hnd * 100 - ten * 10;

    if hnd * ten == one {
      println!("{}", x);
      return;
    }

    x += 1;
  }
}
