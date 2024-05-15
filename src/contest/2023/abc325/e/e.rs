//#[derive_readable]
struct Problem {
    n_cities: usize,           // N
    car_speed: i64,            // A
    train_speed: i64,          // B
    train_offset: i64,         // C
    dist_table: Vec<Vec<i64>>, // D
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum PosTransportation {
    Car(usize),
    Train(usize),
}

use PosTransportation::*;
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

    fn encode(&self, pt: PosTransportation) -> usize {
        match pt {
            Car(i) => i,
            Train(i) => self.n_cities + i,
        }
    }

    fn decode(&self, i: usize) -> PosTransportation {
        if i < self.n_cities {
            Car(i)
        } else {
            Train(i - self.n_cities)
        }
    }

    //頂点倍化した世界のグラフの重み
    fn edge_weight(&self, src: usize, dst: usize) -> ExtInt {
        let src_pt = self.decode(src);
        let dst_pt = self.decode(dst);
        match (src_pt, dst_pt) {
            (Car(s), Car(d)) => Fin(self.time_len_car(s, d)),
            (Car(s), Train(d)) => {
                if s == d {
                    Fin(0)
                } else {
                    Inf
                }
            }
            (Train(_), Car(_)) => Inf,
            (Train(s), Train(d)) => Fin(self.time_len_train(s, d)),
        }
    }

    fn solve(&self) -> Answer {
        // ダイクストラ法 頂点倍化解法 O(|E|log|V|)
        let n_cities = self.n_cities;

        // 頂点0 からのコスト
        let dp: Vec<ExtInt> = {
            let mut pq: BinaryHeap<(Reverse<ExtInt>, usize)> = BinaryHeap::new();
            let init_pos = self.encode(Car(0)); // 0
            let mut dp: Vec<ExtInt> = vec![Inf; n_cities * 2]; // init_pos からの距離
            pq.push((Reverse(Fin(0)), init_pos));
            dp[init_pos] = Fin(0);

            while let Some((Reverse(d), current)) = pq.pop() {
                if d > dp[current] {
                    continue;
                }
                for next in 0..n_cities * 2 {
                    let w = self.edge_weight(current, next);
                    if d + w < dp[next] {
                        dp[next] = d + w;
                        pq.push((Reverse(dp[next]), next));
                    }
                }
            }
            dp
        };

        let dp_last_car = dp[self.encode(Car(n_cities - 1))];
        let dp_last_train = dp[self.encode(Train(n_cities - 1))];
        let ans = min(dp_last_car, dp_last_train).get_fin();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // ダイクストラ法 頂点倍化解法 (O(|V|^2))
        let n_cities = self.n_cities;

        // 頂点0 からのコスト
        let dp: Vec<ExtInt> = {
            let init_pos = self.encode(Car(0)); // 0
            let mut dp: Vec<ExtInt> = vec![Inf; n_cities * 2]; // init_pos からの距離
            let mut fixed: Vec<bool> = vec![false; n_cities * 2];
            dp[init_pos] = Fin(0);

            for _ in 0..n_cities * 2 {
                let min_v = (0..n_cities * 2)
                    .filter(|i| !fixed[*i])
                    .min_by_key(|i| dp[*i])
                    .unwrap();

                for next in 0..n_cities * 2 {
                    if fixed[next] {
                        continue;
                    }
                    dp[next] = min(dp[next], dp[min_v] + self.edge_weight(min_v, next));
                }
                fixed[min_v] = true;
            }
            dp
        };

        let dp_last_car = dp[self.encode(Car(n_cities - 1))];
        let dp_last_train = dp[self.encode(Train(n_cities - 1))];
        let ans = min(dp_last_car, dp_last_train).get_fin();
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
    Problem::read().solve2().print();
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
use pathfinding::matrix::directions::S;
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
