use crate::suffix_array::*;

#[test]
fn lcp_test() {
    let s = b"abracadabra";
    let sa = suffix_array_naive(s);
    let lcp = lcp_array(s, &sa);
    assert_eq!(sa.len(), lcp.len() + 1);
    for i in 0..s.len() - 1 {
        assert_eq!(s[sa[i]..sa[i] + lcp[i]], s[sa[i + 1]..sa[i + 1] + lcp[i]]);
        assert_ne!(s.get(sa[i] + lcp[i]), s.get(sa[i + 1] + lcp[i]));
    }
}

#[test]
fn sa_is_test() {
    let ss: &[&[u8]] = &[
        b"",
        b"a",
        b"aaaaa",
        b"abracadabra",
        b"mississippi",
        b"31415926535897932384626433",
    ];
    for s in ss {
        let sa_is = sa_is(s, 255);
        let sa_naive = suffix_array_naive(s);
        assert_eq!(sa_is, sa_naive);
    }
}
