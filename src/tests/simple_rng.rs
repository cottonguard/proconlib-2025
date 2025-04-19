use crate::simple_rng::*;

#[test]
fn range() {
    fn case<T: Ord + Copy>(mut f: impl FnMut() -> T, l: T, r: T) {
        let mut fl = false;
        let mut fr = false;
        for _ in 0..100 {
            let x = f();
            fl |= x == l;
            fr |= x == r;
            assert!(l <= x);
            assert!(x <= r);
        }
        assert!(fl);
        assert!(fr);
    }

    let mut rng = Rng::new(1);

    case(|| rng.range(-5..=5), -5, 5);
    case(|| rng.range(-5..5), -5, 4);
    case(|| rng.range(..=i32::MIN + 5), i32::MIN, i32::MIN + 5);
    case(|| rng.range(i32::MAX - 5..), i32::MAX - 5, i32::MAX);
    case(|| rng.range(..=5u32), 0, 5);
    case(|| rng.range(u32::MAX - 5..), u32::MAX - 5, u32::MAX);
}
