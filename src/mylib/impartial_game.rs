use cargo_snippet::snippet;

#[snippet]
/// 計算量: O(xs.len())
pub fn mex(xs: &[usize]) -> usize {
    let contains = xs
        .iter()
        .copied()
        .fold(vec![false; xs.len()], |mut acc, x| {
            if x < xs.len() {
                acc[x] |= true;
            }
            acc
        });
    contains
        .iter()
        .copied()
        .position(|p| !p)
        .unwrap_or(xs.len())
}

#[snippet(prefix = "use subtraction_game::*;")]
pub mod subtraction_game {
    use itertools::Itertools;

    fn mex(xs: &[usize]) -> usize {
        let contains = xs
            .iter()
            .copied()
            .fold(vec![false; xs.len()], |mut acc, x| {
                if x < xs.len() {
                    acc[x] |= true;
                }
                acc
            });
        contains
            .iter()
            .copied()
            .position(|p| !p)
            .unwrap_or(xs.len())
    }

    /// subtraction_game の 0 から n - 1 までの grundy 数を求める
    pub fn subtraction_game_grundy(subtractions: &[usize], n: usize) -> Vec<usize> {
        let mut grundy = vec![usize::MAX; n];
        for x in 0..n {
            let next_grundy_list = subtractions
                .iter()
                .copied()
                .filter(|sub| x >= *sub)
                .map(|sub| grundy[x - sub])
                .collect_vec();
            grundy[x] = mex(&next_grundy_list)
        }
        grundy
    }
}
#[cfg(test)]
mod test_mex {
    use super::mex;
    #[test]
    fn test_mex() {
        assert_eq!(mex(&[]), 0);
        assert_eq!(mex(&[0]), 1);
        assert_eq!(mex(&[1]), 0);
        assert_eq!(mex(&[0, 1]), 2);
        assert_eq!(mex(&[0, 1, 2]), 3);
        assert_eq!(mex(&[2, 0, 1]), 3);
        assert_eq!(mex(&[0, 0, 0, 2, 2]), 1);
        assert_eq!(mex(&[0, 0, 0, 1, 1, 2, 2]), 3);
        assert_eq!(mex(&[0, 2]), 1);
        assert_eq!(mex(&[0, 1, 100]), 2);
    }
}

#[cfg(test)]
mod test_subtraction_game {

    use itertools::Itertools;

    use super::subtraction_game::*;
    #[test]
    fn test_subtraction_game() {
        assert_eq!(
            subtraction_game_grundy(&[1, 2, 3], 10),
            vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1] // [0, 1, 2, 3] の周期4
        );

        assert_eq!(
            subtraction_game_grundy(&[3, 1, 2], 10), // 順番は関係ない
            vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1]
        );

        assert_eq!(
            subtraction_game_grundy(&[3, 4], 15),
            vec![0, 0, 0, 1, 1, 1, 2, 0, 0, 0, 1, 1, 1, 2, 0] // [0, 0, 0, 1, 1, 1, 2] の周期7
        );

        //dbg!(subtraction_game_grundy(&[1, 3, 4], 40));
        dbg!(&subtraction_game_grundy(&[2, 4, 7], 40).iter().join(", "));

        for _ in 0..1000 {
            //experiment();
        }
        //dbg!(subtraction_game_grundy(&[1, 3, 6], 40));
    }

    // fn experiment() {
    //     use rand::{rngs::SmallRng, seq::SliceRandom, *};
    //     let mut rng = SmallRng::from_os_rng();
    //     // let mut rng = SmallRng::seed_from_u64(42);
    //     let n = 3;
    //     let xs = (0..n).map(|_| rng.random_range(0..8)).collect_vec();
    //     let grundy = subtraction_game_grundy(&xs, 100);
    //     if !is_periodic(&grundy) {
    //         dbg!(xs);
    //         dbg!(grundy);
    //     }
    // }

    // // 雑に判定
    // fn is_periodic(xs: &[usize]) -> bool {
    //     xs[1..].windows(10).any(|sub| *sub == xs[..10])
    // }
}
