use std::{
    fmt::{self, Debug},
    iter,
    ops::Range,
    ptr, slice,
};

type Idx = u32;
const NIL: Idx = !0;

#[derive(Clone, Default, Debug)]
pub struct AdjListBuilder {
    heads: Vec<Idx>,
    adj: Vec<(Idx, Idx)>,
}

impl AdjListBuilder {
    pub fn new(num_vert: usize) -> Self {
        Self {
            heads: vec![NIL; num_vert],
            ..Default::default()
        }
    }
    pub fn with_capacity(num_vert: usize, cap_edge: usize) -> Self {
        Self {
            heads: vec![NIL; num_vert],
            adj: Vec::with_capacity(cap_edge),
        }
    }
    pub fn num_vert(&self) -> usize {
        self.heads.len()
    }
    pub fn num_edge(&self) -> usize {
        self.adj.len()
    }
    pub fn edge(&mut self, from: usize, to: usize) -> &mut Self {
        assert!(
            from < self.num_vert(),
            "out of bound (from={from}, len={})",
            self.num_vert()
        );
        assert!(
            to < self.num_vert(),
            "out of bound (to={to}, len={})",
            self.num_vert()
        );

        let i = self.adj.len() as Idx;
        self.adj.push((to as Idx, self.heads[from]));
        self.heads[from] = i;
        self
    }
    pub fn biedge(&mut self, u: usize, v: usize) -> &mut Self {
        self.edge(u, v).edge(v, u)
    }
    pub fn build(self) -> AdjList {
        self.build_impl(|_, _| {})
    }
    #[inline]
    fn build_impl(mut self, mut f: impl FnMut(usize, usize)) -> AdjList {
        let mut adj = Vec::with_capacity(self.adj.len());
        let adj_buf = adj.spare_capacity_mut();
        let mut i = adj_buf.len();
        for j in self.heads.iter_mut().rev() {
            while *j != NIL {
                i -= 1;
                let (adj, next);
                unsafe {
                    (adj, next) = *self.adj.get_unchecked(*j as usize);
                    adj_buf.get_unchecked_mut(i).write(adj);
                }
                f(*j as usize, i);
                *j = next;
            }
            *j = i as Idx;
        }
        unsafe {
            adj.set_len(adj.capacity());
        }
        AdjList {
            adj,
            heads: self.heads,
        }
    }
}

#[derive(Clone, Default)]
pub struct AdjList {
    adj: Vec<Idx>,
    heads: Vec<Idx>,
}

impl AdjList {
    pub fn from_edges(num_vert: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let edges = edges.into_iter();
        let mut builder = AdjListBuilder::with_capacity(num_vert, edges.size_hint().0);
        for (from, to) in edges {
            builder.edge(from, to);
        }
        builder.build()
    }
    pub fn from_biedges(num_vert: usize, edges: impl IntoIterator<Item = (usize, usize)>) -> Self {
        let edges = edges.into_iter();
        let mut builder = AdjListBuilder::with_capacity(num_vert, edges.size_hint().0);
        for (u, v) in edges {
            builder.biedge(u, v);
        }
        builder.build()
    }
    pub fn num_vert(&self) -> usize {
        self.heads.len()
    }
    pub fn num_edge(&self) -> usize {
        self.adj.len()
    }
    #[inline]
    fn range(&self, v: usize) -> Range<usize> {
        let start = self.heads[v];
        let end = self
            .heads
            .get(v + 1)
            .copied()
            .unwrap_or(self.adj.len() as u32);
        start as usize..end as usize
    }
    pub fn adj(&self, v: usize) -> Adj {
        Adj(&self.adj[self.range(v)])
    }
    pub fn deg(&self, v: usize) -> usize {
        let r = self.range(v);
        r.end - r.start
    }
    pub fn edges(&self) -> Edges {
        Edges {
            g: self,
            ui: 0,
            i: 0,
            uj: self.num_vert() as Idx,
            j: self.num_edge() as Idx,
        }
    }
}

impl Debug for AdjList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries((0..self.num_vert()).map(|i| (i, &self.adj[self.range(i)])))
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Adj<'a>(&'a [Idx]);

impl Adj<'_> {
    pub fn get(&self, i: usize) -> usize {
        self.0[i] as usize
    }
    pub fn iter(&self) -> AdjIter {
        self.into_iter()
    }
}

impl<'a> IntoIterator for Adj<'a> {
    type IntoIter = AdjIter<'a>;
    type Item = usize;
    fn into_iter(self) -> Self::IntoIter {
        AdjIter {
            inner: self.0.iter(),
        }
    }
}

pub struct AdjIter<'a> {
    inner: slice::Iter<'a, Idx>,
}

impl Iterator for AdjIter<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|to| *to as usize)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.inner.nth(n).map(|to| *to as usize)
    }
    fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.map(|to| *to as usize).fold(init, f)
    }
}
impl DoubleEndedIterator for AdjIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|to| *to as usize)
    }
}
impl ExactSizeIterator for AdjIter<'_> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct Edges<'a> {
    g: &'a AdjList,
    ui: Idx,
    i: Idx,
    uj: Idx,
    j: Idx,
}

impl Iterator for Edges<'_> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.j {
            return None;
        }
        let v = unsafe { *self.g.adj.get_unchecked(self.i as usize) as usize };
        while self
            .g
            .heads
            .get((self.ui + 1) as usize)
            .is_some_and(|&k| self.i >= k)
        {
            self.ui += 1;
        }
        self.i += 1;
        Some((self.ui as usize, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl DoubleEndedIterator for Edges<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.i >= self.j {
            return None;
        }
        self.j -= 1;
        let v = unsafe { *self.g.adj.get_unchecked(self.j as usize) as usize };
        while self
            .g
            .heads
            .get(self.uj as usize)
            // .is_none_or(|&k| self.j < k)
            .map_or(true, |&k| self.j < k)
        {
            self.uj -= 1;
        }
        Some((self.uj as usize, v))
    }
}

impl ExactSizeIterator for Edges<'_> {
    fn len(&self) -> usize {
        (self.j - self.i) as usize
    }
}

#[derive(Clone, Debug)]
pub struct LabeledAdjListBuilder<T> {
    inner: AdjListBuilder,
    labels: Vec<T>,
}

impl<T> LabeledAdjListBuilder<T> {
    pub fn new(num_vert: usize) -> Self {
        Self {
            inner: AdjListBuilder::new(num_vert),
            labels: vec![],
        }
    }
    pub fn with_capacity(num_vert: usize, cap_edge: usize) -> Self {
        Self {
            inner: AdjListBuilder::with_capacity(num_vert, cap_edge),
            labels: Vec::with_capacity(cap_edge),
        }
    }
    pub fn num_vert(&self) -> usize {
        self.inner.num_vert()
    }
    pub fn num_edge(&self) -> usize {
        self.inner.num_edge()
    }
    pub fn edge(&mut self, from: usize, to: usize, label: T) -> &mut Self {
        self.labels.push(label);
        self.inner.edge(from, to);
        self
    }
    pub fn biedge(&mut self, u: usize, v: usize, label: T) -> &mut Self
    where
        T: Clone,
    {
        self.edge(u, v, label.clone()).edge(v, u, label);
        self
    }
    pub fn build(mut self) -> LabeledAdjList<T> {
        unsafe {
            self.labels.set_len(0);
        }
        let mut labels = Vec::with_capacity(self.inner.adj.len());
        let src = self.labels.spare_capacity_mut();
        let dst = labels.spare_capacity_mut();
        let g = self.inner.build_impl(|i, j| unsafe {
            dst.get_unchecked_mut(j)
                .write(ptr::read(src.get_unchecked(i).as_ptr()));
        });
        unsafe {
            labels.set_len(labels.capacity());
        }
        LabeledAdjList { g, labels }
    }
}

#[derive(Clone)]
pub struct LabeledAdjList<T> {
    g: AdjList,
    labels: Vec<T>,
}

impl<T> LabeledAdjList<T> {
    pub fn from_edges(num_vert: usize, edges: impl IntoIterator<Item = (usize, usize, T)>) -> Self {
        let edges = edges.into_iter();
        let mut builder = LabeledAdjListBuilder::with_capacity(num_vert, edges.size_hint().0);
        for (from, to, label) in edges {
            builder.edge(from, to, label);
        }
        builder.build()
    }
    pub fn from_biedges(num_vert: usize, edges: impl IntoIterator<Item = (usize, usize, T)>) -> Self
    where
        T: Clone,
    {
        let edges = edges.into_iter();
        let mut builder = LabeledAdjListBuilder::with_capacity(num_vert, edges.size_hint().0);
        for (u, v, label) in edges {
            builder.biedge(u, v, label);
        }
        builder.build()
    }
    pub fn num_vert(&self) -> usize {
        self.g.num_vert()
    }
    pub fn num_edge(&self) -> usize {
        self.g.num_edge()
    }
    pub fn adj(&self, v: usize) -> Adj {
        self.g.adj(v)
    }
    pub fn outedges(&self, v: usize) -> OutEdges<T> {
        OutEdges {
            adj: &self.g.adj[self.g.range(v)],
            labels: &self.labels[self.g.range(v)],
        }
    }
    pub fn outedges_mut(&mut self, v: usize) -> OutEdgesMut<T> {
        OutEdgesMut {
            adj: &self.g.adj[self.g.range(v)],
            labels: &mut self.labels[self.g.range(v)],
        }
    }
}

macro_rules! iter {
    ($Iter:ty, $Item:ty, $map:ident) => {
        impl<'a, T> Iterator for $Iter {
            type Item = $Item;
            fn next(&mut self) -> Option<Self::Item> {
                self.inner.next().map($map)
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                self.inner.size_hint()
            }
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                self.inner.nth(n).map($map)
            }
            fn fold<B, F>(self, init: B, f: F) -> B
            where
                Self: Sized,
                F: FnMut(B, Self::Item) -> B,
            {
                self.inner.map($map).fold(init, f)
            }
        }
        impl<'a, T> DoubleEndedIterator for $Iter {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.inner.next_back().map($map)
            }
        }
        impl<'a, T> ExactSizeIterator for $Iter {
            fn len(&self) -> usize {
                self.inner.len()
            }
        }
    };
}

pub struct OutEdges<'a, T> {
    adj: &'a [Idx],
    labels: &'a [T],
}

impl<'a, T> OutEdges<'a, T> {
    pub fn get(&self, i: usize) -> Edge<'a, T> {
        Edge {
            to: self.adj[i] as usize,
            label: &self.labels[i],
        }
    }
}

impl<'a, T> IntoIterator for OutEdges<'a, T> {
    type IntoIter = OutEdgesIter<'a, T>;
    type Item = Edge<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        OutEdgesIter {
            inner: self.adj.iter().zip(self.labels.iter()),
        }
    }
}

pub struct OutEdgesIter<'a, T> {
    inner: iter::Zip<slice::Iter<'a, Idx>, slice::Iter<'a, T>>,
}

fn to_edge<'a, T>(e: (&'a Idx, &'a T)) -> Edge<'a, T> {
    Edge {
        to: *e.0 as usize,
        label: e.1,
    }
}

iter!(OutEdgesIter<'a, T>, Edge<'a, T>, to_edge);

pub struct OutEdgesMut<'a, T> {
    adj: &'a [Idx],
    labels: &'a mut [T],
}

impl<'a, T> OutEdgesMut<'a, T> {
    pub fn get(&'a self, i: usize) -> Edge<'a, T> {
        Edge {
            to: self.adj[i] as usize,
            label: &self.labels[i],
        }
    }
    pub fn get_mut(&'a mut self, i: usize) -> EdgeMut<'a, T> {
        EdgeMut {
            to: self.adj[i] as usize,
            label: &mut self.labels[i],
        }
    }
}

impl<'a, T> IntoIterator for OutEdgesMut<'a, T> {
    type IntoIter = OutEdgesIterMut<'a, T>;
    type Item = EdgeMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        OutEdgesIterMut {
            inner: self.adj.iter().zip(self.labels.iter_mut()),
        }
    }
}

pub struct OutEdgesIterMut<'a, T> {
    inner: iter::Zip<slice::Iter<'a, Idx>, slice::IterMut<'a, T>>,
}

fn to_edge_mut<'a, T>(e: (&'a Idx, &'a mut T)) -> EdgeMut<'a, T> {
    EdgeMut {
        to: *e.0 as usize,
        label: e.1,
    }
}

iter!(OutEdgesIterMut<'a, T>, EdgeMut<'a, T>, to_edge_mut);

pub struct Edge<'a, T> {
    pub to: usize,
    pub label: &'a T,
}

pub struct EdgeMut<'a, T> {
    pub to: usize,
    pub label: &'a mut T,
}
