use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        m: i32,
        s: Chars
    }
    let mut mm = m;
    let mut ll = 0;
    let mut ans = 0;
    for &c in &s {
        if c == '0' {
            ans = ans.max(ll);
            mm = m;
            ll = 0;
        } else if c == '1' {
            if mm == 0 {
                ll += 1;
            } else {
                mm -= 1;
            }
        } else {
            ll += 1;
        }
    }
    ans = ans.max(ll);

    println!("{}", ans)
}
