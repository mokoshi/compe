use proconio::input;

fn main() {
    input! {
        n: i32,
        p: i32,
        q: i32,
        d: [i32; n]
    }

    let mind = *d.iter().min().unwrap();

    println!("{}", p.min(q + mind));
}
