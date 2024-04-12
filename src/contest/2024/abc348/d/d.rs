#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Medicine {
    pos: Pos<i64>,
    energy: i64,
}
#[derive(Debug)]
struct Problem {
    w: usize,
    h: usize,
    grid: Vec<Vec<u8>>,
    medicines: Vec<Medicine>,
}

struct Grid {
    grid: Vec<Vec<u8>>,
    h: usize,
    w: usize,
}

impl Grid {
    fn new(grid: Vec<Vec<u8>>) -> Grid {
        let h = grid.len();
        let w = grid[0].len();
        Grid { grid, h, w }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }

    fn at(&self, pos: Pos<i64>) -> &u8 {
        if self.is_within(pos) {
            self.grid.at(pos)
        } else {
            &b'#'
        }
    }

    fn player_can_move(&self, pos: Pos<i64>) -> bool {
        // at の実装上、is_within のチェックは不要
        self.is_within(pos) && b".ST".contains(self.at(pos))
    }

    fn start_pos(&self) -> Pos<i64> {
        iproduct!(0..self.w, 0..self.h)
            .map(|(x, y)| Pos::new(x as i64, y as i64))
            .find(|p| *self.grid.at(*p) == b'S')
            .unwrap()
    }

    fn goal_pos(&self) -> Pos<i64> {
        iproduct!(0..self.w, 0..self.h)
            .map(|(x, y)| Pos::new(x as i64, y as i64))
            .find(|p| *self.grid.at(*p) == b'T')
            .unwrap()
    }
}

struct MedicineMap {
    medicines_grid: Vec<Vec<Option<Medicine>>>,
}

impl MedicineMap {
    fn new(medicines: &Vec<Medicine>, h: usize, w: usize) -> MedicineMap {
        let mut medicines_grid = vec![vec![None; w]; h];
        for med in medicines {
            *medicines_grid.at_mut(med.pos) = Some(*med);
        }
        MedicineMap { medicines_grid }
    }

    fn get_energy(&self, pos: Pos<i64>) -> Option<i64> {
        self.medicines_grid.at(pos).map(|x| x.energy)
    }
}

struct Dp {
    dp: Vec<Vec<NegExtInt>>,
    h: usize,
    w: usize,
}

impl Dp {
    fn new(h: usize, w: usize) -> Dp {
        let dp = vec![vec![NegExtInt::NegInf; w]; h];
        Dp { dp, h, w }
    }

    fn is_within(&self, pos: Pos<i64>) -> bool {
        let h = self.h as i64;
        let w = self.w as i64;
        0 <= pos.y && pos.y < h && 0 <= pos.x && pos.x < w
    }

    fn at(&self, pos: Pos<i64>) -> &NegExtInt {
        if self.is_within(pos) {
            self.dp.at(pos)
        } else {
            &NegExtInt::NegInf
        }
    }

    fn at_mut(&mut self, pos: Pos<i64>) -> &mut NegExtInt {
        self.dp.at_mut(pos)
    }
}

impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            grid: [Bytes; h],
            n_medicines: usize,
            medicines: [Medicine; n_medicines],
        }
        let medicines = medicines
            .iter()
            .copied()
            .map(|medicine| Medicine {
                pos: Pos {
                    x: medicine.pos.x - 1,
                    y: medicine.pos.y - 1,
                },
                energy: medicine.energy,
            })
            .collect_vec();
        Problem {
            w,
            h,
            grid,
            medicines,
        }
    }

    fn solve(&self) -> Answer {
        let grid = Grid::new(self.grid.clone());
        let mut pq: BinaryHeap<(NegExtInt, Pos<i64>)> = BinaryHeap::new();
        let start_pos = grid.start_pos();
        let goal_pos = grid.goal_pos();
        let mut dp = Dp::new(grid.h, grid.w);
        let medicine_map = MedicineMap::new(&self.medicines, grid.h, grid.w);

        // ここは NegExtInt::Fin(0) が正しい（いろいろ噛み合ってこれでも通るが）
        pq.push((NegExtInt::NegInf, start_pos));
        *dp.at_mut(start_pos) = NegExtInt::NegInf;

        // 薬は取って移動にする。DPの値は薬を取る前

        while let Some((current_energy, current_pos)) = pq.pop() {
            if current_energy < *dp.at(current_pos) {
                continue;
            }

            for next_pos in DIR4_LIST
                .iter()
                .copied()
                .map(|dir| dir + current_pos)
                .filter(|pos| grid.player_can_move(*pos))
            {
                // 薬を（必要なら）取って移動する
                let next_energy = {
                    let mut next_energy = current_energy;
                    if let Some(medicine_energy) = medicine_map.get_energy(current_pos) {
                        next_energy = next_energy.max(NegExtInt::Fin(medicine_energy))
                    }
                    next_energy += NegExtInt::Fin(-1);
                    if next_energy < NegExtInt::Fin(0) {
                        next_energy = NegExtInt::NegInf;
                    }
                    next_energy
                };

                if next_energy > *dp.at(next_pos) {
                    *dp.at_mut(next_pos) = next_energy;
                    pq.push((next_energy, next_pos));
                }
            }
        }
        let ans = dp.at(goal_pos).to_option().map(|x| x >= 0).unwrap_or(false);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // bfs をする解法

        let grid = Grid::new(self.grid.clone());
        let start_pos = grid.start_pos();
        let goal_pos = grid.goal_pos();

        // スタートに薬がない場合はゴールに到達不可能
        if !self.medicines.iter().any(|x| x.pos == start_pos) {
            return Answer { ans: false };
        }

        // ゴールに薬がなければ、ダミーの薬を追加する
        let medicines = {
            let mut medicines = self.medicines.clone();

            if !medicines.iter().any(|x| x.pos == goal_pos) {
                medicines.push(Medicine {
                    pos: goal_pos,
                    energy: 0,
                });
            }
            medicines
        };

        // 各薬から到達可能な薬のリストを得る。(隣接リスト)
        let adj_medicines: Vec<Vec<usize>> = {
            medicines
                .iter()
                .copied()
                .map(|medicine| {
                    // medicines.pos から medicine.energy の手数で各点が到達可能か調べる
                    let can_reach: Vec<Vec<bool>> = {
                        let mut visited = vec![vec![false; grid.w]; grid.h];
                        struct State {
                            pos: Pos<i64>,
                            energy: i64,
                        }
                        let mut queue: Queue<State> = Queue::new();

                        *visited.at_mut(medicine.pos) = true;
                        queue.push(State {
                            pos: medicine.pos,
                            energy: medicine.energy,
                        });

                        while let Some(State {
                            pos: current_pos,
                            energy: current_energy,
                        }) = queue.pop()
                        {
                            if current_energy <= 0 {
                                continue;
                            }

                            for next_pos in DIR4_LIST
                                .iter()
                                .copied()
                                .map(|dir| dir + current_pos)
                                .filter(|pos| grid.player_can_move(*pos))
                            {
                                if *visited.at(next_pos) {
                                    continue;
                                }
                                *visited.at_mut(next_pos) = true;
                                queue.push(State {
                                    pos: next_pos,
                                    energy: current_energy - 1,
                                });
                            }
                        }
                        visited
                    };

                    // 到達可能な薬を調べる
                    medicines
                        .iter()
                        .copied()
                        .positions(|other_medicine| *can_reach.at(other_medicine.pos))
                        .collect_vec()
                })
                .collect_vec()
        };

        // スタートの薬からゴールの薬に到達可能か調べる
        let start_medicine_idx = medicines.iter().position(|x| x.pos == start_pos).unwrap();
        let goal_medicine_idx = medicines.iter().position(|x| x.pos == goal_pos).unwrap();

        let ans = {
            let mut visited: Vec<bool> = vec![false; medicines.len()];
            let mut queue: Queue<usize> = Queue::new();
            visited[start_medicine_idx] = true;
            queue.push(start_medicine_idx);

            while let Some(current_medicine_idx) = queue.pop() {
                for &next_medicine_idx in &adj_medicines[current_medicine_idx] {
                    if visited[next_medicine_idx] {
                        continue;
                    }
                    visited[next_medicine_idx] = true;
                    queue.push(next_medicine_idx);
                }
            }
            visited[goal_medicine_idx]
        };

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
    }
}

fn main() {
    Problem::read().solve2().print();
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
            let p = make_random_problem();
            check(&p);
        }
    }
}

use std::collections::BinaryHeap;

use itertools::iproduct;
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
use pos::*;
pub mod pos {
    use proconio::source::{Readable, Source};
    use std::{
        io::BufRead,
        ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
    };
    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
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
    impl<T: Readable<Output = T>> Readable for Pos<T> {
        type Output = Self;
        fn read<R: BufRead, S: Source<R>>(source: &mut S) -> Self::Output {
            let y = T::read(source);
            let x = T::read(source);
            Pos { x, y }
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

use vec_vec_at::*;
pub mod vec_vec_at {
    use super::pos::*;
    use easy_ext::ext;
    #[ext]
    impl<T> Vec<Vec<T>> {
        pub fn at(&self, pos: Pos<i64>) -> &T {
            &self[pos.y as usize][pos.x as usize]
        }
        pub fn at_mut(&mut self, pos: Pos<i64>) -> &mut T {
            &mut self[pos.y as usize][pos.x as usize]
        }
    }
}
use mod_neg_ext_int::NegExtInt::{self, *};
pub mod mod_neg_ext_int {
    use ac_library::Monoid;
    use std::{
        cmp::Ordering,
        convert::Infallible,
        ops::{Add, AddAssign},
    };
    use NegExtInt::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NegExtInt {
        NegInf,
        Fin(i64),
    }
    impl NegExtInt {
        pub fn get_fin(self) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => panic!("called `NegExtInt::get_fin()` on a `Fin` value"),
            }
        }
        pub fn get_fin_or(self, default: i64) -> i64 {
            match self {
                Fin(val) => val,
                NegInf => default,
            }
        }
        pub fn is_fin(self) -> bool {
            matches!(self, Fin(_))
        }
        pub fn is_neginf(self) -> bool {
            matches!(self, NegInf)
        }
        pub fn to_option(self) -> Option<i64> {
            match self {
                NegInf => None,
                Fin(a) => Some(a),
            }
        }
        pub fn from_option(opt: Option<i64>) -> NegExtInt {
            match opt {
                Some(a) => Fin(a),
                None => NegInf,
            }
        }
        pub fn times(self, t: i64) -> Self {
            match t.cmp(&0) {
                Ordering::Less => panic!("t must be non-negative."),
                Ordering::Equal => Fin(0),
                Ordering::Greater => match self {
                    NegInf => NegInf,
                    Fin(a) => Fin(a * t),
                },
            }
        }
    }
    impl Add for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: Self) -> Self::Output {
            match (self, rhs) {
                (NegInf, NegInf) => NegInf,
                (NegInf, Fin(_)) => NegInf,
                (Fin(_), NegInf) => NegInf,
                (Fin(a), Fin(b)) => Fin(a + b),
            }
        }
    }
    impl AddAssign for NegExtInt {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }
    impl Add<i64> for NegExtInt {
        type Output = NegExtInt;
        fn add(self, rhs: i64) -> Self::Output {
            match self {
                NegInf => NegInf,
                Fin(a) => Fin(a + rhs),
            }
        }
    }
    impl AddAssign<i64> for NegExtInt {
        fn add_assign(&mut self, rhs: i64) {
            *self = *self + rhs;
        }
    }
    impl std::iter::Sum for NegExtInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            let mut s = 0;
            for x in iter {
                match x {
                    NegInf => return NegInf,
                    Fin(x) => s += x,
                }
            }
            Fin(s)
        }
    }
    impl PartialOrd for NegExtInt {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match (self, other) {
                (NegInf, NegInf) => Some(Ordering::Equal),
                (NegInf, Fin(_)) => Some(Ordering::Less),
                (Fin(_), NegInf) => Some(Ordering::Greater),
                (Fin(a), Fin(b)) => PartialOrd::partial_cmp(a, b),
            }
        }
    }
    impl Ord for NegExtInt {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    pub struct NegExtIntAdditive(Infallible);
    impl Monoid for NegExtIntAdditive {
        type S = NegExtInt;
        fn identity() -> Self::S {
            Fin(0)
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a + *b
        }
    }
    pub struct NegExtIntMax(Infallible);
    impl Monoid for NegExtIntMax {
        type S = NegExtInt;
        fn identity() -> Self::S {
            NegInf
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }
}

use mod_queue::*;
pub mod mod_queue {
    use std::collections::VecDeque;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Queue<T> {
        raw: VecDeque<T>,
    }
    impl<T> Queue<T> {
        pub fn new() -> Self {
            Queue {
                raw: VecDeque::new(),
            }
        }
        pub fn push(&mut self, value: T) {
            self.raw.push_front(value)
        }
        pub fn pop(&mut self) -> Option<T> {
            self.raw.pop_back()
        }
        pub fn peek(&self) -> Option<&T> {
            self.raw.back()
        }
        pub fn is_empty(&self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<T> Default for Queue<T> {
        fn default() -> Self {
            Self::new()
        }
    }
}
