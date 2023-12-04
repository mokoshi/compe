use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a:[usize; m],
        b:[usize; m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[a[i] - 1].push(b[i] - 1);
        g[b[i] - 1].push(a[i] - 1);
    }

    let mut q = VecDeque::new();
    let mut visited = vec![-1; n];
    for k in 0..n {
        if visited[k] != -1 || g[k].len() == 0 {
            continue;
        }
        visited[k] = 0;
        for &i in &g[k] {
            q.push_back((k, i));
        }
        while !q.is_empty() {
            let (f, t) = q.pop_front().unwrap();
            if visited[t] == -1 {
                visited[t] = 1 - visited[f];
                for &i in &g[t] {
                    q.push_back((t, i));
                }
            } else if visited[f] == visited[t] {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
