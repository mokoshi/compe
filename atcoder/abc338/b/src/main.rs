use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }
    let mut map = vec![0; 26];
    for c in s {
        map[(c as u8 - b'a') as usize] += 1;
    }
    let mut maxi = 0;
    let mut maxv = map[0];
    for i in 0..26 {
        if map[i] > maxv {
            maxv = map[i];
            maxi = i;
        }
    }
    println!("{}", (b'a' + maxi as u8) as char);
}
