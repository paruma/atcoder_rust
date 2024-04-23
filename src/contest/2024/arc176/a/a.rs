//#[derive_readable]
#[derive(Debug)]
struct Problem {
    size: usize,
    n_pos: usize,
    pos_list: Vec<Pos<usize>>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            size: usize,
            n_pos: usize,
            pos_list: [(Usize1, Usize1); n_pos],
        }

        let pos_list = pos_list
            .iter()
            .copied()
            .map(|(y, x)| Pos::new(x, y))
            .collect_vec();
        Problem {
            size,
            n_pos,
            pos_list,
        }
    }
    fn solve(&self) -> Answer {
        let n_pos = self.n_pos;
        let size = self.size;
        // ジャッジを作る
        let mut row_cnts = vec![0; size];
        let mut col_cnts = vec![0; size];
        let pos_set = self.pos_list.iter().copied().collect::<HashSet<_>>();
        for &pos in &self.pos_list {
            row_cnts[pos.y] += 1;
            col_cnts[pos.x] += 1;
        }

        let mut col_cnt_not2_idx = HashSet::new();
        for x in 0..size {
            if col_cnts[x] != n_pos {
                col_cnt_not2_idx.insert(x);
            }
        }

        let mut ans: Vec<Pos<usize>> = self.pos_list.clone();

        for y in 0..size {
            let mut col_cnt_2_idx_list = vec![];
            for &x in &col_cnt_not2_idx {
                if row_cnts[y] == n_pos {
                    break;
                }
                let p = Pos::new(x, y);
                if pos_set.contains(&p) || col_cnts[x] == n_pos {
                    continue;
                }

                ans.push(p);

                row_cnts[y] += 1;
                col_cnts[x] += 1;

                if col_cnts[x] == n_pos {
                    col_cnt_2_idx_list.push(x);
                    // col_cnt_not2_idx.remove(&x);
                }
            }
            for x in col_cnt_2_idx_list {
                dbg!(col_cnt_not2_idx.remove(&x));
            }
        }
        dbg!(&col_cnts);
        dbg!(&row_cnts);

        Answer { ans }
    }

    fn judge(&self, ans: &Answer) -> bool {
        let size = self.size;
        // 行でジャッジ
        let mut row_cnts = vec![0; size];
        let mut col_cnts = vec![0; size];

        let all_pos = chain!(self.pos_list.iter().copied(), ans.ans.iter().copied()).collect_vec();

        // ans.ans と self の方を連結して行う
        for &p in &all_pos {
            row_cnts[p.y] += 1;
            col_cnts[p.x] += 1;
        }

        let expected = (0..size).map(|_| 2).collect_vec();

        assert_eq!(row_cnts, expected);
        assert_eq!(col_cnts, expected);

        row_cnts == expected && col_cnts == expected
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        todo!();
        // let ans = 0;
        // Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<Pos<usize>>,
}

impl Answer {
    #[fastout]
    fn print(&self) {
        // fastout つけておく
        println!("{}", self.ans.len());
        for p in &self.ans {
            println!("{} {}", p.y + 1, p.x + 1);
        }
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
use pos::*;
pub mod pos {
    use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos<T> {
        pub x: T,
        pub y: T,
    }
    impl<T> Pos<T> {
        pub fn new(x: T, y: T) -> Pos<T> {
            Pos { x, y }
        }
    }
    impl<T: Mul<Output = T> + Copy> Pos<T> {
        pub fn scala_mul(self, rhs: T) -> Pos<T> {
            Pos::new(self.x * rhs, self.y * rhs)
        }
    }
    impl<T: Add<Output = T> + Mul<Output = T> + Copy> Pos<T> {
        pub fn norm_square(self) -> T {
            self.x * self.x + self.y * self.y
        }
    }
    impl<T: Add<Output = T> + Copy> Add for Pos<T> {
        type Output = Pos<T>;
        fn add(self, rhs: Self) -> Self::Output {
            Pos::new(self.x + rhs.x, self.y + rhs.y)
        }
    }
    impl<T: Sub<Output = T> + Copy> Sub for Pos<T> {
        type Output = Pos<T>;
        fn sub(self, rhs: Self) -> Self::Output {
            Pos::new(self.x - rhs.x, self.y - rhs.y)
        }
    }
    impl<T: Neg<Output = T>> Neg for Pos<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Pos::new(-self.x, -self.y)
        }
    }
    impl<T: num_traits::Zero + Copy> num_traits::Zero for Pos<T> {
        fn zero() -> Self {
            Pos::new(T::zero(), T::zero())
        }
        fn is_zero(&self) -> bool {
            self.x.is_zero() && self.y.is_zero()
        }
    }
    impl<T: Add<Output = T> + Copy> AddAssign for Pos<T> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs
        }
    }
    impl<T: Sub<Output = T> + Copy> SubAssign for Pos<T> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs
        }
    }
    pub const DIR8_LIST: [Pos<i64>; 8] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 1, y: -1 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: -1 },
        Pos { x: -1, y: 0 },
        Pos { x: -1, y: 1 },
    ];
    pub const DIR4_LIST: [Pos<i64>; 4] = [
        Pos { x: 0, y: 1 },
        Pos { x: 1, y: 0 },
        Pos { x: 0, y: -1 },
        Pos { x: -1, y: 0 },
    ];
}
