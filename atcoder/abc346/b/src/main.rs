use proconio::input;

fn main() {
    input! {
        mut w: i32,
        mut b: i32
    }
    println!("{}", if judge(w, b) { "Yes" } else { "No" })
}

fn judge(mut w: i32, mut b: i32) -> bool {
    let a = std::cmp::min(w / 7, b / 5);
    w -= a * 7;
    b -= a * 5;
    let d = w - b;
    return match d {
        -1 => b >= 1 && b <= 3,
        0 => b <= 5,
        1 => true,
        2 => true,
        3 => b >= 2 && b <= 4,
        _ => false,
    };
}

// fn judge(w: i32, b: i32) -> bool {
//     let l = (w + b) as usize;
//     let c = "wbwbwwbwbwbw".repeat(20).chars().collect::<Vec<char>>();
//     for i in 0..c.len() - l {
//         let wc = c[i..i + l].iter().filter(|&x| x == &'w').count();
//         let bc = c[i..i + l].iter().filter(|&x| x == &'b').count();
//         if w as usize == wc && b as usize == bc {
//             return true;
//         }
//     }
//     false
// }
