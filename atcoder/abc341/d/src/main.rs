use num::integer::lcm;
use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        k: u64
    }
    let l = lcm(n, m);
    let nc = l / n;
    let mc = l / m;
    let kc = (k - 1) / (nc + mc - 2);
    let kr = k - kc * (nc + mc - 2);

    let mut left = 0;
    let mut right = l;
    while left < right {
        let mid = (left + right) / 2;
        let nth = mid / n + mid / m;
        if nth < kr {
            left = mid + 1
        } else {
            right = mid;
        }
    }
    println!("{}", kc * l + left);
}
