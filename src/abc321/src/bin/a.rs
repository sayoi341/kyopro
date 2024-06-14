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
    n:usize
  }

  // nの各桁を取り出すし、1桁ずつの数字に分解する
  let n_vec = n.to_string().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

  if n_vec.len() == 1 {
    println!("Yes");
    return;
  }

  for i in 0..n_vec.len() - 1 {
    if !(n_vec[i] > n_vec[i + 1]) {
      println!("No");
      return;
    }
  }

  println!("Yes");
}
