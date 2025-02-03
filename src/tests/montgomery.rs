use crate::{montgomery::*, simple_rng::Rng};

#[test]
fn random() {
    let mut rng = Rng::new(204);
    for _ in 0..100 {
        let m = rng.next_u32() % (1 << 31) | 1;
        let x = rng.next_u32() % m;
        let y = rng.next_u32() % m;
        assert_eq!(
            MontgomeryReduction::<u32>::new(m).mul(x, y),
            (x as u64 * y as u64 % m as u64) as u32,
            "m={m}, x={x}, y={y}"
        );
    }
}

#[test]
fn random64() {
    let mut rng = Rng::new(204);
    for _ in 0..100 {
        let m = rng.next_u64() % (1 << 63) | 1;
        let x = rng.next_u64() % m;
        let y = rng.next_u64() % m;
        assert_eq!(
            MontgomeryReduction::<u64>::new(m).mul(x, y),
            (x as u128 * y as u128 % m as u128) as u64,
            "m={m}, x={x}, y={y}"
        );
    }
}
