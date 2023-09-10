use proconio::input;

fn main() {
    input! {
        n: i32
    }

    let mut d = vec![];
    for i in 1..10 {
        if n % i == 0 {
            d.push(i);
        }
    }

    let mut ans = String::new();
    'root: for i in 0..=n {
        for j in 0..d.len() {
            if i % (n / d[j]) == 0 {
                ans += &d[j].to_string();
                continue 'root;
            }
        }
        ans += "-";
    }

    println!("{}", ans);
}
