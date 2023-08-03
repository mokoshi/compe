use proconio::input;

// https://atcoder.jp/contests/s8pc-4/tasks/s8pc_4_b
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }

    let mut answer = std::i64::MAX;
    for i in 0..1 << n {
        let mut highest = 0;
        let mut cost = 0;
        let mut colors = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                highest = a[j].max(highest + 1);
                cost += highest - a[j];
                colors += 1;
            } else {
                if highest < a[j] {
                    colors += 1;
                }
                highest = a[j].max(highest);
            }
        }

        if colors >= k {
            answer = answer.min(cost);
        }
    }

    println!("{}", answer);
}
