use cargo_snippet::snippet;

#[snippet(prefix = "use rect_add_imos_arbitrary::*;", include = "ab_group")]
#[allow(clippy::module_inception)]
pub mod rect_add_imos_arbitrary {
    use crate::ab_group::ab_group::AbGroup;
    use std::ops::{Bound, RangeBounds};

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct RectAddImosArbitrary<G: AbGroup> {
        h: usize,
        w: usize,
        diff: Vec<Vec<G::S>>,
    }

    impl<G: AbGroup> RectAddImosArbitrary<G> {
        pub fn new(h: usize, w: usize) -> Self {
            let mut diff = Vec::with_capacity(h + 1);
            for _ in 0..=h {
                let mut row = Vec::with_capacity(w + 1);
                for _ in 0..=w {
                    row.push(G::zero());
                }
                diff.push(row);
            }
            Self { h, w, diff }
        }

        pub fn rect_add(
            &mut self,
            y_range: impl RangeBounds<usize>,
            x_range: impl RangeBounds<usize>,
            val: G::S,
        ) {
            let y_range = open_range_bounds(y_range, self.h);
            let x_range = open_range_bounds(x_range, self.w);
            let y1 = y_range.start;
            let y2 = y_range.end;
            let x1 = x_range.start;
            let x2 = x_range.end;

            assert!(y1 <= y2 && y2 <= self.h);
            assert!(x1 <= x2 && x2 <= self.w);

            self.diff[y1][x1] = G::add(&self.diff[y1][x1], &val);
            self.diff[y1][x2] = G::sub(&self.diff[y1][x2], &val);
            self.diff[y2][x1] = G::sub(&self.diff[y2][x1], &val);
            self.diff[y2][x2] = G::add(&self.diff[y2][x2], &val);
        }

        pub fn to_vec(mut self) -> Vec<Vec<G::S>> {
            if self.h == 0 {
                return Vec::new();
            }
            if self.w == 0 {
                return vec![vec![]; self.h];
            }

            // 横方向に累積和
            for y in 0..=self.h {
                for x in 1..=self.w {
                    self.diff[y][x] = G::add(&self.diff[y][x - 1], &self.diff[y][x]);
                }
            }

            // 縦方向に累積和
            for x in 0..=self.w {
                for y in 1..=self.h {
                    self.diff[y][x] = G::add(&self.diff[y - 1][x], &self.diff[y][x]);
                }
            }

            self.diff.truncate(self.h);
            for y in 0..self.h {
                self.diff[y].truncate(self.w);
            }
            self.diff
        }
    }

    fn open_range_bounds(range: impl RangeBounds<usize>, len: usize) -> std::ops::Range<usize> {
        let l = match range.start_bound() {
            Bound::Unbounded => 0,
            Bound::Included(&x) => x,
            Bound::Excluded(&x) => x + 1,
        };
        let r = match range.end_bound() {
            Bound::Unbounded => len,
            Bound::Included(&x) => x + 1,
            Bound::Excluded(&x) => x,
        };
        l..r
    }
}

#[cfg(test)]
mod tests {
    use super::rect_add_imos_arbitrary::*;
    use crate::ab_group::ab_group::AdditiveAbGroup;
    use rand::{Rng, SeedableRng};

    #[test]
    fn test_rect_add_imos_arbitrary_basic() {
        type G = AdditiveAbGroup<i64>;
        let (h, w) = (5, 5);
        let mut imos = RectAddImosArbitrary::<G>::new(h, w);

        assert_eq!(imos.clone().to_vec(), vec![vec![0; w]; h]);

        imos.rect_add(1..3, 1..3, 5);
        let expected1 = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(imos.clone().to_vec(), expected1);

        imos.rect_add(0..2, 0..2, -3);
        let expected2 = vec![
            vec![-3, -3, 0, 0, 0],
            vec![-3, 2, 5, 0, 0],
            vec![0, 5, 5, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
        assert_eq!(imos.clone().to_vec(), expected2);
    }

    #[test]
    fn test_empty() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = RectAddImosArbitrary::<G>::new(0, 0);
        assert_eq!(imos.clone().to_vec(), Vec::<Vec<i64>>::new());
        imos.rect_add(0..0, 0..0, 10);
        assert_eq!(imos.to_vec(), Vec::<Vec<i64>>::new());

        let imos = RectAddImosArbitrary::<G>::new(5, 0);
        assert_eq!(imos.to_vec(), vec![Vec::<i64>::new(); 5]);
    }

    #[test]
    fn test_open_range_bounds_coverage() {
        type G = AdditiveAbGroup<i64>;
        let mut imos = RectAddImosArbitrary::<G>::new(5, 5);
        use std::ops::Bound;
        // Excluded start, Excluded end
        imos.rect_add((Bound::Excluded(0), Bound::Excluded(2)), (Bound::Excluded(0), Bound::Excluded(2)), 1);
        let res = imos.to_vec();
        assert_eq!(res[1][1], 1);
        assert_eq!(res[0][0], 0);
    }

    #[test]
    fn test_random_rect_add_imos_arbitrary() {
        type G = AdditiveAbGroup<i64>;
        let mut rng = rand::rngs::SmallRng::seed_from_u64(42);

        for _ in 0..100 {
            let h = rng.random_range(1..=50);
            let w = rng.random_range(1..=50);
            let mut naive_vec = vec![vec![0i64; w]; h];
            let mut imos = RectAddImosArbitrary::<G>::new(h, w);

            for _ in 0..100 {
                let y1 = rng.random_range(0..=h);
                let y2 = rng.random_range(y1..=h);
                let x1 = rng.random_range(0..=w);
                let x2 = rng.random_range(x1..=w);
                let val = rng.random_range(-1000..=1000);

                for y in y1..y2 {
                    for x in x1..x2 {
                        naive_vec[y][x] += val;
                    }
                }
                imos.rect_add(y1..y2, x1..x2, val);
            }
            assert_eq!(imos.to_vec(), naive_vec);
        }
    }
}
