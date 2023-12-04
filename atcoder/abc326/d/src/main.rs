use itertools::{self, Itertools};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        r: Chars,
        c: Chars
    }
    for ap in (0..n).permutations(n) {
        for bp in (0..n).permutations(n) {
            if !(0..n).all(|i| ap[i] != bp[i]) {
                continue;
            }
            'cloop: for cp in (0..n).permutations(n) {
                if !(0..n).all(|i| ap[i] != cp[i] && bp[i] != cp[i]) {
                    continue;
                }

                let mut board = vec![vec!['.'; n]; n];
                for i in 0..n {
                    board[i][ap[i]] = 'A';
                    board[i][bp[i]] = 'B';
                    board[i][cp[i]] = 'C';
                }

                for i in 0..n {
                    for j in 0..n {
                        if board[i][j] != '.' {
                            if board[i][j] != r[i] {
                                continue 'cloop;
                            } else {
                                break;
                            }
                        }
                    }
                }
                for i in 0..n {
                    for j in 0..n {
                        if board[j][i] != '.' {
                            if board[j][i] != c[i] {
                                continue 'cloop;
                            } else {
                                break;
                            }
                        }
                    }
                }

                println!("Yes");
                for i in 0..n {
                    println!("{}", board[i].iter().join(""));
                }
                return;
            }
        }
    }

    println!("No");
}
