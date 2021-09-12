#[test]
fn ex_groupby() {
    use itertools::Itertools;

    //TODO: asssert_eqで書いてみる。(ysをVecにできないのかな？)
    let xs = vec![1, 2, 3, 3, 5];
    let ys = xs.iter().group_by(|&key| *key);
    for (key, _) in &ys {
        println!("{:?}", key);
    }
}


