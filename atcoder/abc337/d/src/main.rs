use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize, k: usize,
        s: [Chars; h]
    }

    let mut ans = i32::MAX;
    if k <= w {
        for y in 0..h {
            let mut c = 0;
            let mut len = 0;
            for x in 0..w {
                len += 1;
                if s[y][x] == '.' {
                    c += 1;
                } else if s[y][x] == 'x' {
                    len = 0;
                    c = 0;
                    continue;
                }
                if len > k && s[y][x - k] == '.' {
                    c -= 1;
                }
                if len >= k {
                    ans = ans.min(c);
                }
            }
        }
    }
    if k <= h {
        for x in 0..w {
            let mut c = 0;
            let mut len = 0;
            for y in 0..h {
                len += 1;
                if s[y][x] == '.' {
                    c += 1;
                } else if s[y][x] == 'x' {
                    len = 0;
                    c = 0;
                    continue;
                }
                if len > k && s[y - k][x] == '.' {
                    c -= 1;
                }
                if len >= k {
                    ans = ans.min(c);
                }
            }
        }
    }
    println!("{}", if ans == i32::MAX { -1 } else { ans });
}
