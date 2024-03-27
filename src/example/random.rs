#[test]
fn rand() {
    use rand::{rngs::SmallRng, seq::SliceRandom, *};
    let mut rng = SmallRng::from_entropy();

    rng.gen_range(0..10); // 0..10 の一様乱数
    rng.gen_bool(0.3); // 30% の確率で true
    rng.gen_ratio(2, 5); // 2/5 の確率で true
    let mut _y = vec![1, 2, 3];
    _y.shuffle(&mut rng);
}
