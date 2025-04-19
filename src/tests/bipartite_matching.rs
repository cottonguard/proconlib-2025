use std::collections::HashSet;

use crate::{bipartite_matching::*, simple_rng::*};

#[test]
fn bipartite_matching_random() {
    let mut rng = Rng::new(5656);

    let mut a: Vec<usize> = (0..50).collect();
    let mut b: Vec<usize> = (0..50).collect();

    for _ in 0..100 {
        let n = a.len();
        rng.shuffle(&mut a);
        rng.shuffle(&mut b);
        let k = rng.range(..=a.len().min(b.len()));
        let p = rng.range(..=k);
        let q = rng.range(k..=n);
        let mut bm = BipartiteMatching::new(a.len(), b.len());
        let mut edges = HashSet::new();
        for i in 0..k {
            bm.edge(a[i], b[i]);
            edges.insert((a[i], b[i]));
        }
        for _ in 0..rng.range(0..300) {
            let i = rng.range(..n);
            let (i, j) = if i < p {
                (i, rng.range(..p))
            } else if i < k {
                (i, rng.range(p..k))
            } else if i < q {
                if p == 0 {
                    continue;
                }
                (rng.range(..p), i)
            } else {
                if k - p == 0 {
                    continue;
                }
                (i, rng.range(p..k))
            };
            bm.edge(a[i], b[j]);
            edges.insert((a[i], b[j]));
        }
        let matches: Vec<_> = bm.run().collect();
        assert_eq!(matches.len(), k);
        assert!(
            matches.iter().all(|e| edges.contains(e)),
            "matches={matches:?}, edges={edges:?}"
        );
        for i in 0..matches.len() {
            for j in i + 1..matches.len() {
                assert_ne!(matches[i].0, matches[j].0);
                assert_ne!(matches[i].1, matches[j].1);
            }
        }
    }
}
