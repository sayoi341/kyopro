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
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    aarr: [usize; n],
    m: usize,
    barr: [usize; m],
    l: usize,
    carr: [usize; l],
    q: usize,
    xarr: [usize; q],
  }

  let mut set = BTreeSet::new();
  for (a, b, c) in iproduct!(aarr.iter(), barr.iter(), carr.iter()) {
    set.insert(a + b + c);
  }

  for x in xarr {
    if set.contains(&x) {
      println!("Yes");
    } else {
      println!("No");
    }
  }
}
