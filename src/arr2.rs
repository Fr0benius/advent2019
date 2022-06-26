use std::ops::Index;
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Arr2<T> {
    n: usize,
    m: usize,
    v: Vec<T>,
}

impl<T> Arr2<T> {
    pub fn from_fn<F: FnMut(usize, usize) -> T>(n: usize, m: usize, mut f: F) -> Self {
        let mut v = Vec::with_capacity(n * m);
        for i in 0..n {
            for j in 0..m {
                v.push(f(i, j));
            }
        }
        Self { n, m, v }
    }
    pub fn from_raw(n: usize, m: usize, v: Vec<T>) -> Self {
        assert_eq!(n * m, v.len());
        Self { n, m, v }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.n, self.m)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.v.iter()
    }
    pub fn col(&self, j: usize) -> impl Iterator<Item = &T> {
        (0..self.n).map(move |i| &self[i][j])
    }
}

impl<T: Clone> Arr2<T> {
    pub fn new(n: usize, m: usize, x: T) -> Self {
        Self {
            n,
            m,
            v: vec![x; n * m],
        }
    }
}

impl<T> Index<usize> for Arr2<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.n);
        &self.v[i * self.m..(i + 1) * self.m]
    }
}

impl<T> IntoIterator for Arr2<T> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.v.into_iter()
    }
}

