use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }

    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for x in 0..w {
        for y in 0..h {
            if s[y][x] == '.' || visited[y][x] {
                continue;
            }

            ans += 1;

            let mut q = VecDeque::new();
            q.push_back((x, y));
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                        if nx < 0 || nx > w as i32 - 1 || ny < 0 || ny > h as i32 - 1 {
                            continue;
                        }
                        let (nx, ny) = (nx as usize, ny as usize);
                        if s[ny][nx] == '.' || visited[ny][nx] {
                            continue;
                        }
                        q.push_back((nx, ny));
                        visited[ny][nx] = true;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
