use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        ab: [(usize, usize); n]
    }
    let ptns1 = ptns(&ab, 0, h, w);
    let ptns2 = ptns(&ab, 1, h, w);
    let ptns3 = ptns(&ab, 2, h, w);
    let ptns4 = ptns(&ab, 3, h, w);
    let ptns5 = ptns(&ab, 4, h, w);
    let ptns6 = ptns(&ab, 5, h, w);
    let ptns7 = ptns(&ab, 6, h, w);
    for &(x1, y1, (a1, b1)) in &ptns1 {
        let cnt1 = if a1 > 0 { a1 * b1 } else { 0 };
        for &(x2, y2, (a2, b2)) in &ptns2 {
            let c = if a2 > 0 && !col((x1, y1, (a1, b1)), (x2, y2, (a2, b2))) {
                a2 * b2
            } else {
                0
            };
            let cnt2 = cnt1 + c;
            if cnt2 == h * w {
                println!("Yes");
                return;
            }
            for &(x3, y3, (a3, b3)) in &ptns3 {
                let c = if a3 > 0
                    && !col((x1, y1, (a1, b1)), (x3, y3, (a3, b3)))
                    && !col((x2, y2, (a2, b2)), (x3, y3, (a3, b3)))
                {
                    a3 * b3
                } else {
                    0
                };
                let cnt3 = cnt2 + c;
                if cnt3 == h * w {
                    println!("Yes");
                    return;
                }
                for &(x4, y4, (a4, b4)) in &ptns4 {
                    let c = if a4 > 0
                        && !col((x1, y1, (a1, b1)), (x4, y4, (a4, b4)))
                        && !col((x2, y2, (a2, b2)), (x4, y4, (a4, b4)))
                        && !col((x3, y3, (a3, b3)), (x4, y4, (a4, b4)))
                    {
                        a4 * b4
                    } else {
                        0
                    };
                    let cnt4 = cnt3 + c;
                    if cnt4 == h * w {
                        println!("Yes");
                        return;
                    }
                    for &(x5, y5, (a5, b5)) in &ptns5 {
                        let c = if a5 > 0
                            && !col((x1, y1, (a1, b1)), (x5, y5, (a5, b5)))
                            && !col((x2, y2, (a2, b2)), (x5, y5, (a5, b5)))
                            && !col((x3, y3, (a3, b3)), (x5, y5, (a5, b5)))
                            && !col((x4, y4, (a4, b4)), (x5, y5, (a5, b5)))
                        {
                            a5 * b5
                        } else {
                            0
                        };
                        let cnt5 = cnt4 + c;
                        if cnt5 == h * w {
                            println!("Yes");
                            return;
                        }
                        for &(x6, y6, (a6, b6)) in &ptns6 {
                            let c = if a6 > 0
                                && !col((x1, y1, (a1, b1)), (x6, y6, (a6, b6)))
                                && !col((x2, y2, (a2, b2)), (x6, y6, (a6, b6)))
                                && !col((x3, y3, (a3, b3)), (x6, y6, (a6, b6)))
                                && !col((x4, y4, (a4, b4)), (x6, y6, (a6, b6)))
                                && !col((x5, y5, (a5, b5)), (x6, y6, (a6, b6)))
                            {
                                a6 * b6
                            } else {
                                0
                            };
                            let cnt6 = cnt5 + c;
                            if cnt6 == h * w {
                                println!("Yes");
                                return;
                            }
                            for &(x7, y7, (a7, b7)) in &ptns7 {
                                let c = if a7 > 0
                                    && !col((x1, y1, (a1, b1)), (x7, y7, (a7, b7)))
                                    && !col((x2, y2, (a2, b2)), (x7, y7, (a7, b7)))
                                    && !col((x3, y3, (a3, b3)), (x7, y7, (a7, b7)))
                                    && !col((x4, y4, (a4, b4)), (x7, y7, (a7, b7)))
                                    && !col((x5, y5, (a5, b5)), (x7, y7, (a7, b7)))
                                    && !col((x6, y6, (a6, b6)), (x7, y7, (a7, b7)))
                                {
                                    a7 * b7
                                } else {
                                    0
                                };
                                let cnt7 = cnt6 + c;
                                if cnt7 == h * w {
                                    println!("Yes");
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("No");
}

fn col(
    (x, y, (a, b)): (usize, usize, (usize, usize)),
    (x2, y2, (a2, b2)): (usize, usize, (usize, usize)),
) -> bool {
    if a == 0 || a2 == 0 {
        return false;
    }
    return (x <= x2 + a2 - 1) && (x + a - 1 >= x2) && ((y <= y2 + b2 - 1) && (y + b - 1 >= y2));
}

fn ptns(
    ab: &Vec<(usize, usize)>,
    i: usize,
    h: usize,
    w: usize,
) -> Vec<(usize, usize, (usize, usize))> {
    let mut ptns = vec![(0, 0, (0, 0))];
    if i > ab.len() - 1 {
        return ptns;
    }
    let ab = ab[i];
    if w >= ab.0 && h >= ab.1 {
        for (x, y) in iproduct!(0..w - ab.0 + 1, 0..h - ab.1 + 1) {
            ptns.push((x, y, (ab.0, ab.1)));
        }
    }
    if ab.0 == ab.1 {
        return ptns;
    }
    if w >= ab.1 && h >= ab.0 {
        for (x, y) in iproduct!(0..w - ab.1 + 1, 0..h - ab.0 + 1) {
            ptns.push((x, y, (ab.1, ab.0)));
        }
    }
    ptns
}
