use proconio::input;

fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1]
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u - 1].push(v - 1);
        g[v - 1].push(u - 1);
    }

    let mut max = 0;
    let mut cost = 1;
    let mut visited = vec![false; n];
    visited[0] = true;
    for &v in &g[0] {
        let c = dfs(v, &g, &mut visited);
        cost += c;
        max = max.max(c);
    }
    println!("{}", cost - max);
}

fn dfs(u: usize, g: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> u64 {
    if visited[u] {
        return 0;
    }
    visited[u] = true;

    let mut tc = 1;
    for &v in &g[u] {
        tc += dfs(v, g, visited);
    }
    return tc;
}
