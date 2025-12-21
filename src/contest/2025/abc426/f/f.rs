#[fastout]
fn main() {
    input! {
        n: usize,
        xs: [i64; n],
        q: usize,
    }

    let mut seg = RangeChminChmaxAddRangeSumSegtree::from(&xs);

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            k: i64,
        }
        let before_sum = seg.sum(l..=r);
        seg.add(l..=r, -k);
        seg.chmax(l..=r, 0);
        let after_sum = seg.sum(l..=r);

        let ans = before_sum - after_sum;
        println!("{}", ans);
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
use abstract_segtree_beats::*;
use range_chmin_chmax_add_range_sum_beats::*;
#[allow(clippy::module_inception)]
pub mod abstract_segtree_beats {
    fn ceil_pow2(n: u32) -> u32 {
        32 - n.saturating_sub(1).leading_zeros()
    }
    pub trait MonoidBeats {
        type S: Clone;
        fn identity() -> Self::S;
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
        fn fails(a: &Self::S) -> bool;
    }
    pub trait MapMonoidBeats {
        type M: MonoidBeats;
        type F: Clone;
        fn identity_element() -> <Self::M as MonoidBeats>::S {
            Self::M::identity()
        }
        fn binary_operation(
            a: &<Self::M as MonoidBeats>::S,
            b: &<Self::M as MonoidBeats>::S,
        ) -> <Self::M as MonoidBeats>::S {
            Self::M::binary_operation(a, b)
        }
        fn fails(a: &<Self::M as MonoidBeats>::S) -> bool {
            Self::M::fails(a)
        }
        fn identity_map() -> Self::F;
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S;
        fn composition(f: &Self::F, g: &Self::F) -> Self::F;
    }
    impl<F: MapMonoidBeats> Default for SegtreeBeats<F> {
        fn default() -> Self {
            Self::new(0)
        }
    }
    impl<F: MapMonoidBeats> SegtreeBeats<F> {
        pub fn new(n: usize) -> Self {
            vec![F::identity_element(); n].into()
        }
    }
    impl<F: MapMonoidBeats> From<Vec<<F::M as MonoidBeats>::S>> for SegtreeBeats<F> {
        fn from(v: Vec<<F::M as MonoidBeats>::S>) -> Self {
            let n = v.len();
            let log = ceil_pow2(n as u32) as usize;
            let size = 1 << log;
            let mut d = vec![F::identity_element(); 2 * size];
            let lz = vec![F::identity_map(); size];
            d[size..(size + n)].clone_from_slice(&v);
            let mut ret = SegtreeBeats {
                n,
                size,
                log,
                d,
                lz,
            };
            for i in (1..size).rev() {
                ret.update(i);
            }
            ret
        }
    }
    impl<F: MapMonoidBeats> SegtreeBeats<F> {
        pub fn set(&mut self, mut p: usize, x: <F::M as MonoidBeats>::S) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = x;
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
        pub fn get(&mut self, mut p: usize) -> <F::M as MonoidBeats>::S {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p].clone()
        }
        pub fn prod<R>(&mut self, range: R) -> <F::M as MonoidBeats>::S
        where
            R: RangeBounds<usize>,
        {
            if range.start_bound() == Bound::Unbounded && range.end_bound() == Bound::Unbounded {
                return self.all_prod();
            }
            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => 0,
            };
            assert!(l <= r && r <= self.n);
            if l == r {
                return F::identity_element();
            }
            l += self.size;
            r += self.size;
            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push(r >> i);
                }
            }
            let mut sml = F::identity_element();
            let mut smr = F::identity_element();
            while l < r {
                if l & 1 != 0 {
                    sml = F::binary_operation(&sml, &self.d[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    smr = F::binary_operation(&self.d[r], &smr);
                }
                l >>= 1;
                r >>= 1;
            }
            F::binary_operation(&sml, &smr)
        }
        pub fn all_prod(&self) -> <F::M as MonoidBeats>::S {
            self.d[1].clone()
        }
        pub fn apply(&mut self, mut p: usize, f: F::F) {
            assert!(p < self.n);
            p += self.size;
            for i in (1..=self.log).rev() {
                self.push(p >> i);
            }
            self.d[p] = F::mapping(&f, &self.d[p]);
            for i in 1..=self.log {
                self.update(p >> i);
            }
        }
        pub fn apply_range<R>(&mut self, range: R, f: F::F)
        where
            R: RangeBounds<usize>,
        {
            let mut r = match range.end_bound() {
                Bound::Included(r) => r + 1,
                Bound::Excluded(r) => *r,
                Bound::Unbounded => self.n,
            };
            let mut l = match range.start_bound() {
                Bound::Included(l) => *l,
                Bound::Excluded(l) => l + 1,
                Bound::Unbounded => 0,
            };
            assert!(l <= r && r <= self.n);
            if l == r {
                return;
            }
            l += self.size;
            r += self.size;
            for i in (1..=self.log).rev() {
                if ((l >> i) << i) != l {
                    self.push(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.push((r - 1) >> i);
                }
            }
            {
                let l2 = l;
                let r2 = r;
                while l < r {
                    if l & 1 != 0 {
                        self.all_apply(l, f.clone());
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        self.all_apply(r, f.clone());
                    }
                    l >>= 1;
                    r >>= 1;
                }
                l = l2;
                r = r2;
            }
            for i in 1..=self.log {
                if ((l >> i) << i) != l {
                    self.update(l >> i);
                }
                if ((r >> i) << i) != r {
                    self.update((r - 1) >> i);
                }
            }
        }
        pub fn max_right<G>(&mut self, mut l: usize, g: G) -> usize
        where
            G: Fn(<F::M as MonoidBeats>::S) -> bool,
        {
            assert!(l <= self.n);
            assert!(g(F::identity_element()));
            if l == self.n {
                return self.n;
            }
            l += self.size;
            for i in (1..=self.log).rev() {
                self.push(l >> i);
            }
            let mut sm = F::identity_element();
            while {
                while l % 2 == 0 {
                    l >>= 1;
                }
                if !g(F::binary_operation(&sm, &self.d[l])) {
                    while l < self.size {
                        self.push(l);
                        l *= 2;
                        let res = F::binary_operation(&sm, &self.d[l]);
                        if g(res.clone()) {
                            sm = res;
                            l += 1;
                        }
                    }
                    return l - self.size;
                }
                sm = F::binary_operation(&sm, &self.d[l]);
                l += 1;
                {
                    let l = l as isize;
                    (l & -l) != l
                }
            } {}
            self.n
        }
        pub fn min_left<G>(&mut self, mut r: usize, g: G) -> usize
        where
            G: Fn(<F::M as MonoidBeats>::S) -> bool,
        {
            assert!(r <= self.n);
            assert!(g(F::identity_element()));
            if r == 0 {
                return 0;
            }
            r += self.size;
            for i in (1..=self.log).rev() {
                self.push((r - 1) >> i);
            }
            let mut sm = F::identity_element();
            while {
                r -= 1;
                while r > 1 && r % 2 != 0 {
                    r >>= 1;
                }
                if !g(F::binary_operation(&self.d[r], &sm)) {
                    while r < self.size {
                        self.push(r);
                        r = 2 * r + 1;
                        let res = F::binary_operation(&self.d[r], &sm);
                        if g(res.clone()) {
                            sm = res;
                            r -= 1;
                        }
                    }
                    return r + 1 - self.size;
                }
                sm = F::binary_operation(&self.d[r], &sm);
                {
                    let r = r as isize;
                    (r & -r) != r
                }
            } {}
            0
        }
        pub fn to_vec(&mut self) -> Vec<<F::M as MonoidBeats>::S> {
            (0..self.n).map(|i| self.get(i)).collect()
        }
    }
    pub struct SegtreeBeats<F>
    where
        F: MapMonoidBeats,
    {
        n: usize,
        size: usize,
        log: usize,
        d: Vec<<F::M as MonoidBeats>::S>,
        lz: Vec<F::F>,
    }
    impl<F> SegtreeBeats<F>
    where
        F: MapMonoidBeats,
    {
        fn update(&mut self, k: usize) {
            self.d[k] = F::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
        }
        fn all_apply(&mut self, k: usize, f: F::F) {
            self.d[k] = F::mapping(&f, &self.d[k]);
            if k < self.size {
                self.lz[k] = F::composition(&f, &self.lz[k]);
                if F::fails(&self.d[k]) {
                    self.push(k);
                    self.update(k)
                }
            }
        }
        fn push(&mut self, k: usize) {
            self.all_apply(2 * k, self.lz[k].clone());
            self.all_apply(2 * k + 1, self.lz[k].clone());
            self.lz[k] = F::identity_map();
        }
    }
    use std::{
        fmt::{Debug, Error, Formatter, Write},
        ops::{Bound, RangeBounds},
    };
    impl<F> Debug for SegtreeBeats<F>
    where
        F: MapMonoidBeats,
        F::F: Debug,
        <F::M as MonoidBeats>::S: Debug,
    {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            for i in 0..self.log {
                for j in 0..1 << i {
                    f.write_fmt(format_args!(
                        "{:?}[{:?}]\t",
                        self.d[(1 << i) + j],
                        self.lz[(1 << i) + j]
                    ))?;
                }
                f.write_char('\n')?;
            }
            for i in 0..self.size {
                f.write_fmt(format_args!("{:?}\t", self.d[self.size + i]))?;
            }
            Ok(())
        }
    }
}
#[allow(clippy::module_inception)]
pub mod range_chmin_chmax_add_range_sum_beats {
    use super::{MapMonoidBeats, MonoidBeats, SegtreeBeats};
    use itertools::Itertools;
    use std::{
        cmp::{max, min},
        convert::Infallible,
        ops::RangeBounds,
    };
    const INF: i64 = i64::MAX;
    const NEG_INF: i64 = i64::MIN;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct RangeSum {
        pub sum: i64,
        pub len: usize,
        pub max: i64,
        pub max_2nd: i64,
        pub max_cnt: usize,
        pub min: i64,
        pub min_2nd: i64,
        pub min_cnt: usize,
    }
    impl RangeSum {
        pub fn unit(x: i64) -> Option<RangeSum> {
            Some(RangeSum {
                sum: x,
                len: 1,
                max: x,
                max_2nd: NEG_INF,
                max_cnt: 1,
                min: x,
                min_2nd: INF,
                min_cnt: 1,
            })
        }
    }
    fn second_lowest(a: i64, a2: i64, b: i64, b2: i64) -> i64 {
        if a == b {
            min(a2, b2)
        } else if a2 <= b {
            a2
        } else if b2 <= a {
            b2
        } else {
            max(a, b)
        }
    }
    fn second_highest(a: i64, a2: i64, b: i64, b2: i64) -> i64 {
        if a == b {
            max(a2, b2)
        } else if a2 >= b {
            a2
        } else if b2 >= a {
            b2
        } else {
            min(a, b)
        }
    }
    pub struct RangeSumMonoid(Infallible);
    impl MonoidBeats for RangeSumMonoid {
        type S = Option<RangeSum>;
        fn identity() -> Self::S {
            Some(RangeSum {
                sum: 0,
                len: 0,
                max: NEG_INF,
                max_2nd: NEG_INF,
                max_cnt: 0,
                min: INF,
                min_2nd: INF,
                min_cnt: 0,
            })
        }
        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            match (a, b) {
                (None, None) => None,
                (None, Some(_)) => None,
                (Some(_), None) => None,
                (Some(a), Some(b)) => {
                    if a.len == 0 {
                        return Some(*b);
                    }
                    if b.len == 0 {
                        return Some(*a);
                    }
                    Some(RangeSum {
                        sum: a.sum + b.sum,
                        len: a.len + b.len,
                        max: max(a.max, b.max),
                        max_2nd: second_highest(a.max, a.max_2nd, b.max, b.max_2nd),
                        max_cnt: if a.max > b.max {
                            a.max_cnt
                        } else if a.max < b.max {
                            b.max_cnt
                        } else {
                            a.max_cnt + b.max_cnt
                        },
                        min: min(a.min, b.min),
                        min_2nd: second_lowest(a.min, a.min_2nd, b.min, b.min_2nd),
                        min_cnt: if a.min < b.min {
                            a.min_cnt
                        } else if a.min > b.min {
                            b.min_cnt
                        } else {
                            a.min_cnt + b.min_cnt
                        },
                    })
                }
            }
        }
        fn fails(a: &Self::S) -> bool {
            a.is_none()
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Func {
        pub add: i64,
        pub upper: i64,
        pub lower: i64,
    }
    impl Func {
        pub fn new_add(add: i64) -> Self {
            Func {
                add,
                upper: INF,
                lower: NEG_INF,
            }
        }
        pub fn new_chmin(x: i64) -> Self {
            Func {
                add: 0,
                upper: x,
                lower: NEG_INF,
            }
        }
        pub fn new_chmax(x: i64) -> Self {
            Func {
                add: 0,
                upper: INF,
                lower: x,
            }
        }
    }
    pub struct RangeChminChmaxAddRangeSum(Infallible);
    impl MapMonoidBeats for RangeChminChmaxAddRangeSum {
        type F = Func;
        type M = RangeSumMonoid;
        fn identity_map() -> Self::F {
            Func {
                add: 0,
                upper: INF,
                lower: NEG_INF,
            }
        }
        fn mapping(f: &Self::F, x: &<Self::M as MonoidBeats>::S) -> <Self::M as MonoidBeats>::S {
            match x {
                None => None,
                Some(x) => {
                    if x.len == 0 {
                        return Some(*x);
                    }
                    if x.len == 1 {
                        let mut next_x = *x;
                        if f.add != 0 {
                            next_x.sum += f.add;
                            next_x.max += f.add;
                            next_x.min += f.add;
                        }
                        if next_x.max > f.upper {
                            next_x.max = f.upper;
                            next_x.min = f.upper;
                            next_x.sum = f.upper;
                        }
                        if next_x.min < f.lower {
                            next_x.min = f.lower;
                            next_x.max = f.lower;
                            next_x.sum = f.lower;
                        }
                        return Some(next_x);
                    }
                    let mut next_x = *x;
                    if f.add != 0 {
                        next_x.sum += f.add * (next_x.len as i64);
                        if next_x.max != NEG_INF {
                            next_x.max += f.add;
                        }
                        if next_x.max_2nd != NEG_INF {
                            next_x.max_2nd += f.add;
                        }
                        if next_x.min != INF {
                            next_x.min += f.add;
                        }
                        if next_x.min_2nd != INF {
                            next_x.min_2nd += f.add;
                        }
                    }
                    if next_x.max > f.upper {
                        if next_x.max_2nd < f.upper {
                            next_x.sum -= (next_x.max - f.upper) * (next_x.max_cnt as i64);
                            next_x.max = f.upper;
                            if next_x.min > f.upper {
                                next_x.min = f.upper;
                            }
                            if next_x.min_2nd > f.upper {
                                next_x.min_2nd = f.upper;
                            }
                            if next_x.max == next_x.min {
                                next_x.min = f.upper;
                            }
                        } else {
                            return None;
                        }
                    }
                    if next_x.min < f.lower {
                        if next_x.min_2nd > f.lower {
                            next_x.sum += (f.lower - next_x.min) * (next_x.min_cnt as i64);
                            next_x.min = f.lower;
                            if next_x.max < f.lower {
                                next_x.max = f.lower;
                            }
                            if next_x.max_2nd < f.lower {
                                next_x.max_2nd = f.lower;
                            }
                            if next_x.min == next_x.max {
                                next_x.max = f.lower;
                            }
                        } else {
                            return None;
                        }
                    }
                    Some(next_x)
                }
            }
        }
        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            let add_new = g.add + f.add;
            let upper_new = min(if g.upper == INF { INF } else { g.upper + f.add }, f.upper);
            let lower_new = max(
                min(
                    if g.lower == NEG_INF {
                        NEG_INF
                    } else {
                        g.lower + f.add
                    },
                    f.upper,
                ),
                f.lower,
            );
            Func {
                add: add_new,
                upper: upper_new,
                lower: lower_new,
            }
        }
    }
    pub struct RangeChminChmaxAddRangeSumSegtree {
        segtree: SegtreeBeats<RangeChminChmaxAddRangeSum>,
        len: usize,
    }
    impl RangeChminChmaxAddRangeSumSegtree {
        pub fn new(n: usize) -> Self {
            let segtree = SegtreeBeats::<RangeChminChmaxAddRangeSum>::new(n);
            Self { segtree, len: n }
        }
        pub fn from(xs: &[i64]) -> Self {
            let len = xs.len();
            let segtree = SegtreeBeats::<RangeChminChmaxAddRangeSum>::from(
                xs.iter().copied().map(RangeSum::unit).collect_vec(),
            );
            Self { segtree, len }
        }
        #[allow(clippy::len_without_is_empty)]
        pub fn len(&self) -> usize {
            self.len
        }
        pub fn set(&mut self, p: usize, x: i64) {
            self.segtree.set(p, RangeSum::unit(x));
        }
        pub fn get(&mut self, p: usize) -> i64 {
            self.segtree.get(p).unwrap().sum
        }
        pub fn sum<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().sum
        }
        pub fn min<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().min
        }
        pub fn max<R: RangeBounds<usize>>(&mut self, range: R) -> i64 {
            self.segtree.prod(range).unwrap().max
        }
        pub fn chmax<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_chmax(x));
        }
        pub fn chmin<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_chmin(x));
        }
        pub fn add<R: RangeBounds<usize>>(&mut self, range: R, x: i64) {
            self.segtree.apply_range(range, Func::new_add(x));
        }
        pub fn to_vec(&mut self) -> Vec<i64> {
            (0..self.len).map(|i| self.get(i)).collect_vec()
        }
    }
}
