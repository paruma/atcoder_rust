//#[derive_readable]
struct Problem {
    n: usize,
    ps: Vec<usize>,
    lr_list: Vec<u8>,
}

use ac_library::ModInt998244353 as Mint;

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            ps: [Usize1; n],
            lr_list: Bytes,
        }
        Problem { n, ps, lr_list }
    }

    fn solve(&self) -> Answer {
        let n = self.n;
        let ps = &self.ps;
        let lr_list = &self.lr_list;

        let modn = |x: i64| {
            if x >= 0 {
                x % n as i64
            } else {
                (x % n as i64 + n as i64) % n as i64
            }
        };

        // 左: -1, 右: +1
        let ans = [1, -1]
            .iter()
            .map(|&p0_lr| {
                match lr_list[ps[0]] {
                    b'L' => {
                        if p0_lr == 1 {
                            // +1 は右を表す
                            return Mint::new(0);
                        }
                    }
                    b'R' => {
                        if p0_lr == -1 {
                            // -1 は左を表す
                            return Mint::new(0);
                        }
                    }
                    b'?' => {}
                    _ => unreachable!(),
                };

                let mut visited = vec![false; n];
                visited[ps[0]] = true;

                let mut cnts = vec![];

                for i in 1..n {
                    let current = ps[i];
                    // next はcurrent の右隣 or 左隣 (0番目の人が選択していない左右)
                    // next の方にあるスプーンを取らない
                    let next = modn(-p0_lr + (current as i64)) as usize;
                    let cnt = if visited[next] {
                        match lr_list[current] {
                            // b'L' => {
                            //     if p0_lr == -1 {
                            //         1
                            //     } else {
                            //         0
                            //     }
                            // }
                            // b'R' => {
                            //     if p0_lr == 1 {
                            //         1
                            //     } else {
                            //         0
                            //     }
                            // }
                            b'L' => 1,
                            b'R' => 1,
                            b'?' => 2,
                            _ => unreachable!(),
                        }
                    } else {
                        match lr_list[current] {
                            // current の利き手は 0番目に取った人の利き手でないといけない。
                            b'L' => {
                                if p0_lr == -1 {
                                    1
                                } else {
                                    0
                                }
                            }
                            b'R' => {
                                if p0_lr == 1 {
                                    1
                                } else {
                                    0
                                }
                            }
                            b'?' => 1,
                            _ => unreachable!(),
                        }
                    };

                    cnts.push(Mint::new(cnt));
                    visited[current] = true;
                }

                //dbg!(cnts.iter().product::<i64>());
                cnts.iter().product::<Mint>()
            })
            .sum::<Mint>()
            .val();
        Answer { ans: ans as i64 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

use ac_library::ModInt998244353;
// ====== import ======
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};

// ====== output func ======
#[allow(unused_imports)]
use print_vec::*;
pub mod print_vec {
    use itertools::Itertools;
    use proconio::fastout;
    #[fastout]
    pub fn print_vec<T: std::fmt::Debug>(arr: &[T]) {
        for a in arr {
            println!("{:?}", a);
        }
    }
    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Debug>(arr: &[T]) {
        let msg = arr.iter().map(|x| format!("{:?}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Debug>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{:?}", x)).join(" ");
            println!("{}", msg);
        }
    }
    pub fn print_bytes(bytes: &[u8]) {
        let msg = String::from_utf8(bytes.to_vec()).unwrap();
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec_bytes(vec_bytes: &[Vec<u8>]) {
        for row in vec_bytes {
            let msg = String::from_utf8(row.to_vec()).unwrap();
            println!("{}", msg);
        }
    }
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
