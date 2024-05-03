use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        mut a: [u64; n]
    }
    a.sort();

    let mut m = 0;
    let mut ans = (1 + k) * k / 2;
    for i in 0..n {
        if a[i] > k {
            break;
        }
        if a[i] != m {
            ans -= a[i];
        }
        m = a[i];
    }
    println!("{}", ans);
}

/*
3 5
1 2 3

5 5
1 2 3 4 5

 */
