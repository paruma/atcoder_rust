// #[fastout]
pub fn divide_conquer_convolution<M>(ps: &[Vec<StaticModInt<M>>]) -> Vec<StaticModInt<M>>
where
    M: Modulus,
{
    let len = ps.len();
    if len == 1 {
        ps[0].clone()
    } else {
        ac_library::convolution(
            &divide_conquer_convolution(&ps[..len / 2]),
            &divide_conquer_convolution(&ps[len / 2..]),
        )
    }
}
fn main() {
    input! {
        n: usize,
        m: usize,
        xs: [i64; n],
    }
    use ac_library::ModInt998244353 as Mint;

    let polys = {
        let mut polys = vec![vec![Mint::new(1), Mint::new(-1)]];

        for &x in &xs {
            let mut ys = vec![Mint::new(0); (x + 1) as usize];
            ys[0] = Mint::new(1);
            ys[x as usize] = Mint::new(-1);
            polys.push(ys);
        }
        polys
    };

    // let acc = polys.iter().fold(vec![Mint::new(1)], |acc, p| {
    //     ac_library::convolution(&acc, p)
    // });

    // メモ: 普通に畳み込みしても分割統治で畳み込みしても実行時間はあんまり変わらなかった
    let acc = divide_conquer_convolution(&polys);

    // acc = ac_library::convolution::convolution(&acc, &ys);

    let mut init = vec![Mint::new(0); acc.len()];
    init[0] = Mint::new(1);

    let ans = bostan_mori(init, acc, (m) as u64);
    println!("{}", ans);
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
use ac_library::{Modulus, StaticModInt, convolution};

/// Bostan-Mori法
///
/// 有理関数 P(x)/Q(x) の N 次の係数を求める
///
/// # 引数
///
/// * `p` - 分子多項式 P(x) の係数ベクトル
/// * `q` - 分母多項式 Q(x) の係数ベクトル
/// * `n` - 求める係数の次数
///
/// # 戻り値
///
/// `[x^n] P(x)/Q(x)`
///
/// # 計算量
///
/// O(K log K log N), K は Q(x) の次数
pub fn bostan_mori<M: Modulus>(
    mut p: Vec<StaticModInt<M>>,
    mut q: Vec<StaticModInt<M>>,
    mut n: u64,
) -> StaticModInt<M> {
    while n > 0 {
        let k = q.len() - 1;
        if k == 0 {
            return if p.is_empty() {
                StaticModInt::new(0)
            } else {
                p[0] * q[0].inv()
            };
        }

        // Q(-x) を計算
        let mut q_neg_x = q.clone();
        for i in (1..q.len()).step_by(2) {
            q_neg_x[i] = -q_neg_x[i];
        }

        // P(x) * Q(-x) と Q(x) * Q(-x) を計算
        p = convolution(&p, &q_neg_x);
        q = convolution(&q, &q_neg_x);

        // P と Q の偶数次または奇数次の項を取り出す
        let mut p_new = Vec::new();
        if n % 2 == 1 {
            for i in (1..p.len()).step_by(2) {
                p_new.push(p[i]);
            }
        } else {
            for i in (0..p.len()).step_by(2) {
                p_new.push(p[i]);
            }
        }
        p = p_new;

        let mut q_new = Vec::new();
        for i in (0..q.len()).step_by(2) {
            q_new.push(q[i]);
        }
        q = q_new;

        n /= 2;
    }

    if p.is_empty() {
        StaticModInt::new(0)
    } else {
        p[0] * q[0].inv()
    }
}
