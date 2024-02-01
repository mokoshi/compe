use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut board = vec![vec![0; n]; n];
    let mut c = 1;
    for o in 0..(n - 1) / 2 {
        for i in o..n - o {
            board[o][i] = c;
            c += 1;
        }
        for i in o + 1..n - o {
            board[i][n - 1 - o] = c;
            c += 1;
        }
        for i in o + 1..n - o {
            board[n - 1 - o][n - i - 1] = c;
            c += 1;
        }
        for i in o + 1..n - o - 1 {
            board[n - i - 1][o] = c;
            c += 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            if i == j && i == (n - 1) / 2 {
                print!("T ");
            } else {
                print!("{} ", board[i][j]);
            }
        }
        println!("")
    }
}
