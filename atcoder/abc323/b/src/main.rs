use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n]
    }
    let mut wins = vec![0; n];
    for i in 0..n {
        wins[i] = s[i].iter().filter(|&x| x == &'o').count();
    }

    let mut ii = (1..=n).collect::<Vec<_>>();
    ii.sort_by(|&a, &b| {
        if wins[a - 1] == wins[b - 1] {
            a.cmp(&b)
        } else {
            wins[b - 1].cmp(&wins[a - 1])
        }
    });
    println!(
        "{}",
        ii.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
