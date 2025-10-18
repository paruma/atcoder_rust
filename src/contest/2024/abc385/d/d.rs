#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Move {
    dir: char,
    len: i64,
}

#[derive(Debug, Clone)]
struct Problem {
    n_house: usize,
    n_moves: usize,
    init_pos: Pos<i64>,
    house_pos_list: Vec<Pos<i64>>,
    moves: Vec<Move>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_house: usize,
            n_moves: usize,
            init_pos: (i64, i64),
            house_pos_list: [(i64, i64); n_house],
            moves: [Move; n_moves],
        }
        let init_pos = Pos::new(init_pos.0, init_pos.1);
        let house_pos_list = house_pos_list
            .iter()
            .copied()
            .map(|(x, y)| Pos::new(x, y))
            .collect_vec();
        Problem {
            n_house,
            n_moves,
            init_pos,
            house_pos_list,
            moves,
        }
    }

    fn solve(&self) -> Answer {
        let mut current_pos = self.init_pos;

        // 訪問したら消していく
        let mut house_set_row = HashMap::<i64, BTreeMap<i64, usize>>::new();
        let mut house_set_col = HashMap::<i64, BTreeMap<i64, usize>>::new();

        for (house_idx, house_pos) in self.house_pos_list.iter().copied().enumerate() {
            house_set_col
                .entry(house_pos.x)
                .or_default()
                .insert(house_pos.y, house_idx);

            house_set_row
                .entry(house_pos.y)
                .or_default()
                .insert(house_pos.x, house_idx);
        }

        for m in &self.moves {
            let dir_map = hashmap! {
                'L' => Pos { x: -1, y: 0 },
                'R' => Pos { x: 1, y: 0 },
                'U' => Pos { x: 0, y: 1 },
                'D' => Pos { x: 0, y: -1 },
            };

            let next_pos = current_pos + dir_map[&m.dir].scala_mul(m.len);

            // 通過したindex
            let idx_visited = match m.dir {
                'U' | 'D' => {
                    let min_y = i64::min(current_pos.y, next_pos.y);
                    let max_y = i64::max(current_pos.y, next_pos.y);
                    // min_y..=max_y の間にある家を消していく。

                    if let Some(map) = house_set_col.get(&current_pos.x) {
                        map.range(min_y..=max_y).map(|(_, idx)| *idx).collect_vec()
                    } else {
                        vec![]
                    }
                }
                'L' | 'R' => {
                    let min_x = i64::min(current_pos.x, next_pos.x);
                    let max_x = i64::max(current_pos.x, next_pos.x);
                    // min_x..=max_x の間にある家を消していく。

                    if let Some(map) = house_set_row.get(&current_pos.y) {
                        map.range(min_x..=max_x).map(|(_, idx)| *idx).collect_vec()
                    } else {
                        vec![]
                    }
                }
                _ => panic!(),
            };

            for i in idx_visited {
                let house_pos = self.house_pos_list[i];

                house_set_col
                    .entry(house_pos.x)
                    .or_default()
                    .remove(&house_pos.y);

                house_set_row
                    .entry(house_pos.y)
                    .or_default()
                    .remove(&house_pos.x);
            }

            current_pos = next_pos;
            //
        }

        let cnt1 = house_set_row.values().map(|set| set.len()).sum::<usize>();
        let cnt2 = house_set_col.values().map(|set| set.len()).sum::<usize>();

        assert_eq!(cnt1, cnt2);
        let cnt_house_remain = cnt1;
        let cnt_house_visited = self.n_house - cnt_house_remain;
        Answer {
            final_pos: current_pos,
            cnt_house: cnt_house_visited,
        }
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
    final_pos: Pos<i64>,
    cnt_house: usize,
}

impl Answer {
    fn print(&self) {
        println!(
            "{} {} {}",
            self.final_pos.x, self.final_pos.y, self.cnt_house
        );
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
    fn make_random_problem(rng: &mut SmallRng) -> Problem {
        todo!()
        // let n = rng.random_range(1..=10);
        // let p = Problem { _a: n };
        // println!("{:?}", &p);
        // p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        let mut wrong_cases: Vec<WrongTestCase> = vec![];
        for _ in 0..num_tests {
            let p = make_random_problem(&mut rng);
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
use maplit::hashmap;
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
#[allow(unused_imports)]
use std::cmp::Reverse;
use std::collections::BTreeMap;
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
    pub fn print_chars(chars: &[char]) {
        let msg = chars.iter().collect::<String>();
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
        pub fn inner_product(self, rhs: Self) -> T {
            self.x * rhs.x + self.y * rhs.y
        }
        pub fn norm_square(self) -> T {
            self.inner_product(self)
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
