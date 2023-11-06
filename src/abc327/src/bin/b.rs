#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use num_traits::Pow;
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
        b: i64,
    }

    for i in 1..10i64.pow(18) {
        if b == i.pow(i as u32) {
            println!("{}", i);
            return;
        }

        if b <= i.pow(i as u32) {
            break;
        }
    }

    println!("-1");
}
