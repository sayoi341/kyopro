#![allow(unused_imports)]
use im_rc::{hashmap, hashset};
use itertools::{Itertools, MinMaxResult};
use proconio::{fastout, input, marker::*};
use std::{
  cmp::{max, min},
  collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
  io::*,
  iter::{FromIterator, IntoIterator},
  ops::{Add, Div, Mul, Neg, Sub},
  str::FromStr,
  vec,
};

#[fastout]
fn main() {
  input! {
      n: usize,
      q: usize,
      mut aa: [usize; n],
      ix: [(Usize1, usize); q],
  }

  // Aに含まれない数字の集合
  let mut set: BTreeSet<usize> = BTreeSet::from_iter(0..=n);
  // Aに含まれる数字の出現回数(key = 数字, value = 出現回数)
  let mut map: HashMap<usize, usize> = HashMap::new();

  for &a in aa.iter() {
    *map.entry(a).or_insert(0) += 1;
    set.remove(&a);
  }

  for &(i, x) in ix.iter() {
    // A[i]をxに変更するクエリを処理する

    // A[i]の出現回数を減らす
    *map.entry(aa[i]).or_insert(0) -= 1;

    // A[i]がAに含まれなくなったら、Aに含まれない数字の集合に追加する
    if *map.get(&aa[i]).unwrap() == 0 {
      set.insert(aa[i]);
    }

    // xの出現回数を増やす
    *map.entry(x).or_insert(0) += 1;

    // A[i]をxに変更する
    aa[i] = x;

    // xをAに含まれない数字の集合から削除する
    set.remove(&x);

    // 最小値を出力する
    println!("{}", set.iter().next().unwrap());
  }
}
