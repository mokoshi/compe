use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i64,
        l: [i64; n]
    }

    let calc_lines = |w: i64| -> i64 {
        let mut lines = 1;
        let mut c = l[0];
        for i in 1..n {
            let nextc = c + l[i] + 1;
            if nextc > w {
                c = l[i];
                lines += 1;
            } else {
                c = nextc;
            }
        }
        return lines;
    };

    let maxl = *l.iter().max().unwrap();
    let suml = l.iter().sum::<i64>() + (n as i64) - 1;

    let mut left = maxl;
    let mut right = suml;
    while left < right {
        let mid = (left + right) / 2;
        let lines = calc_lines(mid);
        if lines > m {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
