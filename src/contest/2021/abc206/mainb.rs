use proconio::input;

fn read() -> i64 {
    input! {n: i64}
    n
}

fn solve(n: i64) -> i64 {
    let mut money: i64 = 0;
    let mut day = 0;
    while money < n {
        day += 1;
        money += day;
    }
    day
}

//fn output() {}

fn main() {
    let n = read();
    let ans = solve(n);
    println!("{}", ans);
}
