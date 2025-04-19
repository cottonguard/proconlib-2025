use std::collections::VecDeque;

pub struct BipartiteMatching {
    g: Vec<Vec<usize>>,
    h: Vec<usize>,
}

impl BipartiteMatching {
    pub fn new(n: usize, m: usize) -> Self {
        Self {
            g: vec![vec![]; m],
            h: vec![!0; n],
        }
    }

    pub fn edge(&mut self, u: usize, v: usize) -> &mut Self {
        assert!(u < self.h.len());
        self.g[v].push(u);
        self
    }

    pub fn run(&mut self) -> impl Iterator<Item = (usize, usize)> {
        let mut dist = vec![0; self.g.len()];

        for u in 0..self.g.len() {
            for &x in &self.g[u] {
                if self.h[x] == !0 {
                    self.h[x] = u;
                    dist[u] = !0;
                    break;
                }
            }
        }

        let mut que = VecDeque::new();

        loop {
            que.clear();
            for u in 0..self.g.len() {
                if dist[u] == 0 {
                    que.push_back(u);
                } else {
                    dist[u] = !0;
                }
            }

            while let Some(u) = que.pop_front() {
                for &x in &self.g[u] {
                    let v = self.h[x];
                    if v == !0 {
                        break;
                    }
                    if dist[v] == !0 {
                        que.push_back(v);
                        dist[v] = dist[u] + 1;
                    }
                }
            }

            let mut dfs = Dfs {
                g: &self.g,
                h: &mut self.h,
                dist: &mut dist,
            };
            let mut matched = false;
            for u in 0..self.g.len() {
                if dfs.dist[u] == 0 {
                    matched |= dfs.dfs(u);
                }
            }

            if !matched {
                break;
            }
        }

        self.matches()
    }

    pub fn matches(&self) -> impl Iterator<Item = (usize, usize)> {
        self.h
            .iter()
            .enumerate()
            .filter_map(|(u, &v)| (v < self.g.len()).then_some((u, v)))
    }
}

struct Dfs<'a> {
    g: &'a [Vec<usize>],
    h: &'a mut [usize],
    dist: &'a mut [u32],
}

impl Dfs<'_> {
    fn dfs(&mut self, u: usize) -> bool {
        for &x in &self.g[u] {
            let v = self.h[x];
            if v == !0 {
                self.h[x] = u;
                self.dist[u] = !0;
                return true;
            }
            if self.dist[u] + 1 == self.dist[v] {
                continue;
            }
            if self.dfs(v) {
                self.h[x] = u;
                self.dist[u] = !0;
                return true;
            }
        }
        false
    }
}
