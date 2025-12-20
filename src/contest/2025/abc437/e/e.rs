// #[fastout]
fn dfs(
    index_set: &Vec<HashMap<i64, Vec<usize>>>,
    children_set: &Vec<HashMap<i64, BTreeSet<i64>>>,
    levels: &Vec<usize>,
    level: usize,
    val: i64,
    acc: &mut Vec<usize>,
) {
    // dbg!(level, val);
    if index_set[level].contains_key(&val) {
        for &idx in &index_set[level][&val] {
            acc.push(idx);
            //
        }
    }
    if children_set[level].contains_key(&val) {
        for &child in &children_set[level][&val] {
            dfs(index_set, children_set, levels, level + 1, child, acc);
        }
    }
    // for child in children_set[level].entry(val).or_default() {}

    //
}

fn main() {
    input! {
        n: usize,
        mut xys: [(usize, i64); n],
    }

    xys.insert(0, (0, 0));

    // let mut tree: Vec<HashMap<i64, (Vec<usize>, Vec<i64>)>> = vec![];
    // index_set[l][t] = l階層目で追加されたyの値が t となるような添字の集合
    let mut index_set: Vec<HashMap<i64, Vec<usize>>> = vec![];
    // children_set[l][t] = l階層目で追加されたyの値が t となるような子どものyの値の集合
    let mut children_set: Vec<HashMap<i64, BTreeSet<i64>>> = vec![];

    // levels[i]: A[i] がいる階層
    let mut levels: Vec<usize> = vec![usize::MAX; n + 1];

    index_set.push(HashMap::new());
    children_set.push(HashMap::new());
    index_set[0].insert(0, vec![0]); // key はダミー値
    levels[0] = 0;

    for (i, (x, y)) in xys.iter().copied().enumerate() {
        if i == 0 {
            continue;
        }
        levels[i] = levels[x] + 1;
        if levels[i] >= index_set.len() {
            index_set.push(HashMap::new());
        }
        if levels[i] >= children_set.len() {
            children_set.push(HashMap::new());
        }

        //children_set[levels[x]].insert(xys[x], i);
        children_set[levels[x]]
            .entry(xys[x].1)
            .or_default()
            .insert(y);
        index_set[levels[i]].entry(y).or_default().push(i);
    }
    let mut ans = vec![];

    dfs(&index_set, &children_set, &levels, 0, 0, &mut ans);

    // dbg!(ans);
    // dbg!(levels);
    // dbg!(children_set);
    // dbg!(index_set);

    print_vec(&ans[1..]);
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
use std::collections::BTreeSet;
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
        collections::{BinaryHeap, HashMap, HashSet},
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
        let msg = arr.iter().map(|x| format!("{}", x)).join(" ");
        println!("{}", msg);
    }
    #[fastout]
    pub fn print_vec2<T: std::fmt::Display>(arr: &Vec<Vec<T>>) {
        for row in arr {
            let msg = row.iter().map(|x| format!("{}", x)).join(" ");
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
    #[fastout]
    pub fn print_vec_chars(vec_chars: &[Vec<char>]) {
        for row in vec_chars {
            let msg = row.iter().collect::<String>();
            println!("{}", msg);
        }
    }
    pub fn print_yesno(ans: bool) {
        let msg = if ans { "Yes" } else { "No" };
        println!("{}", msg);
    }
}

// ====== snippet ======
