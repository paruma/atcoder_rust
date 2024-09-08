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

fn break_wall(
    y: usize,
    x: usize,
    wall: &mut [Vec<bool>],
    right_wall: &mut [RangeAffineRangeSumSegtree<i64>],
    left_wall: &mut [RangeAffineRangeSumSegtree<i64>],
    down_wall: &mut [RangeAffineRangeSumSegtree<i64>],
    up_wall: &mut [RangeAffineRangeSumSegtree<i64>],
    w: usize,
    h: usize,
) {
    if !wall[y][x] {
        return;
    }
    wall[y][x] = false;
    {
        let next_left_wall = if x == 0 { -1 } else { left_wall[y].get(x - 1) };
        let next_right_wall = if x == w - 1 {
            w as i64
        } else {
            right_wall[y].get(x + 1)
        };

        let range = (next_left_wall + 1) as usize..=(next_right_wall - 1) as usize;
        left_wall[y].apply_range_update(range.clone(), next_left_wall);
        right_wall[y].apply_range_update(range, next_right_wall);
    }
    {
        let next_up_wall = if y == 0 { -1 } else { up_wall[x].get(y - 1) };
        let next_down_wall = if y == h - 1 {
            h as i64
        } else {
            down_wall[x].get(y + 1)
        };

        let range = (next_up_wall + 1) as usize..=(next_down_wall - 1) as usize;
        //dbg!(next_down_wall);
        //dbg!(range.clone());

        up_wall[x].apply_range_update(range.clone(), next_up_wall);
        down_wall[x].apply_range_update(range, next_down_wall);
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
        let h = self.h;
        let w = self.w;
        let mut wall = vec![vec![true; w]; h];

        let mut right_wall: Vec<RangeAffineRangeSumSegtree<i64>> = (0..h)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..w).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let mut left_wall = (0..h)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..w).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let mut up_wall = (0..w)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..h).map(|i| i as i64).collect_vec()))
            .collect_vec();

        let mut down_wall = (0..w)
            .map(|_| RangeAffineRangeSumSegtree::new(&(0..h).map(|i| i as i64).collect_vec()))
            .collect_vec();

        for q in &self.qs {
            //table! {&wall};
            // let r = right_wall.iter_mut().map(|xs| xs.to_vec()).collect_vec();
            // let l = left_wall.iter_mut().map(|xs| xs.to_vec()).collect_vec();
            // let d = down_wall.iter_mut().map(|xs| xs.to_vec()).collect_vec();
            // let u = up_wall.iter_mut().map(|xs| xs.to_vec()).collect_vec();
            //table! {r};
            //table! {l};
            // table! {d};
            // table! {u};

            //dbg!(q);

            if wall[q.y][q.x] {
                break_wall(
                    q.y,
                    q.x,
                    &mut wall,
                    &mut right_wall,
                    &mut left_wall,
                    &mut down_wall,
                    &mut up_wall,
                    w,
                    h,
                );
                //wall[q.y][q.x] = false;
                //// TODO :修正する
                //right_wall[q.y].set(q.x, q.x as i64 + 1);
                //left_wall[q.y].set(q.x, q.x as i64 - 1);
                //down_wall[q.x].set(q.y, q.y as i64 + 1);
                //up_wall[q.x].set(q.y, q.y as i64 - 1);
            } else {
                // 壁がすでにない
                // 左側/右側/上側/下側の壊すブロックを求める（あれば）

                // let current_left_wall = left_wall[q.y].get(q.x);
                // // (q.y, current_left_wall) を壊す
                // if current_left_wall >= 0 {
                //     wall[q.y][current_left_wall as usize] = false;
                // }

                // let current_right_wall = right_wall[q.y].get(q.x);
                // if current_right_wall < w as i64 {
                //     wall[q.y][current_right_wall as usize] = false;
                // }

                // let current_up_wall = up_wall[q.x].get(q.y);
                // if current_up_wall >= 0 {
                //     wall[current_up_wall as usize][q.x] = false;
                // }

                // let current_down_wall = down_wall[q.x].get(q.y);
                // if current_down_wall < h as i64 {
                //     wall[current_down_wall as usize][q.x] = false;
                // }

                let current_left_wall = left_wall[q.y].get(q.x);
                // (q.y, current_left_wall) を壊す
                if current_left_wall >= 0 {
                    break_wall(
                        q.y,
                        current_left_wall as usize,
                        &mut wall,
                        &mut right_wall,
                        &mut left_wall,
                        &mut down_wall,
                        &mut up_wall,
                        w,
                        h,
                    );
                }

                let current_right_wall = right_wall[q.y].get(q.x);
                if current_right_wall < w as i64 {
                    break_wall(
                        q.y,
                        current_right_wall as usize,
                        &mut wall,
                        &mut right_wall,
                        &mut left_wall,
                        &mut down_wall,
                        &mut up_wall,
                        w,
                        h,
                    );
                }

                let current_up_wall = up_wall[q.x].get(q.y);
                if current_up_wall >= 0 {
                    break_wall(
                        current_up_wall as usize,
                        q.x,
                        &mut wall,
                        &mut right_wall,
                        &mut left_wall,
                        &mut down_wall,
                        &mut up_wall,
                        w,
                        h,
                    );
                }

                let current_down_wall = down_wall[q.x].get(q.y);
                if current_down_wall < h as i64 {
                    //wall[current_down_wall as usize][q.x] = false;
                    break_wall(
                        current_down_wall as usize,
                        q.x,
                        &mut wall,
                        &mut right_wall,
                        &mut left_wall,
                        &mut down_wall,
                        &mut up_wall,
                        w,
                        h,
                    );
                }

                // right_wall たちを更新する

                // let next_left_wall = (current_left_wall - 1).max(-1);
                // let next_right_wall = (current_right_wall + 1).min(w as i64);
                // let next_up_wall = (current_up_wall - 1).max(-1);
                // let next_down_wall = (current_down_wall + 1).min(h as i64);

                // let range_row = {
                //     let left = current_left_wall.max(0);
                //     let right = current_right_wall.min(w as i64 - 1);
                //     (left as usize)..=(right as usize)
                // };

                // let range_col = {
                //     let up = current_up_wall.max(0);
                //     let down = current_down_wall.min(h as i64 - 1);
                //     (up as usize)..=(down as usize)
                // };

                // right_wall[q.y].apply_range_update(range_row.clone(), next_right_wall);

                // left_wall[q.y].apply_range_update(range_row.clone(), next_left_wall);

                // down_wall[q.x].apply_range_update(range_col.clone(), next_down_wall);

                // up_wall[q.x].apply_range_update(range_col.clone(), next_up_wall);
            }
        }

        //table! {&wall};

        let ans = wall.iter().flatten().filter(|x| **x).count() as i64;
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
        let ans = wall.iter().flatten().filter(|x| **x).count() as i64;
        Answer { ans }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Answer {
    ans: i64,
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
        let h = rng.gen_range(2..=4);
        let w = rng.gen_range(2..=4);

        let nq = 5;
        let qs = (0..nq)
            .map(|_| {
                let y = rng.gen_range(0..h);
                let x = rng.gen_range(0..w);
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
use lg::*;
pub mod lg {
    use std::borrow::Borrow;
    use std::fmt;
    use std::iter::once;
    /// Print the values with the line number.
    /// # Examples
    /// ```rust
    /// # use lg::*;
    /// let x = 42;
    /// let y = 43;
    /// lg!(x);
    /// lg!(x, y);
    /// lg!(42, x, 43, y);
    /// ```
    #[macro_export]
    macro_rules ! lg {(@ contents $ head : expr $ (, $ tail : expr ) * ) => {{$ crate :: __lg_internal ! ($ head ) ; $ (eprint ! ("," ) ; $ crate :: __lg_internal ! ($ tail ) ; ) * eprintln ! () ; } } ; ($ ($ expr : expr ) ,* $ (, ) ? ) => {{eprint ! ("{}\u{276f}" , line ! () ) ; $ crate :: lg ! (@ contents $ ($ expr ) ,* ) } } ; }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __lg_internal {
        ($ value : expr ) => {{
            match $value {
                head => {
                    eprint!(
                        " {} = {}",
                        stringify!($value),
                        $crate::__quiet(format!("{:?}", &head))
                    );
                }
            }
        }};
    }
    /// Print many 1D arrays side-by-side with the line number.
    /// # Examples
    /// ```rust
    /// # use lg::*;
    /// let a = [1, 2, 3];
    /// let b = [4, 5, 6];
    /// let c = [7, 8, 9];
    /// rows! {
    ///   "id", // the name of the index
    ///   @"a" => a,
    ///   b,
    ///   @"c" => c,
    /// }
    /// ```
    #[macro_export]
    macro_rules ! rows {{$ index_label : literal , $ (@ offset $ offset : expr , ) ? $ (@ verticalbar $ verticalbar : expr , ) * $ ($ (@$ label : literal => ) ? $ values : expr ) ,* $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; $ (rows . offset ($ offset ) ; ) ? $ (rows . verticalbar ($ verticalbar ) ; ) * rows . index_label ($ index_label ) ; $ ({let mut label = stringify ! ($ values ) . to_string () ; if label . starts_with ("&" ) {label = label [1 .. ] . to_string () ; } $ ({let label_ : &'static str = $ label ; label = label_ . to_string () ; } ) ? rows . row (label , $ values ) ; } ) * eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
    /// Print the 2D array with the line number.
    /// # Examples
    /// ```rust
    /// # use lg::*;
    /// let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    /// table! {
    ///    @"a" => a,
    /// }
    /// table! {
    ///   a,
    /// }
    /// ```
    #[macro_export]
    macro_rules ! table {{$ (@$ name : literal => ) ? $ values : expr $ (, ) ? } => {{#! [allow (unused_assignments ) ] let mut name = stringify ! ($ values ) . to_string () ; if name . starts_with ("&" ) {name = name [1 .. ] . to_string () ; } $ ({let name_ : &'static str = $ name ; name = name_ . to_string () ; } ) ? let mut rows = $ crate :: Rows :: default () ; rows . line_number (line ! () ) ; rows . table_name (name ) ; # [allow (array_into_iter ) ] for (i , row ) in $ values . into_iter () . enumerate () {rows . row (i . to_string () , row ) ; } eprintln ! ("{}" , rows . to_string_table () ) ; } } ; }
    #[doc(hidden)]
    pub fn __quiet(s: impl AsRef<str>) -> String {
        s.as_ref()
            .replace("340282366920938463463374607431768211455", "*")
            .replace("170141183460469231731687303715884105727", "*")
            .replace("18446744073709551615", "*")
            .replace("9223372036854775807", "*")
            .replace("-9223372036854775808", "*")
            .replace("4294967295", "*")
            .replace("2147483647", "*")
            .replace("-2147483648", "*")
            .replace("None", "*")
            .replace("Some", "")
            .replace("true", "#")
            .replace("false", ".")
            .replace(['"', '\''], "")
    }
    #[doc(hidden)]
    #[derive(Default)]
    pub struct Rows {
        line_number: String,
        index_label: String,
        offset: usize,
        verticalbars: Vec<usize>,
        table_name: String,
        rows: Vec<Row>,
    }
    impl Rows {
        pub fn line_number(&mut self, line_number: u32) -> &mut Self {
            self.line_number = format!("{}", line_number);
            self
        }
        pub fn index_label(&mut self, index_label: impl Into<String>) -> &mut Self {
            self.index_label = index_label.into();
            self
        }
        pub fn offset(&mut self, offset: usize) -> &mut Self {
            self.offset = offset;
            self
        }
        pub fn verticalbar(&mut self, verticalbar: impl IntoIterator<Item = usize>) -> &mut Self {
            self.verticalbars.extend(verticalbar);
            self
        }
        pub fn table_name(&mut self, table_name: impl Into<String>) -> &mut Self {
            self.table_name = table_name.into();
            self
        }
        pub fn row(
            &mut self,
            label: impl Into<String>,
            values: impl IntoIterator<Item = impl fmt::Debug>,
        ) -> &mut Self {
            self.rows.push(Row {
                label: label.into(),
                values: values
                    .into_iter()
                    .map(|value| __quiet(format!("{:?}", value)))
                    .collect(),
            });
            self
        }
        pub fn to_string_table(self) -> StringTable {
            let Self {
                line_number,
                index_label,
                offset,
                verticalbars,
                table_name,
                rows,
            } = self;
            let w = rows
                .iter()
                .map(|row| row.values.len())
                .max()
                .unwrap_or_default();
            let mut verticalbar_count = vec![0; w + 1];
            for &v in &verticalbars {
                if (offset..=offset + w).contains(&v) {
                    verticalbar_count[v - offset] += 1;
                }
            }
            StringTable {
                head: StringRow {
                    label: format!(
                        "{line_number}❯ {table_name}{index_label}",
                        index_label = if index_label.is_empty() {
                            String::new()
                        } else {
                            format!("[{}]", index_label)
                        }
                    ),
                    values: (offset..offset + w)
                        .map(|index| index.to_string())
                        .collect(),
                },
                body: rows
                    .iter()
                    .map(|row| StringRow {
                        label: row.label.clone(),
                        values: row.values.clone(),
                    })
                    .collect(),
                verticalbar_count,
            }
        }
    }
    struct Row {
        label: String,
        values: Vec<String>,
    }
    #[doc(hidden)]
    pub struct StringTable {
        head: StringRow,
        body: Vec<StringRow>,
        verticalbar_count: Vec<usize>,
    }
    impl fmt::Display for StringTable {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let Self {
                head,
                body,
                verticalbar_count,
            } = self;
            let w = body
                .iter()
                .map(|row| row.values.len())
                .max()
                .unwrap_or_default();
            let label_width = once(head.label.chars().count())
                .chain(body.iter().map(|row| row.label.chars().count()))
                .max()
                .unwrap();
            let value_width = (0..w)
                .map(|j| {
                    once(j.to_string().len())
                        .chain(
                            body.iter()
                                .map(|row| row.values.get(j).map_or(0, |s| s.chars().count())),
                        )
                        .max()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            gray(f)?;
            write!(
                f,
                "{}",
                head.to_string(label_width, &value_width, verticalbar_count, true)
            )?;
            resetln(f)?;
            for row in body {
                write!(
                    f,
                    "{}",
                    row.to_string(label_width, &value_width, verticalbar_count, false)
                )?;
                writeln!(f)?;
            }
            Ok(())
        }
    }
    struct StringRow {
        label: String,
        values: Vec<String>,
    }
    impl StringRow {
        fn to_string(
            &self,
            label_width: usize,
            value_width: &[usize],
            varticalbars_count: &[usize],
            label_align_left: bool,
        ) -> String {
            let Self { label, values } = self;
            let w = value_width.len();
            let mut s = String::new();
            s.push_str(&if label_align_left {
                format!("{label:<label_width$} |")
            } else {
                format!("{label:^label_width$} |")
            });
            for j in 0..w {
                let value_width = value_width[j];
                s.push_str("|".repeat(varticalbars_count[j]).as_str());
                if varticalbars_count[j] == 0 && j != 0 && value_width <= 1 {
                    s.push(' ');
                }
                match values.get(j) {
                    Some(value) => {
                        s.push_str(&format!(" {value:>value_width$}",));
                    }
                    None => {
                        s.push_str(" ".repeat(value_width + 1).as_str());
                    }
                }
            }
            s
        }
    }
    const GRAY: &str = "\x1b[48;2;127;127;127;37m";
    const RESET: &str = "\x1b[0m";
    fn gray(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{GRAY}")
    }
    fn resetln(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{RESET}")
    }
    /// Format a iterator of [`bool`]s.
    pub fn bools<B, I>(iter: I) -> String
    where
        B: Borrow<bool>,
        I: IntoIterator<Item = B>,
    {
        format!(
            "[{}]",
            iter.into_iter()
                .map(|b| ['.', '#'][usize::from(*(b.borrow()))])
                .collect::<String>(),
        )
    }
}
