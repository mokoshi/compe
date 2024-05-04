use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut ox = vec![];
    let mut oy = vec![];
    let mut ex = vec![];
    let mut ey = vec![];
    for i in 0..n {
        let (x, y) = (xy[i].0 + xy[i].1, xy[i].0 - xy[i].1);
        if x % 2 == 0 && y % 2 == 0 {
            ex.push(x);
            ey.push(y);
        } else {
            ox.push(x);
            oy.push(y);
        }
    }
    ex.sort();
    ey.sort();
    ox.sort();
    oy.sort();

    let mut ans = 0;
    for i in 0..ex.len() {
        ans += (1 - ex.len() as i64 + i as i64 * 2) * ex[i];
    }
    for i in 0..ey.len() {
        ans += (1 - ey.len() as i64 + i as i64 * 2) * ey[i];
    }
    for i in 0..ox.len() {
        ans += (1 - ox.len() as i64 + i as i64 * 2) * ox[i];
    }
    for i in 0..oy.len() {
        ans += (1 - oy.len() as i64 + i as i64 * 2) * oy[i];
    }
    println!("{}", ans / 2);
}
