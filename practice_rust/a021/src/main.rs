use proconio::input;

fn main() {
    input! {
        n: usize,
        hs: [(i64, i64); n]
    }

    let avail = |score: i64| -> bool {
        let mut maxp = vec![0; n];
        for i in 0..n {
            if hs[i].0 > score {
                return false;
            }
            maxp[i] = (score - hs[i].0) / hs[i].1;
        }

        maxp.sort();
        for i in 0..n {
            if maxp[i] < i as i64 {
                return false;
            }
        }

        true
    };

    let mut left: i64 = 0;
    let mut right: i64 = 10_i64.pow(16);
    while left < right {
        let mid = (left + right) / 2;
        if avail(mid) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    println!("{}", left);
}
