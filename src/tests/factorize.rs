use crate::factorize::*;

#[test]
fn test() {
    let small = [
        2, 2, 2, 3, 5, 7, 11, 13, 101, 478067, 161957, 161957, 161957, 288209,
    ];
    let large = [984032359, 104089577, 776603077, 978490013, 372883961];
    for i in 0..small.len() {
        for j in i + 1..small.len() {
            for k in j + 1..small.len() {
                let n = small[i] * small[j] * small[k];
                let mut res = factorize(n);
                res.sort();
                let mut expected = [small[i], small[j], small[k]];
                expected.sort();
                assert_eq!(res, expected);
            }
        }
    }
    for i in 0..large.len() {
        for j in i + 1..large.len() {
            let n = large[i] * large[j];
            let mut res = factorize(n);
            res.sort();
            let mut expected = [large[i], large[j]];
            expected.sort();
            assert_eq!(res, expected);
        }
    }
}
