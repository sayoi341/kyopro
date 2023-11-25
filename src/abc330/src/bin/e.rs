#![allow(unused_imports)]
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
        aa: [usize; n],
        ix: [(Usize1, usize); q],
    }

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, &a) in aa.iter().enumerate() {
        *map.entry(i).or_insert(0) = a;
    }

    for (i, x) in ix {
        *map.entry(i).or_insert(0) = x;
        let max = map.iter().max_by_key(|x| x.1).unwrap().1 + 1;

        's: for i in 0..=max {
            let mut vec = map.values().clone().collect_vec();
            vec.sort();
            if vec.binary_search(&&i).is_err() {
                println!("{}", i);
                break 's;
            }
        }
    }
}
