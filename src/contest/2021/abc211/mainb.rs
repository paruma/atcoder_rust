#![allow(clippy::let_unit_value)]
use proconio::input;

fn read() -> Vec<String> {
    input! {strs: [String;4]}
    strs
}

fn solve(strs: &[String]) -> bool {
    let mut ideal = vec!["H", "2B", "3B", "HR"]
        .iter()
        .map(|&x| String::from(x))
        .collect::<Vec<_>>();
    ideal.sort();
    let mut strs = strs.to_vec();
    strs.sort();
    strs == ideal
}

fn output(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

fn main() {
    let strs = read();
    let ans = solve(&strs);
    output(ans);
    //println!("{}", ans);
}
