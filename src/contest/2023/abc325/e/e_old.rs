//#[derive_readable]
struct Problem {
    n_cities: usize,           // N
    car_speed: i64,            // A
    train_speed: i64,          // B
    train_offset: i64,         // C
    dist_table: Vec<Vec<i64>>, // D
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_cities: usize,                         // N
            car_speed: i64,                          // A
            train_speed: i64,                        // B
            train_offset: i64,                       // C
            dist_table: [[i64; n_cities]; n_cities], // D
        }
        Problem {
            n_cities,
            car_speed,
            train_speed,
            train_offset,
            dist_table,
        }
    }
    fn time_len_train(&self, src: usize, dst: usize) -> i64 {
        self.dist_table[src][dst] * self.train_speed + self.train_offset
    }

    fn time_len_car(&self, src: usize, dst: usize) -> i64 {
        self.dist_table[src][dst] * self.car_speed
    }

    fn solve(&self) -> Answer {
        let n_cities = self.n_cities;
        // 3つめ電車を使ったかどうか
        let mut p_queue: BinaryHeap<(Reverse<ExtInt>, usize, bool)> = BinaryHeap::new();
        // 社用車のみ
        // 電車あり（電車を使ったら電車しか使えない）
        let mut dp_car: Vec<ExtInt> = vec![Inf; n_cities];
        let mut finished_car: Vec<bool> = vec![false; n_cities];
        let mut dp_train: Vec<ExtInt> = vec![Inf; n_cities];
        let mut finished_train: Vec<bool> = vec![false; n_cities];
        p_queue.push((Reverse(Fin(0)), 0, false));
        dp_car[0] = Fin(0);
        dp_train[0] = Fin(0);

        while let Some((Reverse(d), current, used_train)) = p_queue.pop() {
            // current から次の点を計算
            if used_train {
                if finished_train[current] {
                    continue;
                }
                finished_train[current] = true;
            } else {
                if finished_car[current] {
                    continue;
                }
                finished_car[current] = true;
            }
            for next in 0..n_cities {
                if used_train {
                    // 電車しか使えない
                    if dp_train[next] > dp_train[current] + Fin(self.time_len_train(current, next))
                    {
                        dp_train[next] =
                            dp_train[current] + Fin(self.time_len_train(current, next));
                        p_queue.push((Reverse(dp_train[next]), next, true));
                    }
                } else {
                    // 電車も車も使える
                    // 車の方が早ければ車だけ使う
                    #[allow(clippy::collapsible_if)]
                    if self.time_len_car(current, next) <= self.time_len_train(current, next) {
                        if dp_car[next] > dp_car[current] + Fin(self.time_len_car(current, next)) {
                            dp_car[next] = dp_car[current] + Fin(self.time_len_car(current, next));
                            dp_train[next] = min(
                                dp_train[next],
                                dp_car[current] + Fin(self.time_len_car(current, next)),
                            );
                            p_queue.push((Reverse(dp_car[next]), next, false));
                        }
                    } else {
                        // 車を使う方と電車を使う方の両方を考える
                        if dp_car[next] > dp_car[current] + Fin(self.time_len_car(current, next)) {
                            dp_car[next] = dp_car[current] + Fin(self.time_len_car(current, next));
                            p_queue.push((Reverse(dp_car[next]), next, false));
                        }

                        if dp_train[next]
                            > dp_car[current] + Fin(self.time_len_train(current, next))
                        {
                            dp_train[next] =
                                dp_car[current] + Fin(self.time_len_train(current, next));
                            p_queue.push((Reverse(dp_train[next]), next, true));
                        }
                    }
                }
            }
        }

        let ans = min(dp_car[n_cities - 1], dp_train[n_cities - 1]).get_fin();
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

use std::{
    cmp::{min, Reverse},
    collections::BinaryHeap,
};

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
use mod_ext_int::ExtInt::{self, *};
pub mod mod_ext_int {
    use std::{cmp::Ordering, ops::Add};
    use ExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtInt {
        Inf,
        Fin(i64),
    }
    impl ExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                Inf => panic!("called `ExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                Inf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_inf(self) -> bool {
            matches!(self, Inf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                Inf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> ExtInt {
            match opt {
                Some(a) => Fin(a),
                None => Inf,
            }
        }
    }
    impl Add for ExtInt {
        type Output = ExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (Inf, Inf) => Inf,
                (Inf, Fin(_)) => Inf,
                (Fin(_), Inf) => Inf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl PartialOrd for ExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (Inf, Inf) => Some(Ordering::Equal),
                (Inf, Fin(_)) => Some(Ordering::Greater),
                (Fin(_), Inf) => Some(Ordering::Less),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for ExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
}
