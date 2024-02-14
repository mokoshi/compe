use proconio::input;
use std::f64::consts::E;

fn main() {
    input! { p: f64 };

    // 微分して2分探索で極地を見つける場合
    if false {
        // f(x) = x + p / 2^(x/1.5) = x + p * 0.5^(x/1.5) = x + p * (0.5^1/1.5)^x
        // f'(x) = 1 + p * log(0.5^1/1.5) * (0.5^1/1.5)^x
        let df = |x| -> f64 {
            return 1.0 + p * 0.5_f64.powf(1.0 / 1.5).log(E) * 0.5_f64.powf(x / 1.5);
        };

        let mut left = 0.0;
        let mut right = p;
        loop {
            let d = (right - left) / 2.0;
            if d < 10_f64.powi(-8) {
                break;
            }
            let mid = left + d;
            if df(mid) < 0.0 {
                left = mid;
            } else {
                right = mid;
            }
        }
        println!("{}", left + p / 2.0_f64.powf(left / 1.5));
    }

    // 3分探索で極地を見つける場合
    if true {
        let f = |x: f64| -> f64 {
            return x + p * 0.5_f64.powf(x / 1.5);
        };

        let mut left = 0.0;
        let mut right = p;
        loop {
            let d = (right - left) / 3.0;
            if d < 10_f64.powi(-8) {
                break;
            }
            let l = left + d;
            let r = right - d;
            if f(l) > f(r) {
                left = l;
            } else {
                right = r;
            }
        }
        println!("{}", f(left));
    }
}
