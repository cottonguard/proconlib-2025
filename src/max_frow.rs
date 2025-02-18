use std::{
    collections::VecDeque,
    ops::{Add, Sub},
};

type Idx = u32;

pub struct MaxFrow<T> {
    edges: Vec<Edge<T>>,
    heads: Vec<Idx>,
}

struct Edge<T> {
    next: Idx,
    v: Idx,
    cap: T,
}

const NIL: Idx = !0;

pub trait Flow: Copy + Ord + Add<Output = Self> + Sub<Output = Self> {
    const ZERO: Self;
}

macro_rules! int {
    ($ty:ident) => {
        impl Flow for $ty {
            const ZERO: Self = 0;
        }
    };
}

int!(i32);
int!(i64);

impl<T: Flow> MaxFrow<T> {
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![],
            heads: vec![NIL; n],
        }
    }

    pub fn num_verts(&self) -> usize {
        self.heads.len()
    }

    fn arc(&mut self, u: usize, v: usize, cap: T) {
        let next = self.heads[u];
        self.heads[u] = self.edges.len() as Idx;
        self.edges.push(Edge {
            next,
            v: v as Idx,
            cap,
        });
    }

    pub fn edge(&mut self, u: usize, v: usize, cap: T) {
        self.arc(u, v, cap);
        self.arc(v, u, T::ZERO);
    }

    pub fn flow(&mut self, s: usize, t: usize) -> T {
        self.flow_(s, t, None)
    }

    pub fn flow_limit(&mut self, s: usize, t: usize, limit: T) -> T {
        self.flow_(s, t, Some(limit))
    }

    fn flow_(&mut self, s: usize, t: usize, limit: Option<T>) -> T {
        Run {
            dist: vec![],
            next_edge: vec![],
            t: t as Idx,
            edges: &mut self.edges,
            heads: &self.heads,
        }
        .run(s, t, limit)
    }
}

struct Run<'a, T> {
    edges: &'a mut [Edge<T>],
    heads: &'a [Idx],
    dist: Vec<Idx>,
    next_edge: Vec<Idx>,
    t: Idx,
}

impl<T: Flow> Run<'_, T> {
    fn run(&mut self, s: usize, t: usize, limit: Option<T>) -> T {
        let mut total = T::ZERO;
        let mut que = VecDeque::new();
        while limit.is_none_or(|limit| total < limit) {
            self.dist.clear();
            self.dist.resize(self.heads.len(), !0);
            self.dist[s] = 0;
            que.clear();
            que.push_back(s as Idx);
            let mut reached = false;
            'bfs: while let Some(u) = que.pop_front() {
                let mut i = self.heads[u as usize];
                while i != NIL {
                    let Edge { next, v, cap } = self.edges[i as usize];
                    if cap > T::ZERO && self.dist[v as usize] == !0 {
                        self.dist[v as usize] = self.dist[u as usize] + 1;
                        que.push_back(v);
                        if v as usize == t {
                            reached = true;
                            break 'bfs;
                        }
                    }
                    i = next;
                }
            }
            if !reached {
                break;
            }
            self.next_edge.clear();
            self.next_edge.extend_from_slice(self.heads);
            let mut add = self.dfs(s);
            if add == T::ZERO {
                continue;
            }
            if let Some(limit) = limit {
                add = add.min(limit - total);
            }
            total = total + add;
            let mut u = s;
            while u != t {
                let i = self.next_edge[u] as usize;
                let j = i ^ 1;
                self.edges[i].cap = self.edges[i].cap - add;
                self.edges[j].cap = self.edges[j].cap + add;
                u = self.edges[i].v as usize;
            }
        }
        total
    }

    fn dfs(&mut self, u: usize) -> T {
        while self.next_edge[u] != NIL {
            let Edge { next, v, cap } = self.edges[self.next_edge[u] as usize];
            if cap > T::ZERO && self.dist[u] + 1 == self.dist[v as usize] {
                if v == self.t {
                    return cap;
                }
                let add = self.dfs(v as usize);
                if add > T::ZERO {
                    return add.min(cap);
                }
            }
            self.next_edge[u] = next;
        }
        self.dist[u] = !0;
        T::ZERO
    }
}
