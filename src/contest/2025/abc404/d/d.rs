struct Animal {
    n_park: usize,
    parks: Vec<usize>,
}
fn main() {
    input! {
        n_park: usize,
        n_animal: usize,
        park_costs: [i64; n_park],
    }

    // animal → park
    let animal_to_park = (0..n_animal)
        .map(|_| {
            input! {
                k: usize,
                parks: [Usize1; k],
            }
            Animal {
                n_park: k,
                parks: parks,
            }
        })
        .collect_vec();

    let park_to_animal = animal_to_park.iter().enumerate().fold(
        vec![vec![]; n_park],
        |mut acc, (animal_id, parks)| {
            for &park_id in &parks.parks {
                acc[park_id].push(animal_id);
            }
            acc
        },
    );

    let ans = std::iter::repeat([0, 1, 2])
        .take(10)
        .multi_cartesian_product()
        .filter(|park_cnts| {
            // park_cnts[i]: 動物園 i に行く回数

            let mut animal_cnts = vec![0; n_animal];

            for park_id in 0..n_park {
                for &animal_id in &park_to_animal[park_id] {
                    animal_cnts[animal_id] += park_cnts[park_id];
                }
            }

            animal_cnts.iter().copied().all(|cnt| cnt >= 2)
        })
        .map(|cnts| {
            (0..n_park)
                .map(|i| park_costs[i] * (cnts[i] as i64))
                .sum::<i64>()
        })
        .min()
        .unwrap();

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
}

#[allow(unused)]
fn print_yesno(ans: bool) {
    let msg = if ans { "Yes" } else { "No" };
    println!("{}", msg);
}

// ====== snippet ======
