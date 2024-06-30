use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [u64; n]
    }
    let mut ans = vec![0; n + 1];
    ans[0] = 1;
    let mut sa = vec![(0, 0); n];
    let mut end = 0;
    let mut prev = u64::MAX;
    for i in 0..n {
        // println!("--------{}---------", i);
        if h[i] <= prev {
            // println!("shorter wall. i: {}, h[i]: {}", i, h[i]);
            ans[i + 1] = ans[i] + h[i];
            sa[end] = (h[i], i);
            end += 1;
        } else {
            // println!("larger wall. i: {}, h[i]: {}", i, h[i]);
            let p = sa[0..end].partition_point(|&(height, _)| height > h[i]);
            if p == 0 {
                // println!("the wall is largest.");
                ans[i + 1] = h[i] * (i + 1) as u64 + 1;
                sa[0] = (h[i], i);
                end = 1;
            } else {
                let (ph, pi) = sa[p - 1];
                // println!("second larger wall. pi: {}, ph: {}", pi, ph);
                ans[i + 1] = ans[pi + 1] + h[i] * (i - pi) as u64;
                sa[p] = (h[i], i);
                end = p + 1;
            }
        }
        // println!("ans[{}] = {}", i + 1, ans[i + 1]);
        // println!("sa: {:?}", &sa[0..end]);
        prev = h[i];
    }
    println!("{}", ans[1..].iter().join(" "));
}
