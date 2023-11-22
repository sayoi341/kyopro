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

fn dfs ()

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    // 探索済みかどうかの配列
    let mut visit = vec![vec![false; w]; h];
    //　キューの作成
    let mut q: VecDeque<(isize, isize)> = VecDeque::new();

    // 開始位置の探索
    for (i, vw) in c.iter().enumerate().take(h) {
        for (j, v) in vw.iter().enumerate().take(w) {
            if *v == 's' {
                q.push_back((j as isize, i as isize));
                visit[j][i] = true;
            }
        }
    }

    while let Some((x, y)) = q.pop_back() {
        // 探索
        for i in x - 1..=x + 1 {
            // ガード
            if i < 0 || w as isize <= i {
                continue;
            }

            if c[y as usize][i as usize] == 'g' {
                println!("Yes");
                return;
            }

            if c[y as usize][i as usize] == '.' && !visit[y as usize][i as usize] {
                q.push_back((i, y));
                visit[y as usize][i as usize] = true;
            }
        }

        for j in y - 1..=y + 1 {
            if j < 0 || h as isize <= j {
                continue;
            }

            if c[j as usize][x as usize] == 'g' {
                println!("Yes");
                return;
            }

            if c[j as usize][x as usize] == '.' && !visit[j as usize][x as usize] {
                q.push_back((x, j));
                visit[j as usize][x as usize] = true;
            }
        }
    }

    println!("No");
}
