use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
        mut a: [i32; n]
    }
    a.sort();

    let mut ans = 0;
    for i in 0..n {
        let to = a.partition_point(|&x| x < a[i] + m);
        ans = ans.max(to - i);
    }
    println!("{}", ans);
}
