pub fn cartesian_tree_up<T: Ord>(a: &[T]) -> Vec<usize> {
    let mut p = vec![!0; a.len()];
    cartesian_tree_impl(a, |v, c, _| p[c] = v);
    p
}

pub fn cartesian_tree_down<T: Ord>(a: &[T]) -> Vec<(usize, usize)> {
    let mut t = vec![(!0, !0); a.len()];
    cartesian_tree_impl(a, |u, c, right| {
        if right {
            t[u].1 = c;
        } else {
            t[u].0 = c;
        }
    });
    t
}

fn cartesian_tree_impl<T: Ord>(a: &[T], mut f: impl FnMut(usize, usize, bool)) {
    let mut stk: Vec<usize> = vec![];
    for (i, x) in a.iter().enumerate() {
        let mut c = None;
        while let Some(&k) = stk.last() {
            if a[k] <= *x {
                break;
            }
            c = Some(k);
            stk.pop();
            let Some(&j) = stk.last() else {
                break;
            };
            f(j, k, true);
        }
        if let Some(c) = c {
            f(i, c, false)
        }
        stk.push(i);
    }
    for w in stk.windows(2).rev() {
        f(w[0], w[1], true);
    }
}
