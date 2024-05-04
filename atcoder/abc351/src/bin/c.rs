use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }
    let mut q = VecDeque::new();
    q.push_back(a[0]);
    for i in 1..n {
        let mut c = a[i];
        loop {
            match q.back() {
                Some(last) => {
                    if last == &c {
                        c += 1;
                        q.pop_back();
                        continue;
                    }
                }
                _ => (),
            }
            q.push_back(c);
            break;
        }
    }

    println!("{}", q.len())
}
