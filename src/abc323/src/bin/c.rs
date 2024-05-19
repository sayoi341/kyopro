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
use superslice::Ext;

fn main() {
  input! {
    n: usize,
    m: usize,
    a: [usize; m],
    s: [Chars; n]
  }

  let mut points = vec![0; n];
  for (i, line) in s.iter().enumerate() {
    for (j, c) in line.iter().enumerate() {
      if *c == 'o' {
        points[i] += a[j];
      }
    }
    points[i] += i + 1;
  }

  let max_index = points.iter().position_max().unwrap();
  let max_point = points.iter().max().unwrap();

  let mut i_a = a.iter().enumerate().map(|(i, &v)| (i, v)).collect::<Vec<_>>();
  // 降順でソート
  i_a.sort_by(|a, b| b.1.cmp(&a.1));

  for i in 0..n {
    if i == max_index {
      println!("0");
      continue;
    }

    let mut my_points = points[i];
    let mut count = 0;

    for (index, point) in &i_a {
      if my_points > *max_point {
        break;
      }

      if s[i][*index] == 'x' {
        my_points += point;
        count += 1;
      }
    }

    println!("{}", count);
  }
}
