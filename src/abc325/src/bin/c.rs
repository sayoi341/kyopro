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
        h: isize,
        w: isize,
        s: [Chars; h],
    }

    let mut visit = vec![vec![false; w as usize]; h as usize];

    let mut ans = 0usize;

    for x in 0..w {
        for y in 0..h {
            if visit[y as usize][x as usize] || s[y as usize][x as usize] == '.' {
                continue;
            }

            ans += 1;

            let mut q: VecDeque<(isize, isize)> = VecDeque::new();
            q.push_back((x, y));

            while let Some(v) = q.pop_back() {
                // 周りの要素を探索
                let (x, y) = v;
                for tx in x - 1..=x + 1 {
                    for ty in y - 1..=y + 1 {
                        if tx >= w || ty >= h || tx < 0 || ty < 0 {
                            continue;
                        }
                        if visit[ty as usize][tx as usize] || s[ty as usize][tx as usize] == '.' {
                            continue;
                        }
                        visit[ty as usize][tx as usize] = true;
                        q.push_back((tx, ty));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
