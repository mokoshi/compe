use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n]
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        let a = s[i].clone();
        let b = s[i].clone().chars().rev().collect::<String>();
        if set.contains(&a) || set.contains(&b) {
            continue;
        }
        set.insert(a);
    }

    println!("{}", set.len());
}
