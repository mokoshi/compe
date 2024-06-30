use proconio::input;

fn main() {
    input! {
        s: (i128, i128),
        t: (i128, i128)
    }
    let dy = (t.1 - s.1).abs();
    let dx = (if s.0 > t.0 {
        t.0 - if s.1 % 2 == 0 {
            (s.0 / 2) * 2
        } else {
            ((s.0 - 1) / 2) * 2 + 1
        }
    } else if s.0 < t.0 {
        t.0 - if s.1 % 2 == 0 {
            (s.0 / 2) * 2 + 1
        } else {
            ((s.0 + 1) / 2) * 2
        }
    } else {
        0
    })
    .abs();

    if dx < dy {
        println!("{}", dy);
    } else {
        println!("{}", dy + (dx - dy + 1) / 2);
    }
}
