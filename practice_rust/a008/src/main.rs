use proconio::input;

// https://atcoder.jp/contests/s8pc-6/tasks/s8pc_6_b
fn main() {
    input! {
        n: usize,
        ab: [(i64, i64); n]
    };

    let mut a = ab.iter().map(|x| x.0).collect::<Vec<i64>>();
    let mut b = ab.iter().map(|x| x.1).collect::<Vec<i64>>();
    a.sort();
    b.sort();

    let i = a[n / 2];
    let e = b[n / 2];

    let mut answer: i64 = 0;
    for (a, b) in &ab {
        answer += (a - i).abs() + (b - e).abs() + (a - b).abs();
    }

    println!("{}", answer);
}
