use proconio::{input, marker::Bytes};

fn main() {
    input! {
        s: Bytes,
        t: Bytes
    }
    let df = |cc: Vec<u8>| -> u8 {
        let ds = cc[0].abs_diff(cc[1]);
        if ds >= 3 {
            5 - ds
        } else {
            ds
        }
    };
    println!("{}", if df(s) == df(t) { "Yes" } else { "No" })
}
