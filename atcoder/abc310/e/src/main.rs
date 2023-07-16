use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String
    }

    let mut m = vec![];
    for c in s.chars() {
        m.push(c as usize - 48);
    }

    let mut table = vec![vec![0; n]; n];
    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            if i == j {
                table[i][j] = m[i];
            } else {
                table[i][j] = if table[i][j - 1] == 1 && m[j] == 1 {
                    0
                } else {
                    1
                };
            }
            ans += table[i][j];
        }
    }

    println!("{}", ans)
}
