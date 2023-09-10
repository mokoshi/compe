use proconio::input;

fn main() {
    input! {
        n: usize,
        ss: String,
        q: usize,
        txc: [(usize, usize, char); q],
    }
    let mut s: Vec<char> = ss.chars().collect();

    let mut lasti = -1;
    let mut lastt = 0;
    for i in 0..q {
        if txc[i].0 != 1 {
            lasti = i as i32;
            lastt = txc[i].0;
        }
    }

    for i in 0..q {
        let (t, x, c) = txc[i];
        if t == 1 {
            s[x - 1] = c;
        } else if i as i32 == lasti {
            for j in 0..n {
                s[j] = if lastt == 2 {
                    s[j].to_lowercase().collect::<Vec<char>>()[0]
                } else {
                    s[j].to_uppercase().collect::<Vec<char>>()[0]
                }
            }
        }
    }
    println!("{}", s.iter().collect::<String>());
}
