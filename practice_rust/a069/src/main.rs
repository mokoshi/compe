use proconio::input;

fn main() {
    input! {
        q: usize,
    }
    let mut t = vec![0; 100000 / 2];
    for i in 1..100000 / 2 {
        let j = i * 2 + 1;
        t[i] = t[i - 1]
            + if is_prime(j) && is_prime((j + 1) / 2) {
                1
            } else {
                0
            };
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize
        }
        println!(
            "{}",
            t[(r - 1) / 2] - if l == 1 { 0 } else { t[(l - 2) / 2] }
        );
    }
}

fn is_prime(n: usize) -> bool {
    if n <= 1 {
        return false;
    }
    let m = (n as f64).sqrt() as i64;
    for i in 2..=m {
        if n as i64 % i == 0 {
            return false;
        }
    }
    return true;
}
