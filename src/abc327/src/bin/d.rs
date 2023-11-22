#![allow(unused_imports)]
use itertools::{Itertools, MinMaxResult};
use ndarray::ShapeBuilder;
use petgraph::graph::DiGraph;
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

fn dfs(graph: &Vec<Vec<usize>>, v: usize, c: isize, colors: &mut Vec<isize>) -> bool {
    // 訪問している頂点を色で塗る
    colors[v] = c;

    for &connected_v in graph[v].iter() {
        if colors[connected_v] == 0 {
            // 隣接している頂点がまだ塗られていない場合
            if !dfs(graph, connected_v, -c, colors) {
                // 隣接している頂点を塗ることができなかった場合
                return false;
            }
        } else if colors[connected_v] != -c {
            // 隣接している頂点が塗られていて、色が同じ場合
            return false;
        }
    }
    true
}

fn is_bipartite(graph: &Vec<Vec<usize>>) -> bool {
    let n = graph.len();
    let mut colors = vec![0; n];

    for v in 0..n {
        if colors[v] != 0 {
            // すでに塗られている場合はスキップ
            continue;
        }
        if !dfs(graph, v, 1, &mut colors) {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        b: [usize; m],
    }

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

    for i in 0..m {
        graph[a[i] - 1].push(b[i] - 1);
        graph[b[i] - 1].push(a[i] - 1);
    }

    for g in graph.iter_mut().take(n) {
        g.sort();
        g.dedup();
    }

    println!("{}", if is_bipartite(&graph) { "Yes" } else { "No" });
}
