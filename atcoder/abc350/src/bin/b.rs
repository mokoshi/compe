use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        t: [usize; q]
    }
    let mut v = vec![true; n];
    for t in t {
        v[t - 1] = !v[t - 1]
    }
    println!("{}", v.iter().filter(|&&x| x).count())
}
