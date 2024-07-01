#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use multimap::MultiMap;
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
    aarr: [Usize1; n],
    warr: [usize; n],
  }

  let mut boxes = vec![0; n];

  for a in &aarr {
    boxes[*a] += 1;
  }

  let mut map: MultiMap<usize, usize> = MultiMap::new();
  for (a, w) in aarr.iter().zip(warr.iter()) {
    map.insert(*a, *w);
  }

  //println!("{:?}", boxes);

  let mut ans = 0;

  for (i, bi) in boxes.iter().enumerate() {
    if *bi < 2 {
      continue;
    }
    //println!("{}", i);
    let vec = map.remove(&i).unwrap();
    //println!("{:?}", vec);
    // vec.sum - vec.max
    ans += vec.iter().sum::<usize>() - vec.iter().max().unwrap();
  }

  println!("{}", ans);
}
