#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    y1: i64,
    x1: i64,
    y2: i64,
    x2: i64,
}

struct Problem {
    n: usize,
    n_q: usize,
    grid: Vec<Vec<u8>>,
    qs: Vec<Query>,
}

impl Problem {
    fn read() -> Problem {
        input! {
            n: usize,
            n_q: usize,
            grid: [Bytes; n],
            qs: [Query; n_q],
        }
        Problem { n, n_q, grid, qs }
    }
    fn solve(&self) -> Answer {
        let Problem { n, n_q, grid, qs } = self;
        let grid = grid
            .iter()
            .map(|row| row.iter().copied().map(|ch| (ch == b'B') as i64).collect_vec())
            .collect_vec();

        let grid22 =
            (0..2 * n).map(|y| (0..2 * n).map(|x| grid[y % n][x % n]).collect_vec()).collect_vec();

        let cumsum = CumSum2D::new(&grid22);

        let ans = qs
            .iter()
            .copied()
            .map(|q| {
                // 命名がうまく言ってない感じがする。命名がうまく行ったらなぁ。
                let (y1, x1, y2, x2) = (q.y1, q.x1, q.y2, q.x2);
                // (y1, x1) が左上のブロックに来るように平行移動する
                let n = *n as i64;
                let diff_y = y1 - y1 % n;
                let diff_x = x1 - x1 % n;

                let (y1, x1, y2, x2) = (y1 - diff_y, x1 - diff_x, y2 - diff_y, x2 - diff_x);

                let by1 = y1 / n; // 0 になるはず
                let bx1 = x1 / n; // 0 になるはず
                let by2 = y2 / n;
                let bx2 = x2 / n;

                // 真ん中のブロックを一旦消す
                let y2_prime = by2.min(1) * n + y2 % n;
                let x2_prime = bx2.min(1) * n + x2 % n;

                // 縮小した長方形で黒マスを数える
                let sum1 = cumsum.get_rect_sum(
                    (x1 as usize, y1 as usize),
                    (x2_prime as usize + 1, y2_prime as usize + 1),
                );

                let sum2 = if bx2 < 2 {
                    0
                } else {
                    cumsum.get_rect_sum((0, y1 as usize), (n as usize, y2_prime as usize + 1))
                        * (bx2 - bx2.min(1))
                };

                let sum3 = if by2 < 2 {
                    0
                } else {
                    cumsum.get_rect_sum((x1 as usize, 0), (x2_prime as usize + 1, n as usize))
                        * (by2 - by2.min(1))
                };

                let black_cnt_in_square = cumsum.get_rect_sum((0, 0), (n as usize, n as usize));

                // 消した正方形何個ある？
                let deleted_square_cnt = (by2 - by2.min(1)) * (bx2 - bx2.min(1));

                sum1 + black_cnt_in_square * deleted_square_cnt + sum2 + sum3
            })
            .collect_vec();

        Answer { ans }
    }

    fn solve2(&self) -> Answer {
        let Problem { n, n_q, grid, qs } = self;
        let grid = grid
            .iter()
            .map(|row| row.iter().copied().map(|ch| (ch == b'B') as i64).collect_vec())
            .collect_vec();

        struct QuerySolver {
            cumsum: CumSum2D,
            sq_size: i64,
        }
        impl QuerySolver {
            fn new(grid: &Vec<Vec<i64>>, sq_size: i64) -> Self {
                let cumsum = CumSum2D::new(grid);
                Self { cumsum, sq_size }
            }

            fn prefix_sum(&self, y: i64, x: i64) -> i64 {
                //  [0, x) × [0, y) で数える
                let cnt_sq_x = x / self.sq_size;
                let cnt_sq_y = y / self.sq_size;
                let remain_x = (x % self.sq_size) as usize;
                let remain_y = (y % self.sq_size) as usize;
                let sq_size_usize = self.sq_size as usize;
                let cnt_in_sq = self.cumsum.get_rect_sum((0, 0), (sq_size_usize, sq_size_usize));

                let sum1 = cnt_sq_x * cnt_sq_y * cnt_in_sq;
                let sum2 = self.cumsum.get_rect_sum((0, 0), (sq_size_usize, remain_y)) * cnt_sq_x;
                let sum3 = self.cumsum.get_rect_sum((0, 0), (remain_x, sq_size_usize)) * cnt_sq_y;
                let sum4 = self.cumsum.get_rect_sum((0, 0), (remain_x, remain_y));
                sum1 + sum2 + sum3 + sum4
            }

            fn solve(&self, q: Query) -> i64 {
                let Query { y1, x1, y2, x2 } = q;
                self.prefix_sum(y2 + 1, x2 + 1)
                    - self.prefix_sum(y1, x2 + 1)
                    - self.prefix_sum(y2 + 1, x1)
                    + self.prefix_sum(y1, x1)
            }
        }

        let solver = QuerySolver::new(&grid, *n as i64);

        let ans = qs.iter().copied().map(|q| solver.solve(q)).collect_vec();

        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: Vec<i64>,
}

impl Answer {
    fn print(&self) {
        print_vec(&self.ans);
    }
}

fn main() {
    Problem::read().solve2().print();
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_problem() {
        assert_eq!(1 + 1, 2);
    }
}

// ====== import ======
use itertools::iproduct;
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

use cumsum_2d::*;
pub mod cumsum_2d {
    pub struct CumSum2D {
        pub cumsum: Vec<Vec<i64>>,
    }
    impl CumSum2D {
        pub fn new(xss: &Vec<Vec<i64>>) -> CumSum2D {
            if xss.is_empty() {
                return CumSum2D { cumsum: vec![vec![0]] };
            }
            let height = xss.len();
            let width = xss[0].len();
            let mut cumsum = vec![vec![0; width + 1]; height + 1];
            for y in 1..height + 1 {
                for x in 1..width + 1 {
                    cumsum[y][x] = cumsum[y - 1][x] + cumsum[y][x - 1] - cumsum[y - 1][x - 1]
                        + xss[y - 1][x - 1];
                }
            }
            CumSum2D { cumsum }
        }
        pub fn get_rect_sum(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i64 {
            self.cumsum[y2][x2] - self.cumsum[y2][x1] - self.cumsum[y1][x2] + self.cumsum[y1][x1]
        }
    }
}
