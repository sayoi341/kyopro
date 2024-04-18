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
      da: [usize;n]
  }

  let mut ans = 0usize;

  for i in 1..=n {
    'dayloop: for j in 1..=da[i - 1] {
      let month = i.to_string().chars().collect_vec();
      let day = j.to_string().chars().collect_vec();

      for c_m in month.iter() {
        for c_d in day.iter() {
          if c_m != c_d {
            continue 'dayloop;
          }
        }
      }
      ans += 1;
    }
  }

  // この書き方めっちゃいいわ
  println!("{ans}");
}
