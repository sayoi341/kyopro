#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use libm::sqrt;
use nalgebra::ComplexField;
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
      d: usize,
  }

  let ds = sqrt(d as f64) as usize;
  let mut ans = usize::MAX;

  for x in 0..=ds {
    let y = sqrt((d - x * x) as f64);
    let yy = y as usize;

    ans = min(ans, (x * x + yy * yy).abs_diff(d));
    ans = min(ans, (x * x + (yy + 1) * (yy + 1)).abs_diff(d));
  }

  println!("{}", ans);
}
