#![allow(unused_imports)]
use ac_library::*;
use itertools::{Itertools, MinMaxResult};
use ndarray::arr0;
use procon_lib::MultiSet;
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

#[allow(dead_code)]
mod procon_lib {
  use std::{
    borrow::Borrow,
    collections::{
      btree_map::{self},
      BTreeMap,
    },
    ops::{Bound, RangeBounds},
  };

  #[derive(Debug, PartialEq, Clone)]
  pub struct MultiSet<T> {
    size: usize,
    btree_map: BTreeMap<T, usize>,
  }

  impl<T: Ord> Default for MultiSet<T> {
    fn default() -> Self {
      Self::new()
    }
  }

  impl<T: Ord> From<Vec<T>> for MultiSet<T> {
    fn from(value: Vec<T>) -> Self {
      let size = value.len();

      let mut btree_map = BTreeMap::default();
      for key in value {
        *btree_map.entry(key).or_insert(0) += 1;
      }

      Self { size, btree_map }
    }
  }

  impl<T: Ord> MultiSet<T> {
    pub fn clear(&mut self) {
      self.size = 0;
      self.btree_map.clear();
    }

    pub fn contains<Q>(&self, value: &Q) -> bool
    where
      T: Borrow<Q>,
      Q: Ord + ?Sized,
    {
      self.btree_map.contains_key(value)
    }

    pub fn insert(&mut self, value: T) {
      self.size += 1;
      *self.btree_map.entry(value).or_insert(0) += 1;
    }

    pub fn first(&self) -> Option<&T> {
      if let Some((key, _)) = self.btree_map.iter().next() {
        Some(key)
      } else {
        None
      }
    }

    pub fn last(&self) -> Option<&T> {
      if let Some((key, _)) = self.btree_map.iter().next_back() {
        Some(key)
      } else {
        None
      }
    }

    pub fn is_empty(&self) -> bool {
      self.btree_map.is_empty()
    }

    pub fn len(&self) -> usize {
      self.btree_map.len()
    }

    pub fn size(&self) -> usize {
      self.size
    }

    pub fn marge(&mut self, other: &mut MultiSet<T>)
    where
      T: Clone,
    {
      self.size += other.size;

      for (key, val) in other.btree_map.iter() {
        if let Some(prev) = self.btree_map.get_mut(key) {
          *prev += *val;
        } else {
          self.btree_map.insert(key.clone(), *val);
        }
      }
    }

    pub fn new() -> Self {
      Self {
        size: 0,
        btree_map: BTreeMap::new(),
      }
    }

    pub fn pop_first(&mut self) -> Option<T>
    where
      T: Clone,
    {
      if self.is_empty() {
        None
      } else {
        self.size -= 1;

        let first = self.first().unwrap().clone();
        self.remove(&first);
        Some(first)
      }
    }

    pub fn pop_last(&mut self) -> Option<T>
    where
      T: Clone,
    {
      if self.is_empty() {
        None
      } else {
        self.size -= 1;

        let last = self.last().unwrap().clone();
        self.remove(&last);
        Some(last)
      }
    }

    pub fn remove(&mut self, value: &T) -> bool
    where
      T: Clone,
    {
      self.btree_map.entry(value.clone()).and_modify(|e| *e -= 1);
      if let Some(&cnt) = self.btree_map.get(&value) {
        if cnt == 0 {
          self.btree_map.remove(&value);
        }

        self.size -= 1;
        true
      } else {
        false
      }
    }

    pub fn lower_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
    where
      T: Borrow<Q>,
      Q: Ord,
    {
      match bound {
        Bound::Unbounded => unreachable!(),
        _ => {
          if let Some((key, _)) = self.btree_map.range((bound, Bound::Unbounded)).next() {
            Some(key)
          } else {
            None
          }
        }
      }
    }

    pub fn upper_bound<Q>(&self, bound: Bound<&Q>) -> Option<&T>
    where
      T: Borrow<Q>,
      Q: Ord,
    {
      match bound {
        Bound::Unbounded => unreachable!(),
        _ => {
          if let Some((key, _)) = self.btree_map.range((Bound::Unbounded, bound)).next_back() {
            Some(key)
          } else {
            None
          }
        }
      }
    }

    pub fn iter(&self) -> Iter<'_, T> {
      Iter { range: self.range(..) }
    }

    pub fn range<U: ?Sized, R>(&self, range: R) -> Range<'_, T>
    where
      U: Ord,
      T: Borrow<U> + Ord,
      R: RangeBounds<U>,
    {
      Range {
        last: None,
        counter: 0,
        range: self.btree_map.range(range),
      }
    }

    pub fn count<Q>(&self, value: &Q) -> usize
    where
      T: Borrow<Q>,
      Q: Ord,
    {
      if let Some(&cnt) = self.btree_map.get(value) {
        cnt
      } else {
        0
      }
    }
  }

  #[derive(Debug, Clone, Default)]
  pub struct Range<'a, T>
  where
    T: 'a,
  {
    last: Option<&'a T>,
    counter: usize,
    range: btree_map::Range<'a, T, usize>,
  }

  impl<'a, T> Iterator for Range<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      if self.counter == 0 {
        if let Some((elem, &cnt)) = self.range.next() {
          self.last = Some(elem);
          self.counter = cnt - 1;
          Some(elem)
        } else {
          None
        }
      } else {
        self.counter -= 1;
        self.last
      }
    }
  }

  impl<'a, T> DoubleEndedIterator for Range<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
      if self.counter == 0 {
        if let Some((elem, &cnt)) = self.range.next_back() {
          self.last = Some(elem);
          self.counter = cnt;
          Some(elem)
        } else {
          None
        }
      } else {
        self.counter -= 1;
        self.last
      }
    }
  }

  #[derive(Clone, Debug, Default)]
  pub struct Iter<'a, T>
  where
    T: 'a,
  {
    range: Range<'a, T>,
  }

  impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      self.range.next()
    }
  }

  impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
      self.range.next_back()
    }
  }
}
fn main() {
  input! {
      n: usize,
      m: usize,
      mut aarr: [usize; n],
      mut barr: [usize; m],
  }
  aarr.sort();
  barr.sort();

  let mut ans = 0;

  let mut aset = MultiSet::from(aarr);
  for b in barr {
    if let Some(costi) = aset.lower_bound(Bound::Included(&b)).cloned() {
      ans += costi;
      aset.remove(&costi);
    } else {
      println!("-1");
      return;
    }
  }

  println!("{}", ans);
}
