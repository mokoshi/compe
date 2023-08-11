use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let avg: i64 = a.iter().sum::<i64>() / n as i64;
    let mut remain = a.iter().sum::<i64>() - avg * n as i64;
    a.sort();

    let mut answer = 0;
    for j in 0..n {
        let i = n - j - 1;
        if a[i] <= avg {
            break;
        }
        if remain > 0 {
            answer += a[i] - avg - 1;
            remain -= 1;
        } else {
            answer += a[i] - avg;
        }
    }

    println!("{}", answer)
}
