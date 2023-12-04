use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    }
    let s = s
        .iter()
        .map(|&x| x.to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>();

    let mut cnt = vec![0; 10];
    for i in s {
        cnt[i] += 1;
    }

    let mut ans = 0;
    'root: for i in 0..=3162277_i64 {
        let mut v = i * i;
        let mut vc = vec![0; 10];
        for _ in 0..=(v as f64).log10() as i32 {
            vc[(v % 10) as usize] += 1;
            v /= 10;
        }
        for i in 1..10 {
            if vc[i] != cnt[i] {
                continue 'root;
            }
        }
        if vc[0] <= cnt[0] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
