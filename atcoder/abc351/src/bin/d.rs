use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    }
    let mut b = vec![vec![0; w]; h];
    let mut ans = 1;
    for i in 0..h {
        for j in 0..w {
            if b[i][j] > 0 || s[i][j] == '#' {
                continue;
            }
            let c = i * w + j + 1;
            let mut q = VecDeque::from([(j, i)]);
            let mut tmp = 0;
            while !q.is_empty() {
                let (x, y) = q.pop_front().unwrap();
                if b[y][x] == c {
                    continue;
                }
                tmp += 1;
                b[y][x] = c;
                if can_move((x, y), &s, h, w) {
                    if x > 0 {
                        q.push_back((x - 1, y));
                    }
                    if x < w - 1 {
                        q.push_back((x + 1, y));
                    }
                    if y > 0 {
                        q.push_back((x, y - 1));
                    }
                    if y < h - 1 {
                        q.push_back((x, y + 1));
                    }
                }
            }
            ans = ans.max(tmp);
        }
    }
    println!("{}", ans);
}

fn can_move((x, y): (usize, usize), s: &Vec<Vec<char>>, h: usize, w: usize) -> bool {
    (x == 0 || s[y][x - 1] != '#')
        && (x == w - 1 || s[y][x + 1] != '#')
        && (y == 0 || s[y - 1][x] != '#')
        && (y == h - 1 || s[y + 1][x] != '#')
}
