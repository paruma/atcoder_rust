#[test]
fn rand() {
    use rand::{rngs::SmallRng, seq::SliceRandom, *};
    let mut rng = SmallRng::from_os_rng();
    // let mut rng = SmallRng::seed_from_u64(42);

    rng.random_range(0..10); // 0..10 の一様乱数
    rng.random_range(0.0..1.0); // 0.0..1.0 の一様乱数
    rng.gen_bool(0.3); // 30% の確率で true
    rng.gen_ratio(2, 5); // 2/5 の確率で true
    let mut _y = [1, 2, 3];
    _y.shuffle(&mut rng);
}
