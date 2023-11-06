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
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut visited = vec![true; n];

    for i in 0..m {
        if !graph[a[i] - 1].binary_search(&(b[i] - 1)).is_ok() {
            graph[a[i] - 1].push(b[i] - 1);
        }
        if !graph[b[i] - 1].binary_search(&(a[i] - 1)).is_ok() {
            graph[b[i] - 1].push(a[i] - 1);
        }
        visited[a[i] - 1] = false;
        visited[b[i] - 1] = false;
    }

    println!("{:?}", graph);
    println!("{:?}", visited);
}
