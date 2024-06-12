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
    s: Chars,
  }

  let map = BTreeMap::from_iter(vec![
    ("tourist", 3858),
    ("ksun48", 3679),
    ("Benq", 3658),
    ("Um_nik", 3648),
    ("apiad", 3638),
    ("Stonefeang", 3630),
    ("ecnerwala", 3613),
    ("mnbvmar", 3555),
    ("newbiedmy", 3516),
    ("semiexp", 3481),
  ]);

  let string = s.iter().collect::<String>();

  println!("{}", map.get(string.as_str()).unwrap());
}
