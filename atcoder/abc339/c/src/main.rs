use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }
    let mut min = 0;
    let mut s = 0;
    for i in 0..n {
        s += a[i];
        min = min.min(s);
    }
    println!("{}", s - min)
}

/*
4
5 30 7 -20

 */
