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
        m: usize,
        aa: [usize; m],
    }

    let mut arr = vec![0usize; n];
    let mut max_index = 0usize;

    for i in 0..m {
        arr[aa[i] - 1] += 1;

        if arr[aa[i] - 1] == arr[max_index] && max_index < aa[i] - 1 {
        } else if arr[aa[i] - 1] >= arr[max_index] {
            max_index = aa[i] - 1;
        }

        println!("{}", max_index + 1);
    }
}
