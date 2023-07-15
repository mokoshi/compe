use proconio::input;

// https://atcoder.jp/contests/abc095/tasks/arc096_a
fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
        x: i32,
        y: i32
    };

    if a + b <= 2 * c {
        println!("{}", a * x + b * y);
        return;
    }
    if x > y {
        let mut answer = 2 * c * y;
        answer += if a <= 2 * c {
            (x - y) * a
        } else {
            (x - y) * 2 * c
        };
        println!("{}", answer);
    } else {
        let mut answer = 2 * c * x;
        answer += if b <= 2 * c {
            (y - x) * b
        } else {
            (y - x) * 2 * c
        };
        println!("{}", answer);
    }
}
