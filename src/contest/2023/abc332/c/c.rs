//#[derive_readable]
struct Problem {
    n_days: usize,
    n_plain_shirts: usize,
    plans: Vec<u8>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_days: usize,
            n_plain_shirts: usize,
            plans: Bytes,
        }
        Problem {
            n_days,
            n_plain_shirts,
            plans,
        }
    }
    fn solve(&self) -> Answer {
        let Problem {
            n_days,
            n_plain_shirts,
            plans,
        } = self;
        // 0: 予定なし (Tシャツ洗濯)
        // 1: 食事に行く予定 (無地 or ロゴ入りを1枚着る)
        // 2: 競技プログラミングのイベントに行く予定 (ロゴ入りを1枚着る)
        let mut n_logo_sharts = 0; //買う場合に増やす
        let mut used_plain_sharts = 0;
        let mut used_logo_sharts = 0;

        for &ch in plans {
            match ch {
                b'0' => {
                    used_plain_sharts = 0;
                    used_logo_sharts = 0;
                }
                b'1' => {
                    // 無地が余っている
                    if *n_plain_shirts > used_plain_sharts {
                        used_plain_sharts += 1;
                    } else {
                        if n_logo_sharts > used_logo_sharts {
                            // ロゴシャツが余っている
                            used_logo_sharts += 1;
                        } else {
                            // ロゴシャツを買う
                            n_logo_sharts += 1;
                            used_logo_sharts += 1;
                        }
                    }
                }
                b'2' => {
                    if n_logo_sharts > used_logo_sharts {
                        // ロゴシャツが余っている
                        used_logo_sharts += 1;
                    } else {
                        // ロゴシャツを買う
                        n_logo_sharts += 1;
                        used_logo_sharts += 1;
                    }
                }
                _ => panic!(),
            }
        }

        let ans = n_logo_sharts;
        Answer { ans }
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
