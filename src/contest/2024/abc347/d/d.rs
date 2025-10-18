#[derive_readable]
#[derive(Debug)]
struct Problem {
    x_pc: u32,
    y_pc: u32,
    c: u64,
}

fn to_bits(x: u64) -> Vec<u64> {
    (0..60).rev().map(|i| (x >> i) & 1).collect()
}

fn from_bits(x_bits: &[u64]) -> u64 {
    x_bits
        .iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, b)| b << i)
        .sum()
}

impl Problem {
    fn read() -> Problem {
        input! {
            p: Problem
        }
        p
    }
    fn solve(&self) -> Answer {
        let x_pc = self.x_pc;
        let y_pc = self.y_pc;
        let c = self.c;
        let c_pc = c.count_ones();

        // 最初から i32(i64) でよかった感じがする
        let x_and_y_pc_times2 = (x_pc + y_pc) as i32 - c_pc as i32;
        let ans = if x_and_y_pc_times2 % 2 == 0
            && 0 <= x_and_y_pc_times2
            && x_and_y_pc_times2 / 2 <= u32::min(x_pc, y_pc) as i32
            && x_and_y_pc_times2 / 2 <= (60 - c_pc) as i32
        {
            let x_and_y_pc = (x_and_y_pc_times2 / 2) as u32;
            // (x&y)&c=0 となるようにする
            let c_bits = to_bits(c);
            let mut x_bits = vec![0; 60];
            let mut y_bits = vec![0; 60];

            let c_bits_0_pos = c_bits.iter().copied().positions(|b| b == 0).collect_vec();
            let c_bits_1_pos = c_bits.iter().copied().positions(|b| b == 1).collect_vec();

            // ここは、take ではなくスライス使うべきだった
            // take だと足りない場合にエラーにならない
            for i in c_bits_0_pos.iter().copied().take(x_and_y_pc as usize) {
                x_bits[i] = 1;
                y_bits[i] = 1;
            }
            for &i in &c_bits_1_pos[0..(x_pc - x_and_y_pc) as usize] {
                x_bits[i] = 1;
            }

            for &i in
                &c_bits_1_pos[(x_pc - x_and_y_pc) as usize..(x_pc + y_pc - 2 * x_and_y_pc) as usize]
            {
                y_bits[i] = 1;
            }

            Some((from_bits(&x_bits), from_bits(&y_bits)))
        } else {
            None
        };

        let ans = Answer { ans };
        // assert!(self.check_ans(&ans));
        self.assert_check_ans(&ans);
        ans
    }

    fn solve2(&self) -> Answer {
        let x_pc = self.x_pc as i64;
        let y_pc = self.y_pc as i64;
        let c = self.c;
        let c_pc = c.count_ones() as i64;

        let ans = (|| {
            if (x_pc + y_pc - c_pc) % 2 != 0 {
                return None;
            }
            // cnt_11: x[i] = 1 かつ y[i] = 1 となる i の数(ビットの数)
            // cnt_10: x[i] = 1 かつ y[i] = 0 となる i の数(ビットの数)
            // cnt_01: x[i] = 0 かつ y[i] = 1 となる i の数(ビットの数)
            // x xor y = c を考えると、以下の連立方程式が成り立つ
            // cnt_10 + cnt_11 = x_pc
            // cnt_01 + cnt_11 = y_pc
            // cnt_01 + cnt_10 = c_pc
            // これらの連立方程式を解くと、cnt_11, cnt_10, cnt_01 は以下のように求まる。
            let cnt_11 = (x_pc + y_pc - c_pc) / 2;
            let cnt_10 = x_pc - cnt_11;
            let cnt_01 = y_pc - cnt_11;
            if cnt_11 + cnt_10 + cnt_01 > 60 || cnt_11 < 0 || cnt_10 < 0 || cnt_01 < 0 {
                return None;
            }
            let mut x = BitSet::new(0);
            let mut y = BitSet::new(0);
            let c = BitSet::new(c as usize);

            // (0, 1) か (1, 0) を詰めていくための iterator
            let mut iter1 = {
                let sub1 = std::iter::repeat((1, 0)).take(cnt_10 as usize);
                let sub2 = std::iter::repeat((0, 1)).take(cnt_01 as usize);
                chain!(sub1, sub2)
            };

            //
            let mut iter2 = {
                let sub1 = std::iter::repeat((1, 1)).take(cnt_11 as usize);
                let sub2 = std::iter::repeat((0, 0));
                chain!(sub1, sub2)
            };

            // x と y を構成していく。c[i] によって、詰める値を変えていく。
            for i in 0..60 {
                if c.contains(i) {
                    // (x[i], y[i]) に (0, 1) か (1, 0) を入れる

                    let (xi, yi) = iter1.next().unwrap();
                    if xi == 1 {
                        x = x.insert(i);
                    }
                    if yi == 1 {
                        y = y.insert(i);
                    }
                } else {
                    // (x[i], y[i]) に (1, 1) か (0, 0) を入れる
                    let (xi, yi) = iter2.next().unwrap();
                    if xi == 1 {
                        x = x.insert(i);
                    }
                    if yi == 1 {
                        y = y.insert(i);
                    }
                }
            }

            Some((x.to_bit() as u64, y.to_bit() as u64))
        })();

        let ans = Answer { ans };
        // assert!(self.check_ans(&ans));
        self.assert_check_ans(&ans);
        ans
    }

    fn check_ans(&self, ans: &Answer) -> bool {
        if let Some((x, y)) = ans.ans {
            x < 2_u64.pow(60)
                && y < 2_u64.pow(60)
                && x.count_ones() == self.x_pc
                && y.count_ones() == self.y_pc
                && x ^ y == self.c
        } else {
            true
        }
    }

    fn assert_check_ans(&self, ans: &Answer) {
        if let Some((x, y)) = ans.ans {
            assert!(x < 2_u64.pow(60));
            assert!(y < 2_u64.pow(60));
            assert!(x.count_ones() == self.x_pc);
            assert!(y.count_ones() == self.y_pc);
            assert!(x ^ y == self.c);
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
    ans: Option<(u64, u64)>,
}

impl Answer {
    fn print(&self) {
        if let Some((x, y)) = self.ans {
            println!("{} {}", x, y);
        } else {
            println!("-1");
        }
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
        let mut rng = SmallRng::from_os_rng();
        let x_pc = rng.random_range(0..=60);
        let y_pc = rng.random_range(0..=60);
        let c = rng.random_range(0..2_u64.pow(60));
        let p = Problem { x_pc, y_pc, c };
        // dbg!(&p);
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..100 {
            let p = make_random_problem();
            p.solve();
        }
    }
}

use itertools::chain;
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
use bitset::*;
#[allow(clippy::module_inception)]
pub mod bitset {
    use itertools::Itertools;
    use std::{
        fmt::{Error, Formatter},
        ops::{BitAnd, BitOr, BitXor},
    };
    #[derive(Clone, Copy, PartialEq, Eq)]
    pub struct BitSet {
        bit: usize,
    }
    impl BitSet {
        #[inline]
        pub fn new(bit: usize) -> BitSet {
            BitSet { bit }
        }
        pub fn to_bit(self) -> usize {
            self.bit
        }
        /// 持っている要素を Vec<usize> で返す
        pub fn to_vec(self, len: usize) -> Vec<usize> {
            (0..len).filter(|i| (self.bit >> i) & 1 == 1).collect_vec()
        }
        pub fn contains(self, x: usize) -> bool {
            (self.bit >> x) & 1 == 1
        }
        pub fn count(self) -> usize {
            self.bit.count_ones() as usize
        }
        pub fn insert(self, x: usize) -> BitSet {
            BitSet::new(self.bit | (1 << x))
        }
        pub fn remove(self, x: usize) -> BitSet {
            BitSet::new(self.bit & !(1 << x))
        }
        pub fn empty() -> BitSet {
            BitSet::new(0)
        }
        pub fn universal_set(size: usize) -> BitSet {
            BitSet::new((1 << size) - 1)
        }
        pub fn complement(self, size: usize) -> BitSet {
            BitSet::new(self.bit ^ ((1 << size) - 1))
        }
        pub fn set_minus(self, other: BitSet) -> BitSet {
            BitSet::new(self.bit & !other.bit)
        }
        pub fn is_empty(self) -> bool {
            self.bit == 0
        }
        pub fn is_subset(self, other: BitSet) -> bool {
            self | other == other
        }
        pub fn all_subset(size: usize) -> impl Iterator<Item = BitSet> {
            (0..(1 << size)).map(BitSet::new)
        }
        pub fn subsets(self) -> impl Iterator<Item = BitSet> {
            std::iter::successors(Some(self.bit), move |x| {
                if *x == 0 {
                    None
                } else {
                    Some((x - 1) & self.bit)
                }
            })
            .map(BitSet::new)
        }
    }
    impl BitAnd for BitSet {
        type Output = BitSet;
        fn bitand(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit & rhs.bit)
        }
    }
    impl BitOr for BitSet {
        type Output = BitSet;
        fn bitor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit | rhs.bit)
        }
    }
    impl BitXor for BitSet {
        type Output = BitSet;
        fn bitxor(self, rhs: BitSet) -> BitSet {
            BitSet::new(self.bit ^ rhs.bit)
        }
    }
    use std::fmt::Debug;
    impl Debug for BitSet {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            f.write_fmt(format_args!("{:#b}", self.bit))?;
            Ok(())
        }
    }
}
