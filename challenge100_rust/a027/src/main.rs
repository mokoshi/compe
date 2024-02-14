use proconio::input;

fn main() {
    input! {
        m: usize,
        n: usize,
        t: [[usize; m]; n]
    }

    let mut ans = 0;
    for y in 0..n {
        for x in 0..m {
            if t[y][x] == 0 {
                continue;
            }
            let mut visited = vec![vec![false; m]; n];
            ans = ans.max(dfs((x as i32, y as i32), &t, &mut visited, n, m));
        }
    }
    println!("{}", ans);
}

fn dfs(
    p: (i32, i32),
    t: &Vec<Vec<usize>>,
    visited: &mut Vec<Vec<bool>>,
    n: usize,
    m: usize,
) -> i32 {
    let mut md = 1;
    visited[p.1 as usize][p.0 as usize] = true;
    for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
        let np = (p.0 + dx, p.1 + dy);
        if np.0 < 0 || np.0 >= m as i32 || np.1 < 0 || np.1 >= n as i32 {
            continue;
        }
        if t[np.1 as usize][np.0 as usize] == 0 {
            continue;
        }
        if visited[np.1 as usize][np.0 as usize] {
            continue;
        }
        md = md.max(dfs(np, t, visited, n, m) + 1);
    }
    visited[p.1 as usize][p.0 as usize] = false;
    md
}
