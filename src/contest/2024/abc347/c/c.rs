// #[derive_readable]
#[derive(Debug)]
struct Problem {
    n: usize,
    a: i64, // 休日
    b: i64, // 平日
    ds: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            a: i64, // 休日
            b: i64, // 平日
            ds: [i64; n],
        }
        Problem { n, a, b, ds }
    }
    fn solve(&self) -> Answer {
        // 座標圧縮を使用したもの
        let a = self.a;
        let b = self.b;
        let n = self.n;
        let ds = &self.ds;
        let ds_mod = ds.iter().copied().map(|x| x % (a + b)).collect_vec();
        let ds_mod_plus_apb = ds_mod.iter().copied().map(|x| x + (a + b)).collect_vec();
        let ds_mod_loop = chain!(ds_mod, ds_mod_plus_apb).collect_vec();
        let cc = CoordinateCompression::new(&ds_mod_loop);

        let ds_mod_loop_compress = cc.compress_vec(&ds_mod_loop);

        let max_coord = cc.max_coord();
        let mut cnts = vec![0_i64; max_coord + 1];

        for &x in &ds_mod_loop_compress {
            cnts[x] += 1;
        }
        let cnts_cumsum = CumSum::new(&cnts);

        let ans = (0..=max_coord / 2)
            .map(|begin| {
                // cnts[begin..end] >= n となるような最小の end を求める

                let end = bin_search((max_coord + 1) as i64, -1, |end| {
                    cnts_cumsum.get_interval_sum(begin, end as usize) >= (n as i64)
                });

                let begin_decom = cc.decompress(begin);
                let end_decom = cc.decompress(end as usize - 1); // 閉区間にする
                end_decom - begin_decom + 1
            })
            .any(|x| x <= a);
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // ソートによる解法
        let a = self.a;
        let b = self.b;
        let n = self.n;
        let ds = &self.ds;
        // 各予定の曜日(今日を0とする)のリスト
        let ds_mod = ds.iter().copied().map(|x| x % (a + b)).collect_vec();
        let ds_mod_plus_apb = ds_mod.iter().copied().map(|x| x + (a + b)).collect_vec();
        let ds_mod_loop = chain!(ds_mod, ds_mod_plus_apb).sorted().collect_vec();
        let ans = (0..n)
            // iから始まるn個の予定の曜日を含む曜日区間の長さの最小値
            .map(|i| ds_mod_loop[n + i - 1] - ds_mod_loop[i] + 1) // i..i+n での区間の最大-最小 + 1
            .any(|x| x <= a);
        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // circular_tuple_windows を使ったもの
        let a = self.a;
        let b = self.b;
        let ds = &self.ds;

        let ds_mod = ds
            .iter()
            .copied()
            .map(|x| x % (a + b))
            .sorted()
            .dedup()
            .collect_vec();
        // let ans = (0..n)
        //     .map(|i| (ds_mod[(n + i - 1).rem_euclid(n)] - ds_mod[i]).rem_euclid(a + b) + 1)
        //     .any(|x| x <= a);
        let ans = ds_mod
            .iter()
            .copied()
            .circular_tuple_windows()
            .map(|(end_inclusive, begin)| (end_inclusive - begin).rem_euclid(a + b) + 1)
            .any(|x| x <= a);
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let a = self.a;
        let b = self.b;
        let ds = &self.ds;
        let ans = (0..(a + b)).any(|today| {
            ds.iter()
                .copied()
                .map(|d| today + d)
                .all(|d| d % (a + b) < a)
        });
        Answer { ans }
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
    Problem::read().solve3().print();
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
        assert_eq!(p.solve3(), p.solve_naive());
    }

    fn make_random_problem() -> Problem {
        let n = 4;
        let mut rng = SmallRng::from_entropy();

        let a = rng.gen_range(1..=4);
        let b = rng.gen_range(1..=4);
        let ds = (0..n).map(|_| rng.gen_range(1..=10)).collect_vec();
        let p = Problem { n, a, b, ds };
        dbg!(&p);
        p
    }

    #[test]
    fn test_with_naive() {
        // 手動でテストを作るのもOK
        for _ in 0..10000 {
            let p = make_random_problem();
            check(&p);
        }
    }
}

use coordinate_compression::CoordinateCompression;
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
pub mod coordinate_compression {
    use itertools::Itertools;

    pub struct CoordinateCompression {
        space: Vec<i64>,
    }

    impl CoordinateCompression {
        /// 計算量: O(|space|log(|space|))
        pub fn new(space: &[i64]) -> Self {
            let space = space.iter().copied().sorted().dedup().collect_vec();
            Self { space }
        }

        /// 計算量: O(log(|space|))
        pub fn compress(&self, x: i64) -> usize {
            self.space.binary_search(&x).unwrap()
        }

        /// 計算量: O(|xs|log(|space|))
        pub fn compress_vec(&self, xs: &[i64]) -> Vec<usize> {
            xs.iter().copied().map(|x| self.compress(x)).collect_vec()
        }

        /// 計算量: O(1)
        pub fn decompress(&self, i: usize) -> i64 {
            self.space[i]
        }

        pub fn max_coord(&self) -> usize {
            self.space.len() - 1
        }
    }
}

use cumsum::*;
pub mod cumsum {
    pub struct CumSum {
        pub cumsum: Vec<i64>,
    }
    impl CumSum {
        /// 計算量: O(|xs|)
        pub fn new(xs: &[i64]) -> CumSum {
            let mut cumsum = vec![0; xs.len() + 1];
            for i in 1..xs.len() + 1 {
                cumsum[i] = cumsum[i - 1] + xs[i - 1];
            }
            CumSum { cumsum }
        }
        /// 計算量: O(1)
        pub fn get_interval_sum(&self, begin: usize, end: usize) -> i64 {
            self.cumsum[end] - self.cumsum[begin]
        }
    }
}

/// 二分探索をする
/// ```text
/// ng ng ng ok ok ok
///          ↑ここの引数の値を返す
/// ```
/// 計算量: O(log(|ok - ng|))
/// ## Arguments
/// * ok != ng
/// * |ok - ng| <= 2^63 - 1, |ok + ng| <= 2^63 - 1
/// * p の定義域について
///     * ng < ok の場合、p は区間 ng..ok で定義されている。
///     * ok < ng の場合、p は区間 ok..ng で定義されている。
/// * p の単調性について
///     * ng < ok の場合、p は単調増加
///     * ok < ng の場合、p は単調減少
/// ## Return
/// * ng < ok の場合: I = { i in ng..ok | p(i) == true } としたとき
///     * I が空でなければ、min I を返す。
///     * I が空ならば、ok を返す。
/// * ok < ng の場合: I = { i in ok..ng | p(i) == true } としたとき
///     * I が空でなければ、max I を返す。
///     * I が空ならば、ok を返す。
pub fn bin_search<F>(mut ok: i64, mut ng: i64, p: F) -> i64
where
    F: Fn(i64) -> bool,
{
    assert!(ok != ng);
    assert!(ok.checked_sub(ng).is_some());
    assert!(ok.checked_add(ng).is_some());
    while num::abs(ok - ng) > 1 {
        let mid = (ok + ng) / 2;
        assert!(mid != ok);
        assert!(mid != ng);
        if p(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}
