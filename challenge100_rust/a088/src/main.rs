use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u8; n]
    }

    let mut board = vec![(a[0], 1)];
    for i in 1..n {
        let c = a[i];
        let li = board.len() - 1;
        if i % 2 == 0 {
            if board[li].0 == c {
                board[li].1 += 1;
            } else {
                board.push((c, 1));
            }
        } else {
            if board[li].0 == c {
                board[li].1 += 1;
            } else {
                if board.len() == 1 {
                    board[li].0 = c;
                    board[li].1 += 1;
                } else {
                    let m = board[li].1;
                    board.pop();
                    board[li - 1].1 += m + 1;
                }
            }
        }
    }

    println!(
        "{}",
        board
            .iter()
            .map(|&x| if x.0 == 0 { x.1 } else { 0 })
            .sum::<i32>()
    );
}
