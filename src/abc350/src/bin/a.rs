#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult};
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
};
use superslice::Ext;

#[fastout]
fn main() {
  input! {
    s: Chars,
  }

  if s[0] == 'A' && s[1] == 'B' && s[2] == 'C' {
    // ABC
    // sの3から6を取り出す
    let str = s[3..6].iter().collect::<String>();
    let num = str.parse::<i32>().unwrap();

    if 0 < num && num < 350 && num != 316 {
      println!("Yes");
    } else {
      println!("No");
    }
  } else {
    println!("No");
  }
}
