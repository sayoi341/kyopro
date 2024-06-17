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
    k: usize,
    g: usize,
    m: usize,
  }

  let mut gc = 0;
  let mut mc = 0;

  for _i in 0..k {
    if gc >= g {
      //  グラスが水で満たされているとき、グラスを捨てる
      gc = 0;
    } else if mc == 0 {
      // そうでなく、マグカップがからの時、マグカップを満たす
      mc = m;
    } else {
      // そうでない時、マグカップがからになるか、グラスが満たされるまでまぐからグラアに水を移す
      let t = min(mc, g - gc);
      mc -= t;
      gc += t;
    }
  }

  println!("{gc} {mc}");
}
