//#[derive_readable]
#[derive(Debug, Clone, PartialEq, Eq)]
struct Sheet {
    width: usize,
    height: usize,
    width_i64: i64,
    height_i64: i64,
    sheet: Vec<Vec<u8>>,
}

impl Sheet {
    fn new(width: usize, height: usize, sheet: Vec<Vec<u8>>) -> Sheet {
        Sheet {
            width,
            height,
            width_i64: width as i64,
            height_i64: height as i64,
            sheet,
        }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        0 <= pos.x && pos.x < self.width_i64 && 0 <= pos.y && pos.y < self.height_i64
    }

    fn empty_sheet(width: usize, height: usize) -> Sheet {
        Sheet::new(width, height, vec![vec![b'.'; width]; height])
    }

    fn is_black(&self, pos: Pos<i64>) -> bool {
        self.sheet[pos.y as usize][pos.x as usize] == b'#'
    }

    fn read() -> Sheet {
        input! {
            height: usize,
            width: usize,
            sheet: [Bytes; height],
        }
        Sheet::new(width, height, sheet)
    }

    // self の pos の位置が左上になるように other_sheet を乗せる
    fn put(&mut self, other_sheet: &Sheet, pos: Pos<i64>) -> bool {
        for other_sheet_pos in iproduct!(0..other_sheet.width, 0..other_sheet.height)
            .map(|(x, y)| Pos::new(x as i64, y as i64))
            .filter(|other_sheet_pos| other_sheet.is_black(*other_sheet_pos))
        {
            let self_pos = other_sheet_pos + pos;
            if !self.is_within(self_pos) {
                return false;
            }
        }

        for other_sheet_pos in iproduct!(0..other_sheet.width, 0..other_sheet.height)
            .map(|(x, y)| Pos::new(x as i64, y as i64))
            .filter(|other_sheet_pos| other_sheet.is_black(*other_sheet_pos))
        {
            let self_pos = other_sheet_pos + pos;
            self.sheet[self_pos.y as usize][self_pos.x as usize] = b'#';
        }
        true
    }
}
#[derive(Debug, Clone)]
struct Problem {
    src1: Sheet,
    src2: Sheet,
    dst: Sheet,
}

impl Problem {
    fn read() -> Problem {
        let src1 = Sheet::read();
        let src2 = Sheet::read();
        let dst = Sheet::read();
        Problem { src1, src2, dst }
    }
    fn solve(&self) -> Answer {
        let src1 = &self.src1;
        let src2 = &self.src2;
        let dst = &self.dst;

        let ans = iproduct!(
            -(src1.height_i64 - 1)..dst.height_i64 + (src1.height_i64 - 1),
            -(src1.width_i64 - 1)..dst.width_i64 + (src1.width_i64 - 1),
            -(src2.height_i64 - 1)..dst.height_i64 + (src2.height_i64 - 1),
            -(src2.width_i64 - 1)..dst.width_i64 + (src2.width_i64 - 1)
        )
        .map(|(y1, x1, y2, x2)| (Pos::new(x1, y1), Pos::new(x2, y2)))
        .any(|(p1, p2)| {
            let mut sheet = Sheet::empty_sheet(dst.width, dst.height);
            if !sheet.put(src1, p1) {
                return false;
            }
            if !sheet.put(src2, p2) {
                return false;
            }
            &sheet == dst
        });

        Answer { ans }
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
    ans: bool,
}

impl Answer {
    fn print(&self) {
        print_yesno(self.ans);
        //println!("{}", self.ans);
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

    #[allow(dead_code)]
    #[derive(Debug)]
    struct WrongTestCase {
        problem: Problem,
        main_ans: Answer,
        naive_ans: Answer,
    }

    #[allow(dead_code)]
    fn check(p: &Problem) -> Option<WrongTestCase> {
        let main_ans = p.solve();
        let naive_ans = p.solve_naive();
        if main_ans != naive_ans {
            Some(WrongTestCase {
                problem: p.clone(),
                main_ans,
                naive_ans,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn make_random_problem() -> Problem {
        todo!()
        // let mut rng = SmallRng::seed_from_u64(42);
        // // let mut rng = SmallRng::from_entropy();
        // let n = rng.gen_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        return; // テスト実行するときはこの return を消す。
        let num_tests = 1000;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem();
            let result = check(&p);
            if let Some(wrong_test_case) = result {
                wrong_cases.push(wrong_test_case);
            }
            if wrong_cases.len() >= max_wrong_case {
                break;
            }
        }

        if !wrong_cases.is_empty() {
            for t in &wrong_cases {
                println!("{:?}", t.problem);
                println!("main ans : {:?}", t.main_ans);
                println!("naive ans: {:?}", t.naive_ans);
                println!();
            }
            println!("{} cases are wrong.", wrong_cases.len());
            panic!();
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
    impl Pos<i64> {
        pub fn around4_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR4_LIST.iter().copied().map(move |d| d + self)
        }
        pub fn around8_pos_iter(self) -> impl Iterator<Item = Pos<i64>> {
            DIR8_LIST.iter().copied().map(move |d| d + self)
        }
    }
}
