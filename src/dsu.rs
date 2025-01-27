use std::mem::MaybeUninit;

type IIdx = i32;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dsu(Vec<IIdx>);

impl Dsu {
    pub fn new(n: usize) -> Self {
        Self(vec![-1; n])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn comp(&self, mut i: usize) -> Comp {
        loop {
            if self.0[i] < 0 {
                return Comp {
                    root: i,
                    size: (-self.0[i]) as usize,
                };
            }
            i = self.0[i] as usize;
        }
    }

    pub fn is_root(&self, i: usize) -> bool {
        self.0[i] < 0
    }

    #[inline]
    pub fn root(&self, i: usize) -> usize {
        self.comp(i).root
    }

    #[inline]
    pub fn size(&self, i: usize) -> usize {
        self.comp(i).size
    }

    #[inline]
    pub fn unite(&mut self, i: usize, j: usize) -> UniteResult {
        let ci = self.comp(i);
        let cj = self.comp(j);

        if ci.root == cj.root {
            return UniteResult {
                is_united: false,
                root: ci.root,
                united_root: ci.root,
                size: ci.size,
            };
        }

        let (r, c) = if ci.size >= cj.size {
            (ci.root, cj.root)
        } else {
            (cj.root, ci.root)
        };
        self.0[r] += self.0[c];
        self.0[c] = r as IIdx;
        UniteResult {
            is_united: true,
            root: r,
            united_root: c,
            size: (-self.0[r]) as usize,
        }
    }

    pub fn comps(&self) -> Comps {
        Comps {
            dsu: self,
            i: 0,
            j: self.len(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Comp {
    pub root: usize,
    pub size: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct UniteResult {
    pub is_united: bool,
    pub root: usize,
    pub united_root: usize,
    pub size: usize,
}

pub struct Comps<'a> {
    dsu: &'a Dsu,
    i: usize,
    j: usize,
}

impl<'a> Iterator for Comps<'a> {
    type Item = Comp;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.j {
            if self.dsu.is_root(self.i) {
                let root = self.i;
                self.i += 1;
                return Some(Comp {
                    root,
                    size: (-self.dsu.0[root]) as usize,
                });
            }
            self.i += 1;
        }
        None
    }
}

#[derive(Debug)]
pub struct DsuMerge<T, F> {
    dsu: Dsu,
    values: Vec<MaybeUninit<T>>,
    merge: F,
}

impl<T, F> DsuMerge<T, F> {
    pub fn len(&self) -> usize {
        self.dsu.len()
    }

    pub fn comp(&self, i: usize) -> (Comp, &T) {
        let comp = self.dsu.comp(i);
        (comp, unsafe { self.values[comp.root].assume_init_ref() })
    }

    pub fn comp_mut(&mut self, i: usize) -> (Comp, &mut T) {
        let comp = self.dsu.comp(i);
        (comp, unsafe { self.values[comp.root].assume_init_mut() })
    }

    pub fn is_root(&self, i: usize) -> bool {
        self.dsu.is_root(i)
    }

    pub fn root(&self, i: usize) -> usize {
        self.dsu.root(i)
    }

    pub fn size(&self, i: usize) -> usize {
        self.dsu.size(i)
    }

    pub fn value(&self, i: usize) -> &T {
        self.comp(i).1
    }

    pub fn value_mut(&mut self, i: usize) -> &mut T {
        self.comp_mut(i).1
    }

    pub fn comps(&self) -> MergeComps<T> {
        MergeComps {
            comps: self.dsu.comps(),
            values: &*self.values,
        }
    }
}

impl<T, F: FnMut(&mut T, T)> DsuMerge<T, F> {
    pub fn new(n: usize, init: T, merge: F) -> Self
    where
        T: Clone,
    {
        Self::new_with(n, |_| init.clone(), merge)
    }

    pub fn new_with(n: usize, mut init: impl FnMut(usize) -> T, merge: F) -> Self {
        Self {
            dsu: Dsu::new(n),
            values: (0..n).map(|i| MaybeUninit::new(init(i))).collect(),
            merge,
        }
    }

    pub fn unite(&mut self, i: usize, j: usize) -> (UniteResult, &mut T) {
        let res = self.dsu.unite(i, j);
        let UniteResult {
            is_united,
            root,
            united_root,
            ..
        } = res;
        if is_united {
            let child_value = unsafe { self.values[united_root].assume_init_read() };
            (self.merge)(unsafe { self.values[root].assume_init_mut() }, child_value);
        }
        (res, unsafe { self.values[root].assume_init_mut() })
    }
}

impl<T: Clone, F: Clone> Clone for DsuMerge<T, F> {
    fn clone(&self) -> Self {
        let mut values = Vec::<MaybeUninit<T>>::with_capacity(self.values.len());
        unsafe {
            values.set_len(self.len());
        }
        for i in 0..self.len() {
            if self.is_root(i) {
                unsafe {
                    values[i].write(self.values[i].assume_init_ref().clone());
                }
            }
        }
        unsafe {
            values.set_len(self.values.len());
        }
        Self {
            dsu: self.dsu.clone(),
            values,
            merge: self.merge.clone(),
        }
    }
}

impl<T, F> Drop for DsuMerge<T, F> {
    fn drop(&mut self) {
        for i in 0..self.len() {
            if self.is_root(i) {
                unsafe {
                    self.values[i].assume_init_read();
                }
            }
        }
    }
}

pub struct MergeComps<'a, T> {
    comps: Comps<'a>,
    values: &'a [MaybeUninit<T>],
}

impl<'a, T> Iterator for MergeComps<'a, T> {
    type Item = (Comp, &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        self.comps
            .next()
            .map(|comp| (comp, unsafe { self.values[comp.root].assume_init_ref() }))
    }
}
