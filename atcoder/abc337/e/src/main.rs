use proconio::{input_interactive, marker::Chars};

fn main() {
    input_interactive! {
        n: usize
    }
    let m = (usize::BITS - (n - 1).leading_zeros()) as usize;
    println!("{}", m);

    let mut people = vec![vec![]; m];
    for j in 1..=n {
        for p in 0..m {
            if j & 1 << p != 0 {
                people[p].push(j);
            }
        }
    }
    for i in 0..m {
        println!(
            "{} {}",
            people[i].len(),
            people[i]
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    input_interactive! { s: Chars }
    let mut j = 0;
    for i in 0..m {
        if s[i] != '0' {
            j += 1 << i;
        }
    }
    println!("{}", if j == 0 { 1 << m } else { j })
}
