#[derive_readable]
#[derive(Debug)]
struct Problem {
    n: i64,
    a: i64,
    x: i64,
    y: i64,
}

struct Rec2 {
    dp: HashMap<i64, f64>,
    a: i64,
    x: i64,
    y: i64,
}

impl Rec2 {
    fn new(a: i64, x: i64, y: i64) -> Self {
        let dp = HashMap::new();
        Self { dp, a, x, y }
    }

    fn rec(&mut self, n: i64) -> f64 {
        if let Some(&ans) = self.dp.get(&n) {
            return ans;
        }
        let ans = if n == 0 {
            0.0
        } else {
            // 決定的な方
            let cand1 = self.rec(n / self.a) + (self.x as f64);

            // 確率的な方
            let cand2 = {
                let sum = self.rec(n / 2)
                    + self.rec(n / 3)
                    + self.rec(n / 4)
                    + self.rec(n / 5)
                    + self.rec(n / 6);
                (sum / 6.0 + self.y as f64) / (5.0 / 6.0)
            };
            f64::min(cand1, cand2)
        };

        self.dp.insert(n, ans);
        ans
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let n = self.n;
        let a = self.a;
        let x = self.x;
        let y = self.y;
        let mut rec2 = Rec2::new(a, x, y);
        let ans = rec2.rec(n);
        //dbg!(&rec2.dp);
        Answer { ans }
    }

    // memoise 検証
    // メソッド内では memoise は使えない
    // #[memoise_map(n)]
    // fn rec2(&self, n: i64) -> f64 {
    //     if n == 0 {
    //         return 0.0;
    //     }
    //     // 決定的な方
    //     let cand1 = self.rec2(n / self.a) + (self.x as f64);

    //     // 確率的な方
    //     let cand2 = {
    //         let sum = self.rec2(n / 2)
    //             + self.rec2(n / 3)
    //             + self.rec2(n / 4)
    //             + self.rec2(n / 5)
    //             + self.rec2(n / 6);
    //         (sum / 6.0 + self.y as f64) / (5.0 / 6.0)
    //     };
    //     f64::min(cand1, cand2)
    // }

    thread_local!(static REC2:std::cell::RefCell<std::collections::BTreeMap<(i64),f64> >  = std::cell::RefCell::new(std::collections::BTreeMap::new()));
    fn rec2_reset() {
        Self::REC2.with(|cache| {
            let mut r = cache.borrow_mut();
            r.clear();
        });
    }
    fn rec2(&self, n: i64) -> f64 {
        if let Some(ret) = Self::REC2.with(|cache| cache.borrow().get(&(n)).cloned()) {
            return ret.clone();
        }
        let ret: f64 = (|| {
            if n == 0 {
                return 0.0;
            }
            let cand1 = self.rec2(n / self.a) + (self.x as f64);
            let cand2 = {
                let sum = self.rec2(n / 2)
                    + self.rec2(n / 3)
                    + self.rec2(n / 4)
                    + self.rec2(n / 5)
                    + self.rec2(n / 6);
                (sum / 6.0 + self.y as f64) / (5.0 / 6.0)
            };
            f64::min(cand1, cand2)
        })();
        Self::REC2.with(|cache| {
            let mut bm = cache.borrow_mut();
            bm.insert((n), ret.clone());
        });
        ret
    }

    fn solve2(&self) -> Answer {
        let ans = self.rec2(self.n);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    ans: f64,
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
    #[allow(unused_imports)]
    use rand::{rngs::SmallRng, seq::SliceRandom, *};

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }

    fn check(p: &Problem) {
        assert_eq!(p.solve(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            // let p = make_random_problem();
            // check(&p);
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use itertools::{chain, iproduct, izip, Itertools};
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Usize1},
};
#[allow(unused_imports)]
use std::collections::{BinaryHeap, HashMap, HashSet};

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
