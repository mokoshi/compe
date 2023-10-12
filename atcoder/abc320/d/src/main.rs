use proconio::input;
use std::collections::VecDeque;

const INF: i64 = std::i64::MAX;

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(usize, usize, i64, i64); m]
    }
    let mut g = vec![vec![]; n + 1];
    for (a, b, x, y) in abxy {
        g[a].push((b, x, y));
        g[b].push((a, -x, -y));
    }

    let mut q = VecDeque::new();
    let mut visited = vec![(INF, INF); n + 1];
    q.push_back((1_usize, 0, 0));
    while !q.is_empty() {
        let (p, x, y) = q.pop_front().unwrap();
        visited[p] = (x, y);
        for &(e, dx, dy) in &g[p] {
            if visited[e].0 != INF {
                continue;
            }
            q.push_back((e, x + dx, y + dy));
        }
    }
    for i in 1..=n {
        if visited[i].0 == INF {
            println!("undecidable");
        } else {
            println!("{} {}", visited[i].0, visited[i].1);
        }
    }
}
