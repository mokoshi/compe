use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize, usize, u64); m]
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a - 1].push((b - 1, c));
        g[b - 1].push((a - 1, c));
    }
    let mut ans = 0;
    for i in 0..n {
        let mut visited = vec![false; n];
        ans = ans.max(dfs(i, &g, &mut visited));
    }
    println!("{}", ans);
}

fn dfs(s: usize, g: &Vec<Vec<(usize, u64)>>, visited: &mut Vec<bool>) -> u64 {
    let mut m = 0;
    visited[s] = true;
    for e in &g[s] {
        if visited[e.0] {
            continue;
        }
        m = m.max(dfs(e.0, g, visited) + e.1);
    }
    visited[s] = false;
    m
}
