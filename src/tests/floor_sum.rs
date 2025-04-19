use crate::{floor_sum::*, simple_rng::*};

#[test]
fn floor_sum_random() {
    let mut rng = Rng::new(1);
    for _ in 0..100 {
        let n = rng.range(0..=100);
        let m = rng.range(1..=10000);
        let a = rng.range(-10000..=10000);
        let b = rng.range(-10000..=10000);

        let mut naive = 0;
        for i in 0..n {
            fn div_floor(x: i64, y: i64) -> i64 {
                x / y - (x % y < 0) as i64
            }
            naive += div_floor(a * i + b, m);
        }

        assert_eq!(floor_sum(n, m, a, b), naive);
    }
}
