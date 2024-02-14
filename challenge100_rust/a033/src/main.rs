use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut table = vec![vec![-1; h]; w];
    table[0][0] = 0;
    let mut q = VecDeque::from([(0, 0)]);
    while !q.is_empty() {
        let p = q.pop_front().unwrap();
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (p.0 + dx, p.1 + dy);
            if np.0 < 0 || np.0 > w as i32 - 1 || np.1 < 0 || np.1 > h as i32 - 1 {
                continue;
            }
            if s[np.1 as usize][np.0 as usize] == '#' || table[np.0 as usize][np.1 as usize] != -1 {
                continue;
            }
            table[np.0 as usize][np.1 as usize] = table[p.0 as usize][p.1 as usize] + 1;
            q.push_back(np);
        }
    }

    if table[w - 1][h - 1] == -1 {
        println!("-1");
        return;
    }

    let mut p = (w as i32 - 1, h as i32 - 1);
    loop {
        if p.0 == 0 && p.1 == 0 {
            break;
        }
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let np = (p.0 + dx, p.1 + dy);
            if np.0 < 0 || np.0 > w as i32 - 1 || np.1 < 0 || np.1 > h as i32 - 1 {
                continue;
            }
            if table[np.0 as usize][np.1 as usize] == table[p.0 as usize][p.1 as usize] - 1 {
                table[p.0 as usize][p.1 as usize] = -2;
                p = np;
                break;
            }
        }
    }
    table[0][0] = -2;

    let mut ans = 0;
    for i in 0..w {
        for j in 0..h {
            if s[j][i] == '.' && table[i][j] != -2 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
