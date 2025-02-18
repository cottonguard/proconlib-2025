pub fn scc<F: FnMut(usize) -> A, A: IntoIterator<Item = usize>>(n: usize, adj: F) -> Vec<usize> {
    let mut dfs = Dfs {
        ord: vec![(-1isize) as usize; n],
        low: vec![(-1isize) as usize; n],
        next_ord: isize::MIN as usize,
        stack: vec![],
        comp_id: 0,
        adj,
    };
    for u in (0..n).rev() {
        if dfs.ord[u] == !0 {
            dfs.dfs(u);
        }
    }
    dfs.ord
}

struct Dfs<F> {
    ord: Vec<usize>,
    low: Vec<usize>,
    next_ord: usize,
    stack: Vec<usize>,
    comp_id: usize,
    adj: F,
}

impl<F: FnMut(usize) -> A, A: IntoIterator<Item = usize>> Dfs<F> {
    fn dfs(&mut self, u: usize) {
        self.stack.push(u);
        self.ord[u] = self.next_ord;
        self.low[u] = self.next_ord;
        self.next_ord += 1;
        for v in (self.adj)(u) {
            if self.ord[v] as isize == -1 {
                self.dfs(v);
                self.low[u] = (self.low[u] as isize).min(self.low[v] as isize) as usize;
            } else {
                self.low[u] = (self.low[u] as isize).min(self.ord[v] as isize) as usize;
            }
        }
        dbg!(u, &self.ord, &self.low);
        if self.ord[u] == self.low[u] {
            let i = self.stack.iter().rposition(|&v| v == u).unwrap();
            for &v in &self.stack[i..] {
                self.ord[v] = self.comp_id;
            }
            self.comp_id += 1;
            self.stack.truncate(i);
        }
    }
}
