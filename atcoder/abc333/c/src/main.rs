use proconio::input;

fn main() {
    input! { n: usize}

    let mut r = vec![];
    for i in 1..=12 {
        r.push("1".repeat(i).parse::<u64>().unwrap());
    }

    let mut a = vec![];
    for i in 0..12 {
        for j in 0..12 {
            for k in 0..12 {
                a.push(r[i] + r[j] + r[k]);
            }
        }
    }
    a.sort();
    a.dedup();
    println!("{:?}", a[n - 1]);
}
