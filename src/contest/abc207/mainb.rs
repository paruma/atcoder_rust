use proconio::input;

fn read() -> (i64, i64, i64, i64) {
    input! {a:i64, b:i64, c:i64, d:i64}
    (a, b, c, d)
}

fn solve(a: i64, b: i64, c: i64, d: i64) -> Option<i64> {
    // min {n | a + nb <= d*nc }
    // a + n(b-dc) <= 0
    // a <= n(dc-b)
    // a/(dc-b) <= n (if dc - d > 0)
    // a/(dc-b) >= n (if dc -d < 0)
    // a <= 0 (if bc -d = 0)

    (d * c - b > 0).then(|| num_integer::div_ceil(a, d * c - b))
}

fn output(ans: Option<i64>) {
    let ans_out = ans.unwrap_or(-1);
    println!("{}", ans_out);
}

fn main() {
    let (a, b, c, d) = read();
    let ans = solve(a, b, c, d);
    output(ans);
}
