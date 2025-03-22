//#[derive_readable]
#[derive(Debug, Clone)]
struct Problem {
    n: usize,
    xs: Vec<i64>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            xs: [i64; n],
        }
        Problem { n, xs }
    }
    #[allow(clippy::vec_init_then_push)]
    fn solve(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let xs = &self.xs;

        // dp[i][k][p][q] = xs[0..i] で 長さk の部分文字列を取って、初項を xs の添字 p から取ってきて、2項目が xs[q] であるようなものの数
        let mut dp = vec![vec![vec![vec![Mint::new(0); n]; n]; n + 1]; n + 1];

        // k=0 の初期化
        for i in 0..=n {
            for p in 0..n {
                for q in 0..n {
                    dp[i][0][p][q] = Mint::new(1);
                }
            }
        }

        // k=1 の初期化
        for i in 0..=n {
            for p in 0..i {
                for q in 0..n {
                    dp[i][1][p][q] = Mint::new(1);
                }
            }
        }

        for i in 1..n {
            for k in 1..n {
                for p in 0..n {
                    for q in 0..n {
                        let term1 = if xs[i] == xs[p] + (k as i64) * (xs[q] - xs[p]) {
                            dp[i][k][p][q]
                        } else {
                            Mint::new(0)
                        };
                        let term2 = dp[i][k + 1][p][q];
                        dp[i + 1][k + 1][p][q] = term1 + term2;
                    }
                }
            }
        }

        let uniq_idx = xs
            .iter()
            .copied()
            .enumerate()
            .unique_by(|(_, x)| *x)
            .map(|(i, _)| i)
            .collect_vec();

        let mut ans = vec![];

        ans.push(n as i64); // k = 1の答え
        for k in 2..=n {
            let sub_ans = iproduct!(0..n, uniq_idx.iter().copied())
                .map(|(p, q)| dp[n][k][p][q])
                .sum::<Mint>()
                .val() as i64;
            ans.push(sub_ans);
        }

        // for i in 0..=n {
        //     for k in 0..=n {
        //         eprintln!("i={}, k={}", i, k);
        //         for p in 0..n {
        //             eprintln!("{}", dp[i][k][p].iter().join(" "));
        //         }
        //     }
        // }

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // HashMap を使った DP の実装
        // defaultdict に相当するものがないとかなり厳しい。
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let xs = &self.xs;

        // dp[i][k][c][d] = xs[0..i] で 長さk の部分文字列を取って、初項がcで公差がdとなる場合の数
        let mut dp = vec![vec![HashMap::<i64, HashMap<i64, Mint>>::new(); n + 1]; n + 1];

        // diff_list[i] = 初項がxs[i] のときのありえる公差
        let diff_list = {
            let mut diff_list = vec![HashSet::<i64>::new(); n];
            for i in 0..n {
                for j in i + 1..n {
                    diff_list[i].insert(xs[j] - xs[i]);
                }
            }
            diff_list
        };

        // k=1 の初期化
        for i in 0..=n {
            for p in 0..i {
                for &d in &diff_list[p] {
                    *dp[i][1]
                        .entry(xs[p])
                        .or_insert(HashMap::new())
                        .entry(d)
                        .or_insert(Mint::new(0)) += 1;
                }
            }
        }

        for i in 1..n {
            for k in 1..n {
                let mut next_dp = HashMap::new();
                // 初項
                for &c in dp[i][k].keys() {
                    // 公差
                    for &d in dp[i][k][&c].keys() {
                        let term1 = if xs[i] == c + (k as i64) * d {
                            dp[i][k][&c][&d]
                        } else {
                            Mint::new(0)
                        };
                        let term2 = dp[i][k + 1]
                            .get(&c)
                            .unwrap_or(&HashMap::new())
                            .get(&d)
                            .copied()
                            .unwrap_or(Mint::new(0));
                        *next_dp
                            .entry(c)
                            .or_insert(HashMap::new())
                            .entry(d)
                            .or_insert(Mint::new(0)) = term1 + term2;
                    }
                }
                dp[i + 1][k + 1] = next_dp;
            }
        }

        let mut ans = vec![];

        ans.push(n as i64); // k = 1の答え
        for k in 2..=n {
            let sub_ans = dp[n][k]
                .values()
                .flat_map(|dp_sub| dp_sub.values())
                .sum::<Mint>()
                .val() as i64;
            ans.push(sub_ans);
        }

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        // DefaultHashMap を使った DP の実装
        use ac_library::ModInt998244353 as Mint;
        let n = self.n;
        let xs = &self.xs;

        // dp[i][k][c][d] = xs[0..i] で 長さk の部分文字列を取って、初項がcで公差がdとなる場合の数
        let mut dp =
            vec![vec![DefaultHashMap::<i64, DefaultHashMap<i64, Mint>>::default(); n + 1]; n + 1];

        // diff_list[i] = 初項がxs[i] のときのありえる公差
        let diff_list = {
            let mut diff_list = vec![HashSet::<i64>::new(); n];
            for i in 0..n {
                for j in i + 1..n {
                    diff_list[i].insert(xs[j] - xs[i]);
                }
            }
            diff_list
        };

        // k=1 の初期化
        for i in 0..=n {
            for p in 0..i {
                for &d in &diff_list[p] {
                    dp[i][1][xs[p]][d] += 1;
                }
            }
        }

        for i in 1..n {
            for k in 1..n {
                let mut next_dp = DefaultHashMap::<i64, DefaultHashMap<i64, Mint>>::default();
                // 初項
                for &c in dp[i][k].keys() {
                    // 公差
                    for &d in dp[i][k][c].keys() {
                        let term1 = if xs[i] == c + (k as i64) * d {
                            dp[i][k][c][d]
                        } else {
                            Mint::new(0)
                        };
                        let term2 = dp[i][k + 1][c][d];
                        next_dp[c][d] = term1 + term2;
                    }
                }
                dp[i + 1][k + 1] = next_dp;
            }
        }

        let mut ans = vec![];

        ans.push(n as i64); // k = 1の答え
        for k in 2..=n {
            let sub_ans = dp[n][k]
                .values()
                .flat_map(|dp_sub| dp_sub.values())
                .sum::<Mint>()
                .val() as i64;
            ans.push(sub_ans);
        }

        Answer { ans }
    }

    fn solve4(&self) -> Answer {
        // メモ化再帰を使った実装
        // HashMap を使うと 2051 ms で TLE するが、FxHashMap を使うと 827 ms で AC できる。
        // (TL は 2000 ms)
        // solve3 (582 ms) では普通の HashMap を使っているが next dp を使っているため、solve4 (普通の HashMap 使用)よりも早い。
        // solve4 のメモ化再帰では next dp で書けない。
        use ac_library::ModInt998244353 as Mint;

        let n = self.n;
        let xs = &self.xs;

        // 初項と公差のペアのリスト
        let head_diff_list = (0..n)
            .tuple_combinations()
            .map(|(i, j)| {
                let head = xs[i];
                let diff = xs[j] - xs[i];
                (head, diff)
            })
            .unique()
            .collect_vec();

        fn rec(
            xs: &[i64],
            i: usize,
            k: usize,
            h: i64,
            d: i64,
            memo: &mut Vec<Vec<FxHashMap<(i64, i64), Mint>>>,
        ) -> Mint {
            if let Some(&ans) = memo[i][k].get(&(h, d)) {
                return ans;
            };

            let ans = {
                if k == 0 {
                    Mint::new(1)
                } else if i == 1 {
                    if xs[0] == h && k == 1 {
                        Mint::new(1)
                    } else {
                        Mint::new(0)
                    }
                } else {
                    if xs[i - 1] == h + (k as i64 - 1) * d {
                        rec(xs, i - 1, k - 1, h, d, memo) + rec(xs, i - 1, k, h, d, memo)
                    } else {
                        rec(xs, i - 1, k, h, d, memo)
                    }
                }
            };
            memo[i][k].insert((h, d), ans);
            ans
        }
        let mut memo = vec![vec![FxHashMap::default(); n + 1]; n + 1];
        let mut ans = vec![];

        ans.push(n as i64); // k = 1の答え
        for k in 2..=n {
            let sub_ans = head_diff_list
                .iter()
                .copied()
                .map(|(h, d)| rec(xs, n, k, h, d, &mut memo))
                .sum::<Mint>();
            ans.push(sub_ans.val() as i64);
        }
        Answer { ans }
    }

    fn solve5(&self) -> Answer {
        // メモ化再帰を使った実装 (solve4 のリファクタリング)
        // 初項と公差はメモ化しない
        // (solve4 → solve5 で) 877ms が 222 ms になった
        use ac_library::ModInt998244353 as Mint;

        let n = self.n;
        let xs = &self.xs;

        // 初項と公差のペアのリスト
        let head_diff_list = (0..n)
            .tuple_combinations()
            .map(|(i, j)| {
                let head = xs[i];
                let diff = xs[j] - xs[i];
                (head, diff)
            })
            .unique()
            .collect_vec();

        struct Rec<'a> {
            xs: &'a [i64],
            h: i64, // 初項
            d: i64, // 公差
        }

        impl<'a> Rec<'a> {
            fn new(xs: &[i64], h: i64, d: i64) -> Rec {
                Rec { xs, h, d }
            }
            fn rec(&self, i: usize, k: usize, memo: &mut Vec<Vec<Option<Mint>>>) -> Mint {
                if let Some(ans) = memo[i][k] {
                    return ans;
                };

                let ans = {
                    if k == 0 {
                        Mint::new(1)
                    } else if i == 1 {
                        if self.xs[0] == self.h && k == 1 {
                            Mint::new(1)
                        } else {
                            Mint::new(0)
                        }
                    } else {
                        if self.xs[i - 1] == self.h + (k as i64 - 1) * self.d {
                            self.rec(i - 1, k - 1, memo) + self.rec(i - 1, k, memo)
                        } else {
                            self.rec(i - 1, k, memo)
                        }
                    }
                };
                memo[i][k] = Some(ans);
                ans
            }
        }

        let mut ans = vec![Mint::new(0); n];

        ans[0] = Mint::new(n); // k = 1 の答え

        for &(h, d) in &head_diff_list {
            let mut memo = vec![vec![None; n + 1]; n + 1];

            for k in 2..=n {
                let sub_ans = Rec::new(xs, h, d).rec(n, k, &mut memo);
                ans[k - 1] += sub_ans;
            }
        } // k と (h, d) の for ループが逆だと N^5 の計算量になってしまう

        let ans = ans.iter().copied().map(|x| x.val() as i64).collect_vec();
        Answer { ans }
    }

    fn solve6(&self) -> Answer {
        // solve5(メモ化再帰) を forループで書き直し
        // 222ms → 50ms になった
        use ac_library::ModInt998244353 as Mint;

        let n = self.n;
        let xs = &self.xs;

        // 初項と公差のペアのリスト
        let head_diff_list = (0..n)
            .tuple_combinations()
            .map(|(i, j)| {
                let head = xs[i];
                let diff = xs[j] - xs[i];
                (head, diff)
            })
            .unique()
            .collect_vec();

        let mut ans = vec![Mint::new(0); n];

        ans[0] = Mint::new(n); // k = 1 の答え

        for &(h, d) in &head_diff_list {
            let mut dp = vec![vec![Mint::new(0); n + 1]; n + 1];
            for i in 0..=n {
                dp[i][0] = Mint::new(1);
            }
            dp[1][1] = if xs[0] == h {
                Mint::new(1)
            } else {
                Mint::new(0)
            };
            for i in 2..=n {
                for k in 1..=n {
                    let choose = if xs[i - 1] == h + (k as i64 - 1) * d {
                        dp[i - 1][k - 1]
                    } else {
                        Mint::new(0)
                    };
                    let no_choose = dp[i - 1][k];
                    dp[i][k] = choose + no_choose;
                }
            }

            for k in 2..=n {
                ans[k - 1] += dp[n][k];
            }
        }

        let ans = ans.iter().copied().map(|x| x.val() as i64).collect_vec();
        Answer { ans }
    }

    fn solve7(&self) -> Answer {
        use ac_library::ModInt998244353 as Mint;

        let n = self.n;
        let xs = &self.xs;

        if n == 1 {
            return Answer { ans: vec![1] };
        }

        let mut dp = vec![vec![vec![Mint::new(0); n + 1]; n]; n];
        // dp[i][j][l]: 末項を A の j 番目、末項の1つ手前を A の i 番目から取ってきたときの長さ l の等差数列の数

        for i in 0..n {
            for j in i + 1..n {
                dp[i][j][2] = Mint::new(1);
            }
        }

        for l in 2..n {
            for i in 0..n {
                for j in i + 1..n {
                    for k in j + 1..n {
                        if xs[i] + xs[k] == 2 * xs[j] {
                            let addition = dp[i][j][l];
                            dp[j][k][l + 1] += addition;
                        }
                    }
                }
            }
        }
        // l = 1 の答え
        let mut ans = vec![n as i64];
        for l in 2..=n {
            let sub_ans = iproduct!(0..n, 0..n)
                .map(|(i, j)| dp[i][j][l])
                .sum::<Mint>()
                .val() as i64;
            ans.push(sub_ans);
        }
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
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec_1line(&self.ans);
    }
}

fn main() {
    Problem::read().solve7().print();
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
        let n = rng.gen_range(1..=10);
        let xs = (0..n).map(|_| rng.gen_range(1..6)).collect_vec();
        let p = Problem { n, xs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 0;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_entropy();
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
#[allow(unused_imports)]
use proconio::{
    derive_readable, fastout, input,
    marker::{Bytes, Chars, Usize1},
};
use rustc_hash::FxHashMap;
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
use default_hash_map::*;
#[allow(clippy::module_inception)]
pub mod default_hash_map {
    use std::hash::Hash;
    use std::{
        borrow::Borrow,
        collections::{
            hash_map::{Iter, IterMut, Keys, Values, ValuesMut},
            HashMap,
        },
    };
    #[derive(Clone, Debug)]
    pub struct DefaultHashMap<K, V> {
        raw: HashMap<K, V>,
        default: V,
    }
    impl<K, V> DefaultHashMap<K, V> {
        pub fn new(default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: HashMap::new(),
                default,
            }
        }
        pub fn from_hash_map(hash_map: HashMap<K, V>, default: V) -> DefaultHashMap<K, V> {
            DefaultHashMap {
                raw: hash_map,
                default,
            }
        }
        pub fn raw(&mut self) -> &mut HashMap<K, V> {
            &mut self.raw
        }
        pub fn keys(&self) -> Keys<K, V> {
            self.raw.keys()
        }
        pub fn values(&self) -> Values<K, V> {
            self.raw.values()
        }
        pub fn values_mut(&mut self) -> ValuesMut<K, V> {
            self.raw.values_mut()
        }
        pub fn iter(&self) -> Iter<K, V> {
            self.raw.iter()
        }
        pub fn iter_mut(&mut self) -> IterMut<K, V> {
            self.raw.iter_mut()
        }
        pub fn len(&mut self) -> usize {
            self.raw.len()
        }
        pub fn is_empty(&mut self) -> bool {
            self.raw.is_empty()
        }
    }
    impl<K, V> DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        pub fn get<Q: ?Sized>(&self, k: &Q) -> &V
        where
            K: Borrow<Q>,
            Q: Hash + Eq,
        {
            self.raw.get(k).unwrap_or(&self.default)
        }
        pub fn get_mut(&mut self, k: K) -> &mut V
        where
            V: Clone,
        {
            self.raw.entry(k).or_insert(self.default.clone())
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            self.raw.insert(k, v)
        }
        pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq,
        {
            self.raw.remove(k)
        }
    }
    impl<K, V> PartialEq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: PartialEq,
    {
        fn eq(&self, other: &DefaultHashMap<K, V>) -> bool {
            self.raw == other.raw && self.default == other.default
        }
    }
    impl<K, V> Eq for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Eq,
    {
    }
    impl<K, V> Default for DefaultHashMap<K, V>
    where
        V: Default,
    {
        fn default() -> DefaultHashMap<K, V> {
            DefaultHashMap::new(V::default())
        }
    }
    impl<K, V> std::ops::Index<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
    {
        type Output = V;
        #[inline]
        fn index(&self, key: K) -> &V {
            self.get(&key)
        }
    }
    impl<K, V> std::ops::IndexMut<K> for DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        V: Clone,
    {
        #[inline]
        fn index_mut(&mut self, key: K) -> &mut V {
            self.get_mut(key)
        }
    }
}
