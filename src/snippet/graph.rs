use cargo_snippet::snippet;

#[snippet]
pub fn bfs() {
    let mut q: VecDeque<(isize, isize)> = VecDeque::new();
    let mut visit = vec![vec![false; w]; h];

    q.push_back((s_x, s_y));
    visit[s_y][s_x] = true;

    while let Some((x, y)) = q.pop_back() {
        // 探索する

        // キューに追加する
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {}
}
