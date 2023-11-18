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
        _n: usize,
        s: Chars,
    }

    let mut arr: [usize; 26] = [0; 26];
    let mut before: char = ' ';
    let mut count: usize = 0;

    for c in s {
        if before == c {
            count += 1;
        } else {
            if before != ' ' {
                arr[(before as u8 - b'a') as usize] =
                    max(arr[(before as u8 - b'a') as usize], count);
            }
            count = 1;
            before = c;
        }
    }

    arr[(before as u8 - b'a') as usize] = max(arr[(before as u8 - b'a') as usize], count);

    println!("{}", arr.iter().sum::<usize>());
}
