#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
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
        n: usize,
        s: Chars,
    }

    if n == 1 {
        println!("No");
        return;
    }

    for i in 0..n - 1 {
        if s[i] == 'a' && s[i + 1] == 'b' {
            println!("Yes");
            return;
        }

        if s[i] == 'b' && s[i + 1] == 'a' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
