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
};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    // 探索済みかどうかの配列
    let mut visit = vec![vec![false; w]; h];
    // 深さ優先探索
    // キューの作成
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    // 開始位置の探索
    for y_i in 0..h {
        for x_i in 0..w {
            if c[y_i][x_i] == 's' {
                visit[y_i][x_i] = true;
                queue.push_back((x_i, y_i));
            }
        }
    }

    while let Some((x, y)) = queue.pop_back() {
        // ゴールに到達したら終了
        if c[y][x] == 'g' {
            println!("Yes");
            return;
        }

        // 4方向の探索
        // !0はビット反転で-1になる
        for (dx, dy) in &[(1, 0), (0, 1), (!0, 0), (0, !0)] {
            // warp_addでオーバーフローしてもpanicしない
            let (nx, ny) = (x.wrapping_add(*dx), y.wrapping_add(*dy));

            // 範囲内かつ壁でなく、探索済みでなければ探索
            if nx < w && ny < h && !visit[ny][nx] && c[ny][nx] != '#' {
                visit[ny][nx] = true;
                queue.push_back((nx, ny));
            }
        }
    }

    println!("No");
}
