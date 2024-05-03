use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u64
    }
    let cc = c.count_ones();
    let max = if a + b > 60 { 120 - a - b } else { a + b };
    let min = a.max(b) - a.min(b);
    if cc <= max && cc >= min && (cc - min) % 2 == 0 {
        let mut d = (cc - min) / 2;
        let mut xc = a;
        let mut yc = b;
        let mut x: u64 = 0;
        let mut y: u64 = 0;
        for i in 0..60 {
            if (c >> i) & 1 == 1 {
                if a < b {
                    if d > 0 {
                        d -= 1;
                        xc -= 1;
                        x += 1 << i;
                    } else {
                        y += 1 << i;
                    }
                } else {
                    if d > 0 {
                        d -= 1;
                        yc -= 1;
                        y += 1 << i;
                    } else {
                        x += 1 << i;
                    }
                }
            } else {
                if a < b {
                    if xc > 0 {
                        xc -= 1;
                        x += 1 << i;
                        y += 1 << i;
                    }
                } else {
                    if yc > 0 {
                        yc -= 1;
                        x += 1 << i;
                        y += 1 << i;
                    }
                }
            }
        }
        println!("{} {}", x, y);
    } else {
        println!("-1");
    }
}
