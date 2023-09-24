use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n-1]
    }

    for i in 0..=100 {
        let mut b = a.clone();
        b.push(i as i32);
        b.sort();
        b.remove(n - 1);
        b.remove(0);
        if b.iter().sum::<i32>() >= x {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
