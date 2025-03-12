use crate::{cht::*, simple_rng::*};

#[test]
fn cht_random() {
    let mut rng = Rng::new(1);
    for _ in 0..100 {
        let mut lines: Vec<_> = (0..10)
            .map(|_| line(rng.range(-100..=100), rng.range(-10000..=10000)))
            .collect();
        let mut a_min = lines.iter().map(|l| l.a).min().unwrap();
        let mut cht = Cht::from(lines.clone());
        for i in 0..20 {
            let x = rng.range(-10000..=10000);
            let ans = cht.y_min(x);
            let naive = lines.iter().map(|l| l.y(x)).min().unwrap();
            assert_eq!(ans, naive);
            if i % 5 == 0 {
                let l = line(rng.range(-100..=a_min), rng.range(-10000..=10000));
                a_min = a_min.min(l.a);
                lines.push(l);
                cht.push_line(l);
            }
        }
    }
}
