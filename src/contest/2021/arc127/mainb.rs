// 解けず
#![allow(clippy::let_unit_value)]
use itertools::Itertools;
use proconio::input;

//------snippet------

//-------------------

fn read() -> (i64, i64) {
    input! {
        //from OnceSource::from(""),
        n: i64, l:i64
    }
    (n, l)
}

fn inc(vec: &[i32]) -> Vec<i32> {
    let mut ans = vec.to_vec();
    ans[0] = (ans[0] + 1) % 3;
    let _added = (ans[0] + 1) / 3; //繰り上がり
    ans[0] += 1;
    for i in 0..(vec.len() - 1) {
        ans[i + 1] = ans[i] / 3;
        ans[i] %= 3;
    }
    ans
}

fn solve(n: i64, l: i64) {
    let l = l as usize;
    let mut idx_list = vec![0; l];

    let mut ans: Vec<Vec<i32>> = Vec::new();
    let mut remain_list = vec![[n, n, n]; l];

    //最後リバースする
    idx_list[l - 1] = 2;
    for _ in 0..n {
        for li in 0..l {
            //idx_listを見る
            ans.push(idx_list.clone());
            remain_list[li][idx_list[li] as usize] -= 1;
        }

        idx_list = inc(&idx_list);
    }

    //remain_list[0..l][0..2n]
    let remain_list = remain_list
        .iter()
        .map(|remain| {
            remain
                .iter()
                .flat_map(|i| vec![i + 1; remain[*i as usize] as usize])
                .collect_vec()
        })
        .collect_vec();

    for i in 0..(2 * n) {
        let i = i as usize;
        let mut added = vec![0_i32; l as usize];
        for li in 0..l {
            added[li] = remain_list[li][i] as i32;
        }
        ans.push(added);
    }

    for mut ans_e in ans {
        ans_e.reverse();
        println!("{}", ans_e.iter().map(|i| i.to_string()).join(""));
    }
}

//fn output() {}

fn main() {
    let (n, l) = read();
    solve(n, l);
    //output();
    //println!("{}", ans);
}
