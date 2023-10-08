use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let rad = if x > a * b / 2.0 * a {
        // 三角形: a * (a * tan(t)) / 2 = a^2 * tan(t) / 2
        // 平行四辺形: (a / cos(t)) * (b * cos(t)) = a * b
        // x = (平行四辺形 - 三角形) * a = (a * b - a^2 * tan(t) / 2) * a = a^2 * b - a^3 / 2 * tan(t)
        // tan(t) = (a^2 * b - x) / (a^3 / 2)
        // t = arctan((a^2 * b - x) / (a^3 / 2))
        f64::atan((a * a * b - x) / (a * a * a / 2.0))
    } else {
        // 三角形: b * (b / tan(t)) / 2 = b^2 / tan(t) / 2
        // x = b^2 / tan(t) / 2 * a
        // tan(t) = (a * b^2) / (2 * x)
        // t = arctan((a * b^2) / (2 * x))
        f64::atan((a * b * b) / (2.0 * x))
    };
    println!("{}", rad * 180.0 / PI);
}
