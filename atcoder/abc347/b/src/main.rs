use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: String
    }
    let mut st = HashSet::new();
    for i in 0..s.len() {
        for j in i..s.len() {
            st.insert(&s[i..=j]);
        }
    }
    println!("{}", st.len())
}
