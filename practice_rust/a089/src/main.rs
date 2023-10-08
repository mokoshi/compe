use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n]
    }
    let mut board = vec![(a[0], 1, 0)];
    for i in 1..n {
        let last = board.len() - 1;
        if a[i] != board[last].0 {
            if board[last].2 > 0 {
                board.push((a[i], 2, 0));
            } else {
                board[last].0 = a[i];
                board[last].1 += 1;
            }
        } else {
            board[last].2 += 1;
        }
    }
    if board[board.len() - 1].2 > 0 {
        board.push((1, 1, 0));
    }

    let m = board.len();
    let mut ans = 0;
    for i in 0..m {
        ans = ans.max(board[i].1);
        if i + 1 < m && board[i].2 == 2 {
            ans = ans.max(board[i].1 + board[i + 1].1 + 1);
        }
        if i + 1 < m && board[i].2 == 1 {
            ans = ans.max(board[i].1 + board[i + 1].1);
        }
        if i + 1 < m && board[i].2 > 2 {
            ans = ans.max(board[i].1 + 2).max(board[i + 1].1 + 2);
        }
        if i + 2 < m && board[i].2 == 1 && board[i + 1].2 == 1 {
            ans = ans.max(board[i].1 + board[i + 1].1 + board[i + 2].1);
        }
    }
    println!("{}", ans);
}
