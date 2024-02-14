use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(usize, usize); n - 1],
        px: [(usize, i64); q],
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut xtable = vec![0; n + 1];
    for (p, x) in px {
        xtable[p] += x;
    }
    let mut seen = vec![false; n + 1];
    let mut ans = vec![0; n + 1];

    dfs(1, 0, &mut ans, &mut seen, &xtable, &g);

    for i in 1..=n {
        print!("{} ", ans[i]);
    }
    println!();
}

fn dfs(
    p: usize,
    cost: i64,
    ans: &mut Vec<i64>,
    seen: &mut Vec<bool>,
    xtable: &Vec<i64>,
    g: &Vec<Vec<usize>>,
) {
    let ncost = cost + xtable[p];
    ans[p] = ncost;
    seen[p] = true;
    for &np in &g[p] {
        if seen[np] {
            continue;
        }
        dfs(np, ncost, ans, seen, xtable, &g);
    }
}
