// 問題文と制約は読みましたか？
#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct CRange {
    left: Usize1,
    right: Usize1,
}
#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        clothes: [CRange; m],
        q: usize,
        qs: [CRange; q]
    }
    let clothes = clothes
        .iter()
        .copied()
        .sorted_by_key(|r| (r.right, Reverse(r.left)))
        .collect_vec();

    let mut right_to_lefts = HashMap::<usize, BTreeSet<usize>>::new();
    let mut left_to_rights = HashMap::<usize, BTreeSet<usize>>::new();
    for &c in &clothes {
        right_to_lefts.entry(c.right).or_default().insert(c.left);
        left_to_rights.entry(c.left).or_default().insert(c.right);
    }

    let clothes_to_index = clothes
        .iter()
        .copied()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<CRange, usize>>();

    // cloths について
    let has_subset = {
        let mut has_subset = vec![false; m];

        let mut set: BTreeSet<usize> = BTreeSet::new();
        for j in 0..m {
            let cloth = clothes[j];
            has_subset[j] = set.range(cloth.left..=cloth.right).next().is_some();
            set.insert(cloth.left);
        }
        has_subset
    };

    let ans = qs
        .iter()
        .copied()
        .map(|q| {
            // パターン1
            let pattern1 = {
                //
                if let Some(&cloth_idx) = clothes_to_index.get(&q) {
                    has_subset[cloth_idx]
                } else {
                    false
                }
            };

            // パターン2
            let pattern2 = {
                // 右→左
                let left = right_to_lefts
                    .get(&q.right)
                    .and_then(|set| set.range(q.left..).min());

                // 左→右
                let right = left_to_rights
                    .get(&q.left)
                    .and_then(|set| set.range(..=q.right).max());

                // dbg!(left);
                // dbg!(right);

                if let (Some(&left), Some(&right)) = (left, right) {
                    left != q.left && (left as i64) - (right as i64) <= 1
                } else {
                    false
                }
            };
            // dbg!(pattern1);
            // dbg!(pattern2);
            pattern1 || pattern2
        })
        .collect_vec();

    for p in ans {
        println!("{}", if p { "Yes" } else { "No" });
    }
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

    /// 間違っていたら false を返す
    fn process_one_test(rng: &mut SmallRng) -> bool {
        // ==== 問題を作る ====
        let n = rng.random_range(1..=10);
        let xs = (0..n).map(|_| rng.random_range(0..10)).collect_vec();

        // ==== 解く ====
        let main_ans = xs.len();
        let naive_ans = 1;

        // ==== 間違っていたら報告をする ====
        if main_ans != naive_ans {
            // 問題を出力
            println!("{:?}", (n, xs));
            println!("main ans : {:?}", main_ans);
            println!("naive ans: {:?}", naive_ans);
            return false;
        }
        true
    }

    #[allow(unreachable_code)]
    #[test]
    #[ignore]
    fn test_with_naive() {
        let num_tests = 100;
        let max_wrong_case = 10; // この件数間違いが見つかったら打ち切り
        let mut cnt_wrong = 0;
        let mut rng = SmallRng::seed_from_u64(42);
        // let mut rng = SmallRng::from_os_rng();
        for _ in 0..num_tests {
            let is_ok = process_one_test(&mut rng);
            if !is_ok {
                cnt_wrong += 1;
            }
            if cnt_wrong >= max_wrong_case {
                break;
            }
        }
        if cnt_wrong > 0 {
            println!("{} cases are wrong.", cnt_wrong);
            panic!();
        }
    }
}

// ====== import ======
#[allow(unused_imports)]
use {
    itertools::{Itertools, chain, iproduct, izip},
    proconio::{
        derive_readable, fastout, input,
        marker::{Bytes, Chars, Usize1},
    },
    rand::{Rng, SeedableRng, rngs::SmallRng, seq::SliceRandom},
    std::{
        cmp::Reverse,
        collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
    },
};

// ====== output func ======
#[allow(unused_imports)]
use print_util::*;
pub mod print_util {
    use itertools::Itertools;
    use proconio::fastout;

    #[fastout]
    pub fn print_vec<T: std::fmt::Display>(arr: &[T]) {
        for a in arr {
            println!("{}", a);
        }
    }

    #[fastout]
    pub fn print_vec_1line<T: std::fmt::Display>(arr: &[T]) {
        println!("{}", arr.iter().join(" "));
    }

    #[fastout]
    pub fn print_vec2<T: std::fmt::Display, R: AsRef<[T]>>(arr: &[R]) {
        for row in arr {
            println!("{}", row.as_ref().iter().join(" "));
        }
    }

    pub fn print_bytes(bytes: &[u8]) {
        println!("{}", std::str::from_utf8(bytes).unwrap());
    }

    pub fn print_chars(chars: &[char]) {
        println!("{}", chars.iter().collect::<String>());
    }

    #[fastout]
    pub fn print_vec_bytes<R: AsRef<[u8]>>(vec_bytes: &[R]) {
        for row in vec_bytes {
            println!("{}", std::str::from_utf8(row.as_ref()).unwrap());
        }
    }

    #[fastout]
    pub fn print_vec_chars<R: AsRef<[char]>>(vec_chars: &[R]) {
        for row in vec_chars {
            println!("{}", row.as_ref().iter().collect::<String>());
        }
    }

    pub fn print_yesno(ans: bool) {
        println!("{}", if ans { "Yes" } else { "No" });
    }
}

// ====== snippet ======
