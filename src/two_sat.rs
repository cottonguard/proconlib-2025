use crate::{adj_list::*, scc::*};

#[derive(Clone, Debug)]
pub struct TwoSat {
    g: AdjListBuilder,
}

impl TwoSat {
    pub fn new(n: usize) -> Self {
        Self {
            g: AdjListBuilder::new(2 * n),
        }
    }

    #[inline]
    pub fn clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        let u = 2 * i + f as usize;
        let v = 2 * j + g as usize;
        self.g.edge(u ^ 1, v);
        self.g.edge(v ^ 1, u);
    }

    pub fn solve(self) -> Option<Vec<bool>> {
        let g = self.g.build();
        let n = g.num_vert() / 2;
        let scc = scc(g.num_vert(), |u| g.adj(u));
        dbg!(&scc);
        let mut res = vec![false; n];
        for (i, res) in res.iter_mut().enumerate() {
            let f = 2 * i;
            let t = f + 1;
            if scc[f] == scc[t] {
                return None;
            }
            *res = scc[f] > scc[t];
        }
        Some(res)
    }
}
