use proconio::input;

fn main() {
    input! {
        q: usize
    }
    let mut v = vec![];
    for _ in 0..q {
        input! {
            t: u8,
            x: i64
        }
        if t == 1 {
            v.push(x);
        } else {
            println!("{}", v[v.len() - x as usize]);
        }
    }
}
