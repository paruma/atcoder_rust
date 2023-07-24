use itertools::Itertools;
use proconio::input;

fn read() -> (usize, Vec<i64>) {
    input! {
        n: usize,
        a: [i64;n]
    }
    (n, a)
}
fn comb2(n: usize) -> i64 {
    let ni = n as i64;
    ni * (ni - 1) / 2
}

fn solve(n: usize, a: &[i64]) -> i64 {
    let mut a = a.to_vec();
    a.sort_unstable();

    // 全部異なってたらnC2
    comb2(n)
        - a.iter()
            .group_by(|&key| *key)
            .into_iter()
            .map(|(_, group)| {
                let cnt = group.count();
                comb2(cnt)
            })
            .sum::<i64>()
}

//fn output() {}

fn main() {
    let (n, a) = read();
    let ans = solve(n, &a);
    println!("{}", ans);
}
