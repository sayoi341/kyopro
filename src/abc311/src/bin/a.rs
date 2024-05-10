#![allow(unused_imports)]
use ac_library::*;
use itertools::{iproduct, Itertools};
use proconio::{marker::*, *};
use std::{
  cmp::{max, min},
  collections::*,
  str::FromStr,
};

fn main() {
  input! {
    _n: usize,
    s: Chars
  }

  let mut a_frag = false;
  let mut b_frag = false;
  let mut c_frag = false;

  for (i, &c) in s.iter().enumerate() {
    match c {
      'A' => a_frag = true,
      'B' => b_frag = true,
      'C' => c_frag = true,
      _ => unreachable!(),
    }

    if a_frag && b_frag && c_frag {
      println!("{}", i + 1);
      return;
    }
  }
}
