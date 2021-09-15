#[test]
fn ex_groupby() {
    use itertools::Itertools;

    //TODO: asssert_eqで書いてみる。(ysをVecにできないのかな？)
    let xs = vec![1, 3, 3, 5, 3];
    let ys = xs.iter().group_by(|&key| *key);
    for (key, group) in &ys {
        dbg!(key);
        dbg!(group.collect::<Vec<_>>());
    }
}
