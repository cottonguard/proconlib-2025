use crate::{integer::*, simple_rng::*};

#[test]
fn ext_gcd_random() {
    let mut rng = Rng::new(1);
    for _ in 0..100 {
        let x = rng.range(-10000..=10000);
        let y = rng.range(-10000..=10000);
        let (g, a, b) = ext_gcd(x, y);
        assert_eq!(a * x + b * y, g, "x={x}, y={y}, a={a}, b={b}");
    }
}

#[test]
fn crt_random() {
    let mut rng = Rng::new(1);
    for _ in 0..1000 {
        let a = rng.range(-10000..=10000);
        let m = rng.range(1..=10000);
        let b = rng.range(-10000..=10000);
        let n = rng.range(1..=10000);
        let res = crt(a, m, b, n);
        if let Some((c, k)) = res {
            assert_eq!(c % m, a.rem_euclid(m), "a={a}, m={m}, b={b}, n={n}");
            assert_eq!(c % n, b.rem_euclid(n), "a={a}, m={m}, b={b}, n={n}");
            assert_eq!(k, lcm(m, n), "a={a}, m={m}, b={b}, n={n}");
        } else {
            assert_ne!(gcd(m, n), 1);
        }
    }
}
