#![allow(unused_imports)]
use ac_library::Dsu;
use itertools::{Itertools, MinMaxResult};
use nalgebra::coordinates::X;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
};
use superslice::Ext;

fn divisors(n: usize) -> Vec<usize> {
    let mut divisors = Vec::new();
    // n := i * x とおくと、 i が i > root(n) の時、　i はすでに ある x に探索されているから
    // i <= root(n) まで探索すればよい
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        // i で n が割り切れた場合
        if n % i == 0 {
            // 約数リストに格納
            divisors.push(i);

            // n := i * x の x を格納。ただし x := i の時は除く
            if i != n / i {
                divisors.push(n / i);
            }
        }
    }
    divisors.sort();
    divisors
}

#[fastout]
fn main() {
    input! {
        n: usize
    }

    let mut ans = 0usize;

    let mut div_list = vec![0];
    for i in 1..=n - 1 {
        div_list.push(divisors(i).len());
    }

    for x in 1..=n - 1 {
        ans += div_list[x] * div_list[n - x];
    }

    println!("{}", ans);
}
