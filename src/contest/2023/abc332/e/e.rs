//#[derive_readable]
#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    n_balls: usize,
    n_boxes: usize,
    weight_list: Vec<i64>,
}

fn mean(xs: &[i64]) -> f64 {
    xs.iter().sum::<i64>() as f64 / xs.len() as f64
}

fn variance(xs: &[i64]) -> f64 {
    let m = mean(xs);
    let sq = |x: f64| x * x;
    xs.iter().map(|&x| sq(x as f64 - m)).sum::<f64>() / xs.len() as f64
}

fn sum_sq(xs: &[i64]) -> i64 {
    xs.iter().map(|&x| x * x).sum()
}

impl Problem {
    fn read() -> Problem {
        input! {
            n_balls: usize,
            n_boxes: usize,
            weight_list: [i64; n_balls],

        }
        Problem {
            n_balls,
            n_boxes,
            weight_list,
        }
    }
    fn solve(&self) -> Answer {
        // 愚直実装(TLE)
        // 商品は玉、袋は箱として扱う
        let n_balls = self.n_balls;
        let n_boxes = self.n_boxes;
        let weight_list = &self.weight_list;

        let ans = (0..n_balls)
            .combinations(n_boxes) // 箱には1つ以上の玉が入っていることが条件→最初からそれぞれの箱に玉1を入れてしまう。最初に箱に入れる玉を選ぶ。
            .map(|init_balls| {
                let init_balls_set = init_balls.iter().copied().collect::<HashSet<_>>();
                let other_balls = (0..n_balls)
                    .filter(|ball| !init_balls_set.contains(ball))
                    .collect_vec();
                let mut boxes = init_balls
                    .iter()
                    .copied()
                    .map(|ball| vec![ball])
                    .collect_vec();

                struct Req<'a> {
                    other_balls: &'a [usize],
                    weight_list: &'a [i64],
                }

                impl<'a> Req<'a> {
                    fn new(other_balls: &'a [usize], weight_list: &'a [i64]) -> Self {
                        Self {
                            other_balls,
                            weight_list,
                        }
                    }

                    fn calc_variance(&self, boxes: &[Vec<usize>]) -> f64 {
                        let weight_by_box = boxes
                            .iter()
                            .map(|balls_in_box| {
                                balls_in_box
                                    .iter()
                                    .copied()
                                    .map(|i| self.weight_list[i])
                                    .sum::<i64>()
                            })
                            .collect_vec();

                        variance(&weight_by_box)
                    }
                    fn req(
                        &self,
                        cnt_balls: usize,
                        boxes: &mut Vec<Vec<usize>>,
                        acc_min_variance: &mut f64,
                    ) {
                        if cnt_balls == self.other_balls.len() {
                            // 計算をする
                            let variance = self.calc_variance(boxes);
                            *acc_min_variance = acc_min_variance.min(variance);
                            return;
                        }

                        let ball = self.other_balls[cnt_balls];
                        for box_i in 0..boxes.len() {
                            if ball > boxes[box_i].last().copied().unwrap() {
                                boxes[box_i].push(ball);
                                self.req(cnt_balls + 1, boxes, acc_min_variance);
                                boxes[box_i].pop();
                            }
                        }
                    }
                }

                let mut acc_min_variance = f64::MAX;
                Req::new(&other_balls, weight_list).req(0, &mut boxes, &mut acc_min_variance);

                acc_min_variance
            })
            .min_by(f64::total_cmp)
            .unwrap();
        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        // スターリング数の数だけ全探索（枝刈りあり）
        // 商品は玉、袋は箱として扱う

        let n_balls = self.n_balls;
        let n_boxes = self.n_boxes;
        let weight_list = &self.weight_list;

        let sum_sq = (0..n_balls)
            .combinations(n_boxes)
            .map(|init_balls| {
                let init_balls_set = init_balls.iter().copied().collect::<HashSet<_>>();
                let other_balls = (0..n_balls)
                    .filter(|ball| !init_balls_set.contains(ball))
                    .collect_vec();
                let mut boxes = init_balls
                    .iter()
                    .copied()
                    .map(|ball| vec![ball])
                    .collect_vec();

                struct Req<'a> {
                    other_balls: &'a [usize],
                    weight_list: &'a [i64],
                }

                impl<'a> Req<'a> {
                    fn new(other_balls: &'a [usize], weight_list: &'a [i64]) -> Self {
                        Self {
                            other_balls,
                            weight_list,
                        }
                    }

                    fn req(
                        &self,
                        cnt_balls: usize,
                        boxes: &mut Vec<Vec<usize>>,
                        sum_weight_sq: i64,        // 各バッグの重みの2乗の総和
                        weight_by_box: &mut [i64], // 各バッグの重み
                        acc_min_sum_sq: &mut i64,
                    ) {
                        if cnt_balls == self.other_balls.len() {
                            // 計算をする
                            *acc_min_sum_sq = (*acc_min_sum_sq).min(sum_weight_sq);
                            return;
                        }

                        if *acc_min_sum_sq <= sum_weight_sq {
                            return;
                        }

                        let ball = self.other_balls[cnt_balls];
                        for box_i in 0..boxes.len() {
                            if ball > boxes[box_i].last().copied().unwrap() {
                                boxes[box_i].push(ball);

                                let current_weight = weight_by_box[box_i];

                                let addition_weight_sq =
                                    2 * weight_by_box[box_i] * self.weight_list[ball]
                                        + self.weight_list[ball] * self.weight_list[ball];
                                let next_sum_weight_sq = sum_weight_sq + addition_weight_sq;

                                weight_by_box[box_i] += self.weight_list[ball];

                                self.req(
                                    cnt_balls + 1,
                                    boxes,
                                    next_sum_weight_sq,
                                    weight_by_box,
                                    acc_min_sum_sq,
                                );
                                weight_by_box[box_i] = current_weight;
                                boxes[box_i].pop();
                            }
                        }
                    }
                }

                let mut weight_by_box = boxes
                    .iter()
                    .map(|balls| balls.iter().map(|ball| weight_list[*ball]).sum::<i64>())
                    .collect_vec();
                let weight_sq_by_box = weight_by_box.iter().copied().map(|w| w * w).collect_vec();
                let sum_weight_sq = weight_sq_by_box.iter().sum();
                let mut acc_min_sum_eq = i64::MAX;

                Req::new(&other_balls, weight_list).req(
                    0,
                    &mut boxes,
                    sum_weight_sq,
                    &mut weight_by_box,
                    &mut acc_min_sum_eq,
                );

                acc_min_sum_eq
            })
            .min()
            .unwrap();
        // これだとans の引き算で誤差が発生してしまう
        // let weight_mean = (weight_list.iter().sum::<i64>() as f64) / (n_boxes as f64);
        // let ans = (sum_sq as f64 / n_boxes as f64) - (weight_mean * weight_mean);
        /*
        ```
        4
        2
        100000000 100000000 2 1
        ```
        こういう入力で0を返してしまう（答えは0.25）
         */

        let weight_sum = weight_list.iter().sum::<i64>();

        let n_boxes_i64 = n_boxes as i64;
        let n_boxes_f64 = n_boxes as f64;
        let ans =
            (n_boxes_i64 * sum_sq - weight_sum * weight_sum) as f64 / (n_boxes_f64 * n_boxes_f64);

        Answer { ans }
    }

    fn solve3(&self) -> Answer {
        let n_balls = self.n_balls;
        let n_boxes = self.n_boxes;
        let weight_list = &self.weight_list;

        let sum_sq = {
            let mut dp = vec![vec![0; 1 << n_balls]; n_boxes + 1]; // 重みの2乗の総和

            // DPの定義
            // dp[k][S]: 箱がk個あるとき、集合Sに入っているボールをk個の箱に入れたときの各箱の重みの2乗の総和の最小値
            // 初期値
            // dp[1][S] = Sに入っているボールの重さの総和の2乗
            // 遷移
            // dp[k+1][S] = min_{A ⊆ S} (dp[k][S-A] + dp[1][A])
            // 答え
            // dp[n_boxes][1<<n_balls-1]

            for s in 0..(1 << n_balls) {
                let weight_sum = (0..n_balls)
                    .filter(|&i| ((s >> i) & 1) == 1)
                    .map(|i| self.weight_list[i])
                    .sum::<i64>();
                dp[1][s] = weight_sum * weight_sum;
            }

            for k in 1..n_boxes {
                for s in 0..(1 << n_balls) {
                    // 箱を1個追加したとき、その追加した箱に入れるボールの集合(sの部分集合)を全探索して最小のものを計算
                    dp[k + 1][s] = std::iter::successors(Some(s), |x| {
                        if *x == 0 {
                            None
                        } else {
                            Some((x - 1) & s)
                        }
                    })
                    .map(|t| dp[k][s & (!t)] + dp[1][t])
                    .min()
                    .unwrap();
                }
            }
            dp[n_boxes][(1 << n_balls) - 1]
        };

        let weight_sum = weight_list.iter().sum::<i64>();

        let n_boxes_i64 = n_boxes as i64;
        let n_boxes_f64 = n_boxes as f64;
        let ans =
            (n_boxes_i64 * sum_sq - weight_sum * weight_sum) as f64 / (n_boxes_f64 * n_boxes_f64);

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Answer {
    ans: f64,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
}
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use num_integer::Roots;
    use num_traits::Pow;
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
        let main_ans = p.solve2();
        let naive_ans = p.solve();
        let diff_abs = f64::abs(main_ans.ans - naive_ans.ans);
        let diff_rel = f64::abs(main_ans.ans - naive_ans.ans) / f64::abs(naive_ans.ans);
        let eps = 1e-6;
        if diff_abs > eps && diff_rel > eps {
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
        let n_boxes = rng.gen_range(2..=6);
        let n_balls = rng.gen_range(n_boxes..=6);
        let weight_list = (0..n_balls)
            .map(|_| 2_i64.pow(rng.gen_range(0..30) as u32))
            .collect_vec();
        let p = Problem {
            n_balls,
            n_boxes,
            weight_list,
        };
        // println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 100000;
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

use std::collections::HashSet;

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
