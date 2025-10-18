/*
ABC370 D - Cross Explosion

solve メソッドがメイン実装の部分なので、solve メソッドを参照してください。

■ 解法1 (GridSolve1 を使った場合)
各点に対して、その点の上下左右にある壁の位置を保持する
（range update をしたいので、range update 遅延セグ木を持つ）

■ 解法2 (GridSolve2 を使った場合)
壁を壊した区間を Union Find で管理する
壁を壊したら両隣の区間をくっつける

■ 解法3 (GridSolve3 を使った場合)
各行・各列の壁の位置を BTreeSet で管理する

■ 解法4 (GridSolve4 を使った場合)
各行・各列の壁の位置を range sum セグ木で管理する(壁あり: 1 壁なし: 0)
右側にある一番近い壁などはセグ木の二分探索を使う

■ 解法5 (GridSolve5 を使った場合)
各行・各列の壁の位置を range min セグ木や range max セグ木で管理する
(壁あり: 座標の値, 壁なし: +∞ または -∞)
range max や range min をすると、壁の位置が得られる。

■ 解法6 (GridSolve6 を使った場合)
区間を sorted set で持つテクニック
*/

#[derive_readable]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Query {
    y: Usize1,
    x: Usize1,
}
#[derive(Debug, Clone)]
struct Problem {
    h: usize,
    w: usize,
    nq: usize,
    qs: Vec<Query>,
}

// 各点の上下左右の壁の位置を range update 遅延セグ木で持つ
// 壁を壊したら、range update で壁の位置を更新する
struct GridSolve1 {
    wall: Vec<Vec<bool>>,
    right_wall: Vec<RangeAffineRangeSumSegtree<i64>>,
    left_wall: Vec<RangeAffineRangeSumSegtree<i64>>,
    down_wall: Vec<RangeAffineRangeSumSegtree<i64>>,
    up_wall: Vec<RangeAffineRangeSumSegtree<i64>>,
    h: usize,
    w: usize,
}

impl GridSolve1 {
    fn new(h: usize, w: usize) -> Self {
        let wall = vec![vec![true; w]; h];
        let right_wall: Vec<RangeAffineRangeSumSegtree<i64>> = (0..h)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..w).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let left_wall = (0..h)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..w).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let up_wall = (0..w)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..h).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let down_wall = (0..w)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..h).map(|i| i as i64).collect_vec()))
            .collect_vec();

        GridSolve1 {
            wall,
            right_wall,
            left_wall,
            down_wall,
            up_wall,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.wall[y][x] {
            return;
        }
        self.wall[y][x] = false;
        {
            let next_left_wall = if x == 0 {
                -1
            } else {
                self.left_wall[y].get(x - 1)
            };
            let next_right_wall = if x == self.w - 1 {
                self.w as i64
            } else {
                self.right_wall[y].get(x + 1)
            };

            let range = (next_left_wall + 1) as usize..=(next_right_wall - 1) as usize;
            self.left_wall[y].apply_range_update(range.clone(), next_left_wall);
            self.right_wall[y].apply_range_update(range, next_right_wall);
        }
        {
            let next_up_wall = if y == 0 {
                -1
            } else {
                self.up_wall[x].get(y - 1)
            };
            let next_down_wall = if y == self.h - 1 {
                self.h as i64
            } else {
                self.down_wall[x].get(y + 1)
            };

            let range = (next_up_wall + 1) as usize..=(next_down_wall - 1) as usize;

            self.up_wall[x].apply_range_update(range.clone(), next_up_wall);
            self.down_wall[x].apply_range_update(range, next_down_wall);
        }
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.left_wall[y].get(x);
        if p >= 0 {
            Some(p as usize)
        } else {
            None
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.right_wall[y].get(x);
        if p < self.w as i64 {
            Some(p as usize)
        } else {
            None
        }
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.up_wall[x].get(y);
        if p >= 0 {
            Some(p as usize)
        } else {
            None
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.down_wall[x].get(y);
        if p < self.h as i64 {
            Some(p as usize)
        } else {
            None
        }
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.wall[y][x]
    }

    fn count_wall(&self) -> usize {
        self.wall.iter().flatten().filter(|x| **x).count()
    }
}

use ac_library::{segtree::Monoid, Additive, Max, Min, Segtree};
use std::{collections::BTreeSet, convert::Infallible, i64};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangeMinMax {
    pub min: i64,
    pub max: i64,
}
impl RangeMinMax {
    pub fn unit(x: i64) -> Self {
        Self { min: x, max: x }
    }
}
pub struct RangeMinMaxMonoid(Infallible);
impl Monoid for RangeMinMaxMonoid {
    type S = RangeMinMax;
    fn identity() -> Self::S {
        RangeMinMax {
            min: i64::MAX,
            max: i64::MIN,
        }
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        RangeMinMax {
            min: i64::min(a.min, b.min),
            max: i64::max(a.max, b.max),
        }
    }
}

// 壁が壊れた区間を Union Find で持つ
struct GridSolve2 {
    wall: Vec<Vec<bool>>,
    row_non_wall_range: Vec<MonoidUnionFind<RangeMinMaxMonoid>>,
    col_non_wall_range: Vec<MonoidUnionFind<RangeMinMaxMonoid>>,
    h: usize,
    w: usize,
}

impl GridSolve2 {
    fn new(h: usize, w: usize) -> Self {
        let wall = vec![vec![true; w]; h];
        let row_non_wall_range = (0..h)
            .map(|_| {
                MonoidUnionFind::new(&(0..w).map(|i| RangeMinMax::unit(i as i64)).collect_vec())
            })
            .collect_vec();

        let col_non_wall_range = (0..w)
            .map(|_| {
                MonoidUnionFind::new(&(0..h).map(|i| RangeMinMax::unit(i as i64)).collect_vec())
            })
            .collect_vec();

        GridSolve2 {
            wall,
            row_non_wall_range,
            col_non_wall_range,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.wall[y][x] {
            return;
        }
        self.wall[y][x] = false;
        // 左右をつなげる
        if x < self.w - 1 && !self.wall[y][x + 1] {
            self.row_non_wall_range[y].unite(x, x + 1);
        }
        if x > 0 && !self.wall[y][x - 1] {
            self.row_non_wall_range[y].unite(x, x - 1);
        }

        // 上下をつなげる
        if y < self.h - 1 && !self.wall[y + 1][x] {
            self.col_non_wall_range[x].unite(y, y + 1);
        }
        if y > 0 && !self.wall[y - 1][x] {
            self.col_non_wall_range[x].unite(y, y - 1);
        }
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.row_non_wall_range[y].same_prod(x).min - 1;
        if p >= 0 {
            Some(p as usize)
        } else {
            None
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.row_non_wall_range[y].same_prod(x).max + 1;
        if p < self.w as i64 {
            Some(p as usize)
        } else {
            None
        }
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.col_non_wall_range[x].same_prod(y).min - 1;
        if p >= 0 {
            Some(p as usize)
        } else {
            None
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.col_non_wall_range[x].same_prod(y).max + 1;
        if p < self.h as i64 {
            Some(p as usize)
        } else {
            None
        }
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.wall[y][x]
    }

    fn count_wall(&self) -> usize {
        self.wall.iter().flatten().filter(|x| **x).count()
    }
}

// 壁を BTreeSet で持つ
struct GridSolve3 {
    wall: Vec<Vec<bool>>,
    row_wall: Vec<BTreeSet<usize>>,
    col_wall: Vec<BTreeSet<usize>>,
    h: usize,
    w: usize,
}
impl GridSolve3 {
    fn new(h: usize, w: usize) -> Self {
        let wall = vec![vec![true; w]; h];
        let row_wall = (0..h)
            .map(|_| (0..w).collect::<BTreeSet<_>>())
            .collect_vec();

        let col_wall = (0..w)
            .map(|_| (0..h).collect::<BTreeSet<_>>())
            .collect_vec();

        GridSolve3 {
            wall,
            row_wall,
            col_wall,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.wall[y][x] {
            return;
        }
        self.wall[y][x] = false;
        self.row_wall[y].remove(&x);
        self.col_wall[x].remove(&y);
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        if x == 0 {
            None
        } else {
            self.row_wall[y].range(..=x - 1).max().copied()
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        self.row_wall[y].range(x + 1..).min().copied()
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        if y == 0 {
            None
        } else {
            self.col_wall[x].range(..=y - 1).max().copied()
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        self.col_wall[x].range(y + 1..).min().copied()
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.wall[y][x]
    }

    fn count_wall(&self) -> usize {
        self.wall.iter().flatten().filter(|x| **x).count()
    }
}

// 壁を range sum セグ木で持つ（壁あり: 1, 壁なし: 0）
struct GridSolve4 {
    wall: Vec<Vec<bool>>,
    row_wall: Vec<Segtree<Additive<i64>>>,
    col_wall: Vec<Segtree<Additive<i64>>>,
    h: usize,
    w: usize,
}
impl GridSolve4 {
    fn new(h: usize, w: usize) -> Self {
        let wall = vec![vec![true; w]; h];

        let row_wall = (0..h)
            .map(|_| Segtree::from(std::iter::repeat(1).take(w).collect_vec()))
            .collect_vec();

        let col_wall = (0..w)
            .map(|_| Segtree::from(std::iter::repeat(1).take(h).collect_vec()))
            .collect_vec();

        GridSolve4 {
            wall,
            row_wall,
            col_wall,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.wall[y][x] {
            return;
        }
        self.wall[y][x] = false;
        self.row_wall[y].set(x, 0);
        self.col_wall[x].set(y, 0);
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.row_wall[y].min_left(x, |sum| *sum == 0);
        if p == 0 {
            None
        } else {
            Some(p - 1)
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.row_wall[y].max_right(x + 1, |sum| *sum == 0);
        if p == self.w {
            None
        } else {
            Some(p)
        }
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.col_wall[x].min_left(y, |sum| *sum == 0);
        if p == 0 {
            None
        } else {
            Some(p - 1)
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.col_wall[x].max_right(y + 1, |sum| *sum == 0);
        if p == self.h {
            None
        } else {
            Some(p)
        }
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.wall[y][x]
    }

    fn count_wall(&self) -> usize {
        self.wall.iter().flatten().filter(|x| **x).count()
    }
}

// 壁を range min セグ木や range max セグ木で持つ（壁あり: 座標, 壁なし: +∞ または -∞）
struct GridSolve5 {
    wall: Vec<Vec<bool>>,
    row_wall_min: Vec<Segtree<Min<i64>>>,
    row_wall_max: Vec<Segtree<Max<i64>>>,
    col_wall_min: Vec<Segtree<Min<i64>>>,
    col_wall_max: Vec<Segtree<Max<i64>>>,
    h: usize,
    w: usize,
}
impl GridSolve5 {
    fn new(h: usize, w: usize) -> Self {
        let wall = vec![vec![true; w]; h];

        let row_wall_min = (0..h)
            .map(|_| Segtree::from((0..(w as i64)).collect_vec()))
            .collect_vec();

        let row_wall_max = (0..h)
            .map(|_| Segtree::from((0..(w as i64)).collect_vec()))
            .collect_vec();

        let col_wall_min = (0..w)
            .map(|_| Segtree::from((0..(h as i64)).collect_vec()))
            .collect_vec();

        let col_wall_max = (0..w)
            .map(|_| Segtree::from((0..(h as i64)).collect_vec()))
            .collect_vec();

        GridSolve5 {
            wall,
            row_wall_min,
            row_wall_max,
            col_wall_min,
            col_wall_max,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.wall[y][x] {
            return;
        }
        self.wall[y][x] = false;
        self.row_wall_min[y].set(x, i64::MAX);
        self.row_wall_max[y].set(x, i64::MIN);
        self.col_wall_min[x].set(y, i64::MAX);
        self.col_wall_max[x].set(y, i64::MIN);
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        if x == 0 {
            return None;
        }
        let p = self.row_wall_max[y].prod(..=x - 1);
        if p == i64::MIN {
            None
        } else {
            Some(p as usize)
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.row_wall_min[y].prod(x + 1..);
        if p == i64::MAX {
            None
        } else {
            Some(p as usize)
        }
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        if y == 0 {
            return None;
        }
        let p = self.col_wall_max[x].prod(..=y - 1);
        if p == i64::MIN {
            None
        } else {
            Some(p as usize)
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let p = self.col_wall_min[x].prod(y + 1..);
        if p == i64::MAX {
            None
        } else {
            Some(p as usize)
        }
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        self.wall[y][x]
    }

    fn count_wall(&self) -> usize {
        self.wall.iter().flatten().filter(|x| **x).count()
    }
}

// 区間を sorted set で管理するテクニック
struct GridSolve6 {
    row_no_wall: Vec<RangeSet>,
    col_no_wall: Vec<RangeSet>,
    h: usize,
    w: usize,
}
impl GridSolve6 {
    fn new(h: usize, w: usize) -> Self {
        let row_no_wall = (0..h).map(|_| RangeSet::new()).collect_vec();
        let col_no_wall = (0..w).map(|_| RangeSet::new()).collect_vec();

        GridSolve6 {
            row_no_wall,
            col_no_wall,
            h,
            w,
        }
    }

    fn break_wall(&mut self, y: usize, x: usize) {
        if !self.is_wall(y, x) {
            return;
        }
        self.row_no_wall[y].insert(x as i64);
        self.col_no_wall[x].insert(y as i64);
    }
    fn get_left_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let left = self.row_no_wall[y].max_exclusive_leq(x as i64);

        if left == -1 {
            None
        } else {
            Some(left as usize)
        }
    }

    fn get_right_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let right = self.row_no_wall[y].min_exclusive_geq(x as i64);

        if right == self.w as i64 {
            None
        } else {
            Some(right as usize)
        }
    }
    fn get_up_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let up = self.col_no_wall[x].max_exclusive_leq(y as i64);

        if up == -1 {
            None
        } else {
            Some(up as usize)
        }
    }
    fn get_down_wall(&mut self, y: usize, x: usize) -> Option<usize> {
        let down = self.col_no_wall[x].min_exclusive_geq(y as i64);

        if down == self.h as i64 {
            None
        } else {
            Some(down as usize)
        }
    }

    fn is_wall(&self, y: usize, x: usize) -> bool {
        !self.row_no_wall[y].contains(x as i64)
    }

    fn count_wall(&self) -> usize {
        self.w * self.h - self.row_no_wall.iter().map(|row| row.len()).sum::<usize>()
    }
}
impl Problem {
    fn read() -> Problem {
        input! {
            h: usize,
            w: usize,
            nq : usize,
            qs: [Query; nq],
        }
        Problem { h, w, nq, qs }
    }

    fn print(&self) {
        println!("{} {} {}", self.h, self.w, self.nq);
        for q in &self.qs {
            println!("{} {}", q.y + 1, q.x + 1);
        }
    }

    fn solve(&self) -> Answer {
        // 解法1 (GridSolve1 を使った場合)
        // 各点に対して、その点の上下左右にある壁の位置を保持する
        // （range update をしたいので、range update 遅延セグ木を持つ）

        // 解法2 (GridSolve2 を使った場合)
        // 壁を壊した区間を Union Find で管理する
        // 壁を壊したら両隣の区間をくっつける

        // 解法3 (GridSolve3 を使った場合)
        // 各行・各列の壁の位置を BTreeSet で管理する

        // 解法4 (GridSolve4 を使った場合)
        // 各行・各列の壁の位置を range sum セグ木で管理する(壁あり: 1 壁なし: 0)
        // 右側にある一番近い壁などはセグ木の二分探索を使う

        // 解法5 (GridSolve5 を使った場合)
        // 各行・各列の壁の位置を range min セグ木や range max セグ木で管理する
        // (壁あり: 座標の値, 壁なし: +∞ または -∞)
        // range max や range min をすると、壁の位置が得られる。

        // 解法6 (GridSolve6 を使った場合)
        // 区間を sorted set で持つテクニック

        let h = self.h;
        let w = self.w;
        // let mut grid = GridSolve1::new(h, w);
        // let mut grid = GridSolve2::new(h, w);
        // let mut grid = GridSolve3::new(h, w);
        // let mut grid = GridSolve4::new(h, w);
        // let mut grid = GridSolve5::new(h, w);
        let mut grid = GridSolve6::new(h, w);

        for q in &self.qs {
            // table! {&grid.wall};
            // dbg!(q);

            if grid.is_wall(q.y, q.x) {
                grid.break_wall(q.y, q.x);
            } else {
                // 壁がすでにない
                // 左側/右側/上側/下側の壊すブロックを求める（あれば）

                if let Some(left) = grid.get_left_wall(q.y, q.x) {
                    grid.break_wall(q.y, left);
                }

                if let Some(right) = grid.get_right_wall(q.y, q.x) {
                    grid.break_wall(q.y, right);
                }

                if let Some(up) = grid.get_up_wall(q.y, q.x) {
                    grid.break_wall(up, q.x);
                }

                if let Some(down) = grid.get_down_wall(q.y, q.x) {
                    grid.break_wall(down, q.x);
                }
            }
        }

        // table! {&grid.wall};

        let ans = grid.count_wall();
        Answer { ans }
    }

    #[allow(dead_code)]
    fn solve_naive(&self) -> Answer {
        let h = self.h;
        let w = self.w;
        let mut wall = vec![vec![true; w]; h];

        for q in &self.qs {
            if wall[q.y][q.x] {
                wall[q.y][q.x] = false;
            } else {
                // 上へ
                if let Some(y) = (0..q.y).rev().find(|&y| wall[y][q.x]) {
                    wall[y][q.x] = false;
                }

                // 下へ
                if let Some(y) = (q.y + 1..h).find(|&y| wall[y][q.x]) {
                    wall[y][q.x] = false;
                }

                // 左へ
                if let Some(x) = (0..q.x).rev().find(|&x| wall[q.y][x]) {
                    wall[q.y][x] = false;
                }

                // 右へ
                if let Some(x) = (q.x + 1..w).find(|&x| wall[q.y][x]) {
                    wall[q.y][x] = false;
                }
            }
        }
        let ans = wall.iter().flatten().filter(|x| **x).count();
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: usize,
}

impl Answer {
    fn print(&self) {
        println!("{}", self.ans);
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
        let h = rng.random_range(2..=4);
        let w = rng.random_range(2..=4);

        let nq = 5;
        let qs = (0..nq)
            .map(|_| {
                let y = rng.random_range(0..h);
                let x = rng.random_range(0..w);
                Query { y, x }
            })
            .collect_vec();

        let p = Problem { h, w, nq, qs };
        println!("{:?}", &p);
        p
    }

    #[allow(unreachable_code)]
    #[test]
    fn test_with_naive() {
        let num_tests = 1000;
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
                t.problem.print();
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
#[allow(unused_imports)]
use std::cmp::Reverse;
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
use range_affine_range_sum::*;
pub mod range_affine_range_sum {
    use ac_library::{LazySegtree, MapMonoid, Monoid};
    use itertools::Itertools;
    use std::convert::Infallible;
    use std::marker::PhantomData;
    use std::ops::{Add, Mul, RangeBounds};
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct RangeSum<T> {
        pub sum: T,
        pub len: i64,
    }
    impl<T> RangeSum<T> {
        pub fn unit(x: T) -> RangeSum<T> {
            RangeSum { sum: x, len: 1 }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Affine<T> {
        pub slope: T,
        pub intercept: T,
    }
    impl<T> Affine<T>
    where
        T: From<i64>,
    {
        /// 区間変更用（定数関数）
        pub fn constant_func(x: T) -> Affine<T> {
            Affine {
                slope: 0.into(),
                intercept: x,
            }
        }
        /// 区間加算用
        pub fn addition_func(x: T) -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: x,
            }
        }
    }
    pub struct ValueLenSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> Monoid for ValueLenSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type S = RangeSum<T>;
        fn identity() -> RangeSum<T> {
            RangeSum {
                sum: 0.into(),
                len: 0,
            }
        }
        fn binary_operation(a: &RangeSum<T>, b: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: a.sum + b.sum,
                len: a.len + b.len,
            }
        }
    }
    pub struct RangeAffineRangeSum<T>(Infallible, PhantomData<fn() -> T>);
    impl<T> MapMonoid for RangeAffineRangeSum<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        type M = ValueLenSum<T>;
        type F = Affine<T>;
        fn identity_map() -> Affine<T> {
            Affine {
                slope: 1.into(),
                intercept: 0.into(),
            }
        }
        fn composition(a: &Affine<T>, b: &Affine<T>) -> Affine<T> {
            Affine {
                slope: a.slope * b.slope,
                intercept: a.slope * b.intercept + a.intercept,
            }
        }
        fn mapping(f: &Affine<T>, x: &RangeSum<T>) -> RangeSum<T> {
            RangeSum {
                sum: f.slope * x.sum + f.intercept * x.len.into(),
                len: x.len,
            }
        }
    }
    pub struct RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        segtree: LazySegtree<RangeAffineRangeSum<T>>,
        len: usize,
    }
    impl<T> RangeAffineRangeSumSegtree<T>
    where
        T: Copy + Mul<Output = T> + Add<Output = T> + From<i64>,
    {
        pub fn new(xs: &[T]) -> RangeAffineRangeSumSegtree<T> {
            let xs = xs.iter().copied().map(RangeSum::unit).collect_vec();
            let len = xs.len();
            RangeAffineRangeSumSegtree {
                segtree: LazySegtree::from(xs),
                len,
            }
        }
        pub fn set(&mut self, p: usize, x: T) {
            self.segtree.set(p, RangeSum::unit(x));
        }
        pub fn get(&mut self, p: usize) -> T {
            self.segtree.get(p).sum
        }
        pub fn range_sum<R>(&mut self, range: R) -> T
        where
            R: RangeBounds<usize>,
        {
            self.segtree.prod(range).sum
        }
        pub fn all_sum(&self) -> T {
            self.segtree.all_prod().sum
        }
        pub fn apply_affine(&mut self, p: usize, slope: T, intercept: T) {
            self.segtree.apply(p, Affine { slope, intercept })
        }
        pub fn apply_update(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::constant_func(x))
        }
        pub fn apply_add(&mut self, p: usize, x: T) {
            self.segtree.apply(p, Affine::addition_func(x))
        }
        pub fn apply_range_affine<R>(&mut self, range: R, slope: T, intercept: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine { slope, intercept })
        }
        pub fn apply_range_update<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::constant_func(x))
        }
        pub fn apply_range_add<R>(&mut self, range: R, x: T)
        where
            R: RangeBounds<usize>,
        {
            self.segtree.apply_range(range, Affine::addition_func(x))
        }
        pub fn to_vec(&mut self) -> Vec<T> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
use monoid_union_find::*;
/// 可換モノイドをのっけた Union Find
pub mod monoid_union_find {
    use ac_library::Monoid;
    use itertools::Itertools;
    #[derive(Clone, Debug)]
    struct RootInfo<S: Clone> {
        count: usize,
        prod: S,
    }
    #[derive(Clone, Debug)]
    struct NonRootInfo {
        parent: usize,
    }
    #[derive(Clone, Debug)]
    enum Node<S: Clone> {
        Root(RootInfo<S>),
        NonRoot(NonRootInfo),
    }
    impl<S: Clone> Node<S> {
        fn root(count: usize, prod: S) -> Node<S> {
            Node::Root(RootInfo { count, prod })
        }
        fn non_root(parent: usize) -> Node<S> {
            Node::NonRoot(NonRootInfo { parent })
        }
    }
    impl<S: Clone> Node<S> {
        fn as_root(&self) -> &RootInfo<S> {
            match self {
                Node::Root(info) => info,
                Node::NonRoot(_) => panic!(),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct MonoidUnionFind<M: Monoid> {
        nodes: Vec<Node<M::S>>,
        cnt_groups: usize,
    }
    impl<M: Monoid> MonoidUnionFind<M> {
        pub fn new(data: &[M::S]) -> MonoidUnionFind<M> {
            let nodes = data.iter().map(|d| Node::root(1, d.clone())).collect_vec();
            MonoidUnionFind {
                nodes,
                cnt_groups: data.len(),
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            match &self.nodes[index] {
                Node::Root(_) => index,
                Node::NonRoot(info) => {
                    let root = self.root(info.parent);
                    self.nodes[index] = Node::non_root(root);
                    root
                }
            }
        }
        pub fn same_count(&mut self, index: usize) -> usize {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().count
        }
        pub fn same_prod(&mut self, index: usize) -> M::S {
            let root_index = self.root(index);
            self.nodes[root_index].as_root().prod.clone()
        }
        pub fn same_prod_ref(&mut self, index: usize) -> &M::S {
            let root_index = self.root(index);
            &self.nodes[root_index].as_root().prod
        }
        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.root(x) == self.root(y)
        }
        pub fn num_groups(&self) -> usize {
            self.cnt_groups
        }
        pub fn unite(&mut self, x: usize, y: usize) -> bool {
            if self.same(x, y) {
                return false;
            }
            self.cnt_groups -= 1;
            let (smaller_root, larger_root) = {
                let x_root = self.root(x);
                let y_root = self.root(y);
                let x_count = self.nodes[x_root].as_root().count;
                let y_count = self.nodes[y_root].as_root().count;
                if x_count < y_count {
                    (x_root, y_root)
                } else {
                    (y_root, x_root)
                }
            };
            let smaller_root_info = self.nodes[smaller_root].as_root();
            let larger_root_info = self.nodes[larger_root].as_root();
            let count = smaller_root_info.count + larger_root_info.count;
            let prod = M::binary_operation(&smaller_root_info.prod, &larger_root_info.prod);
            self.nodes[smaller_root] = Node::non_root(larger_root);
            self.nodes[larger_root] = Node::root(count, prod);
            true
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let n = self.nodes.len();
            let roots = (0..n).map(|i| self.root(i)).collect_vec();
            let group_size = (0..n).map(|i| roots[i]).fold(vec![0; n], |mut acc, x| {
                acc[x] += 1;
                acc
            });
            let result = {
                let mut result = vec![Vec::new(); n];
                for i in 0..n {
                    result[i].reserve(group_size[i]);
                }
                for i in 0..n {
                    result[roots[i]].push(i);
                }
                result
            };
            result.into_iter().filter(|x| !x.is_empty()).collect_vec()
        }
    }
}
use range_set::*;
#[allow(clippy::module_inception)]
pub mod range_set {
    use std::collections::BTreeSet;
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RangeSet {
        set: BTreeSet<(i64, i64)>,
        count: usize,
    }
    impl Default for RangeSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl RangeSet {
        pub fn new() -> RangeSet {
            RangeSet {
                set: vec![(i64::MIN, i64::MIN), (i64::MAX, i64::MAX)]
                    .into_iter()
                    .collect(),
                count: 0,
            }
        }
        pub fn iter(&self) -> impl Iterator<Item = i64> + '_ {
            self.set
                .iter()
                .copied()
                .filter(|&(l, r)| (l, r) != (i64::MIN, i64::MIN) && (l, r) != (i64::MAX, i64::MAX))
                .flat_map(|(left, right)| left..=right)
        }
        pub fn insert(&mut self, x: i64) -> bool {
            if self.contains(x) {
                return false;
            }
            let &(prev_l, prev_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            let &(next_l, next_r) = self.set.range((x + 1, x + 1)..).min().unwrap();
            if prev_r + 1 == x && x == next_l - 1 {
                self.set.remove(&(prev_l, prev_r));
                self.set.remove(&(next_l, next_r));
                self.set.insert((prev_l, next_r));
            } else if prev_r + 1 == x {
                self.set.remove(&(prev_l, prev_r));
                self.set.insert((prev_l, x));
            } else if x == next_l - 1 {
                self.set.remove(&(next_l, next_r));
                self.set.insert((x, next_r));
            } else {
                self.set.insert((x, x));
            }
            self.count += 1;
            true
        }
        pub fn remove(&mut self, x: i64) -> bool {
            if !self.contains(x) {
                return false;
            }
            let &(current_l, current_r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if current_l == x && x == current_r {
                self.set.remove(&(current_l, current_r));
            } else if current_l == x {
                self.set.remove(&(current_l, current_r));
                self.set.insert((x + 1, current_r));
            } else if x == current_r {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
            } else {
                self.set.remove(&(current_l, current_r));
                self.set.insert((current_l, x - 1));
                self.set.insert((x + 1, current_r));
            }
            self.count -= 1;
            true
        }
        pub fn len(&self) -> usize {
            self.count
        }
        pub fn is_empty(&self) -> bool {
            self.count == 0
        }
        pub fn contains(&self, x: i64) -> bool {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            (l..=r).contains(&x)
        }
        /// x 以上で self に入っていない値の最小値を返す (いわゆる mex)
        pub fn min_exclusive_geq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) {
                r + 1
            } else {
                x
            }
        }
        /// x 以下で self に入っていない値の最大値を返す
        pub fn max_exclusive_leq(&self, x: i64) -> i64 {
            let &(l, r) = self.set.range(..(x + 1, x + 1)).max().unwrap();
            if (l..=r).contains(&x) {
                l - 1
            } else {
                x
            }
        }
    }
    impl FromIterator<i64> for RangeSet {
        fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> RangeSet {
            let mut set = RangeSet::new();
            for x in iter {
                set.insert(x);
            }
            set
        }
    }
}
