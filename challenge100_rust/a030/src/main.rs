use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        c: [Chars; h],
    }
    let mut s = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 'S' {
                s = (i, j);
                break;
            }
        }
    }

    let next = |s: (usize, usize), target: char| -> ((usize, usize), i32) {
        let mut table = vec![vec![-1; w]; h];
        let mut q = VecDeque::from([s]);
        table[s.0][s.1] = 0;
        while !q.is_empty() {
            let p = q.pop_front().unwrap();
            for (dy, dx) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let ny = p.0 as i32 + dy;
                let nx = p.1 as i32 + dx;
                if ny < 0 || ny >= h as i32 || nx < 0 || nx >= w as i32 {
                    continue;
                }
                let y = ny as usize;
                let x = nx as usize;
                if table[y][x] == -1 && c[y][x] != 'X' {
                    table[y][x] = table[p.0][p.1] + 1;
                    if c[y][x] == target {
                        return ((y, x), table[y][x]);
                    }
                    q.push_back((y, x));
                }
            }
        }
        panic!()
    };

    let mut ans = 0;
    let mut ns = s;
    for i in 1..=n {
        let (p, d) = next(ns, i.to_string().chars().nth(0).unwrap());
        ns = p;
        ans += d;
    }
    println!("{}", ans);
}
