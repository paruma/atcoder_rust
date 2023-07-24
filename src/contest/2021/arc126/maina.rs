#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::input;

//------snippet------

//-------------------
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Info {
    n2: i64,
    n3: i64,
    n4: i64,
}

fn read() -> (usize, Vec<Info>) {
    input! {
        //from OnceSource::from(""),
        t: usize,
        infos: [(i64, i64, i64); t]
    }
    let infos = infos
        .iter()
        .map(|(n2, n3, n4)| Info {
            n2: *n2,
            n3: *n3,
            n4: *n4,
        })
        .collect_vec();
    (t, infos)
}

#[allow(unused_assignments)]
fn solve0(info: &Info) -> i64 {
    let Info { n2, n3, n4 } = *info;

    let mut n6 = n3 / 2;

    let mut ans = 0;
    let mut n2 = n2;
    let mut n4 = n4;

    // 6+4
    if n6 <= n4 {
        // 6使い切り
        ans += n6;
        n4 -= n6;
        n6 = 0;
    } else {
        // 6余り
        ans += n4;

        n6 -= n4;
        n4 = 0;
    }

    // 6+2+2 (4+4+2とどっちを先にやるべき？)
    if n6 * 2 <= n2 {
        // 2の必要数がn6 * 2
        //6使い切り
        ans += n6;
        let n_use2 = n6 * 2;
        n2 -= n_use2;
        n6 = 0
    } else {
        ans += n2 / 2;
        n6 -= n2 / 2;
        n2 %= 2;
    }

    // 4+4+2
    if n2 * 2 <= n4 {
        //2使い切り
        // 4の必要数がn2*2
        ans += n2;
        let n_use4 = n2 * 2;
        n4 -= n_use4;
        n2 = 0
    } else {
        //4使い切り
        ans += n4 / 2;
        n2 -= n4 / 2;
        n4 %= 2;
    }

    // 4+2+2+2
    if 3 * n4 <= n2 {
        // 4使い切り
        ans += n4;
        let n_use2 = 3 * n4;
        n2 -= n_use2;
        n4 = 0;
    } else {
        //2使い切り
        ans += n2 / 3;
        n4 -= n2 / 3;
        n2 %= 3;
    }

    // 2+2+2+2+2
    ans += n2 / 5;
    n2 %= 5;
    ans
}

fn solve(_t: usize, infos: &[Info]) -> Vec<i64> {
    infos.iter().map(solve0).collect_vec()
}

fn output(ans: &[i64]) {
    for a in ans {
        println!("{}", *a);
    }
}

fn main() {
    let (t, infos) = read();
    let ans = solve(t, &infos);
    output(&ans);
    //println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let ans = solve0(&Info {
            n2: 100,
            n3: 100,
            n4: 100,
        });
        println!("{}", ans);
    }
}
