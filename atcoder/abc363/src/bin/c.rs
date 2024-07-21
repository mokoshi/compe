use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Chars
    }
    s.sort();

    let mut ans = 0;
    loop {
        let mut has_palindrome = false;
        'loopi: for i in 0..=n - k {
            // iスタートが回分かどうか
            for j in 0..k / 2 {
                if s[i + j] != s[i + k - j - 1] {
                    continue 'loopi;
                }
            }
            // 回分だったので next i へ
            has_palindrome = true;
            break;
        }
        if !has_palindrome {
            ans += 1;
        }
        if !next_permutation(&mut s) {
            break;
        }
    }
    println!("{}", ans);
}

fn next_permutation(v: &mut Vec<char>) -> bool {
    let n = v.len();
    let mut i = 0;
    let mut j = 0;
    for x in 0..n - 1 {
        let x = n - 2 - x;
        if v[x] < v[x + 1] {
            i = x;
            break;
        }
        if x == 0 {
            return false;
        }
    }
    for x in 0..n {
        let x = n - 1 - x;
        if v[x] > v[i] {
            j = x;
            break;
        }
    }
    v.swap(i, j);
    v[i + 1..].reverse();

    return true;
}
