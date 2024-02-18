use proconio::{input, marker::Chars};
use std::collections::HashMap;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        s: [Chars; h]
    }
    let mv = HashMap::from([('L', (-1, 0)), ('R', (1, 0)), ('U', (0, -1)), ('D', (0, 1))]);
    let mut ans = 0;
    for x in 1..w - 1 {
        'loopy: for y in 1..h - 1 {
            let mut p = (x, y);
            for i in 0..n {
                if s[p.1][p.0] == '#' {
                    continue 'loopy;
                }
                p.0 = (p.0 as i32 + mv[&t[i]].0) as usize;
                p.1 = (p.1 as i32 + mv[&t[i]].1) as usize;
            }
            if s[p.1][p.0] == '.' {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
