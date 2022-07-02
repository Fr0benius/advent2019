use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::{
    arr2::Arr2,
    dir::neighbors4,
};

pub fn run(input: &str) -> (i64, i64) {
    let n = input.lines().count();
    let chars: Vec<char> = input.chars().filter(|&c| c != '\n').collect();
    let m = chars.len() / n;
    let g = Arr2::from_raw(n, m, chars);
    let (pt0, _) = g.enumerate().find(|&(_, &c)| c == '@').unwrap();

    let mut q = BinaryHeap::from([Reverse((0, pt0, 0u32))]);
    let mut d = HashMap::from([((pt0, 0u32), 0)]);
    let board = Board::new(g.clone());
    let mut part1 = 0;
    let mut nodes = HashSet::new();
    while let Some(Reverse((d0, pt0, m0))) = q.pop() {
        nodes.insert(pt0);
        if d0 > d[&(pt0, m0)] {
            continue;
        }
        if m0 == (1 << 26) - 1 {
            part1 = d0;
            break;
        }
        'a: for neigh in board.neighbors(pt0) {
            let (_, pt, locks, len) = board.walk(pt0, neigh);
            for k in locks {
                if m0 & (1 << k) == 0 {
                    continue 'a;
                }
            }
            let mut m = m0;
            let c = board.g[pt];
            if c.is_ascii_lowercase() {
                m |= 1 << (c as u8 - b'a');
            }
            // if d.contains_key(&(pt, m)) {
            //     continue;
            // }
            let entry = d.entry((pt, m)).or_insert(i64::MAX);
            if *entry > d0 + len {
                *entry = d0 + len;
                q.push(Reverse((d0 + len, pt, m)));
            }
        }
    }

    let (r0, c0) = pt0;
    let starts = [
        (r0 - 1, c0 - 1),
        (r0 - 1, c0 + 1),
        (r0 + 1, c0 - 1),
        (r0 + 1, c0 + 1),
    ];
    let trees = {
        let mut g = g;
        for pt in starts {
            g[pt] = '@';
        }
        for pt in neighbors4(pt0.0, pt0.1, n, m) {
            g[pt] = '#';
        }
        let board = Board::new(g);
        starts.map(|start| board.build_tree(start))
    };
    for i in 0..4 {
        dbg!(trees[i].iter().filter(|v| v.0.is_ascii_lowercase()).count());
    }
    let mut q = BinaryHeap::from([Reverse((0, [0; 4], 0u32))]);
    let mut d = HashMap::from([(([0; 4], 0u32), 0)]);
    let mut part2 = 0;
    while let Some(Reverse((d0, ids, m0))) = q.pop() {
        if d0 > d[&(ids, m0)] {
            continue;
        }
        if m0 == (1 << 26) - 1 {
            part2 = d0;
            break;
        }
        for i in 0..4 {
            let id0 = ids[i];
            let (_, ref edges) = trees[i][id0];
            'b: for &Edge { to, len, ref locks } in edges {
                for &k in locks {
                    if m0 & (1 << k) == 0 {
                        continue 'b;
                    }
                }
                let c = trees[i][to].0;
                let m = m0
                    | if c.is_ascii_lowercase() {
                        1 << (c as u8 - b'a')
                    } else {
                        0
                    };
                let mut ids = ids;
                ids[i] = to;
                let entry = d.entry((ids, m)).or_insert(i64::MAX);
                if *entry > d0 + len {
                    *entry = d0 + len;
                    q.push(Reverse((d0 + len, ids, m)));
                }
            }
        }
    }

    (part1, part2)
}

struct Board {
    g: Arr2<char>,
}

type Pt = (usize, usize);

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    len: i64,
    locks: Vec<u8>,
}

impl Board {
    fn new(g: Arr2<char>) -> Self {
        Self { g }
    }
    fn neighbors(&self, (r, c): Pt) -> Vec<Pt> {
        let (n, m) = self.g.dims();
        neighbors4(r, c, n, m)
            .filter(|&pt| self.g[pt] != '#')
            .collect()
    }

    fn walk(&self, mut prev: Pt, mut pt: Pt) -> (Pt, Pt, Vec<u8>, i64) {
        let mut locks = vec![];
        let mut d = 1;
        loop {
            let c = self.g[pt];
            if c.is_ascii_uppercase() {
                locks.push(c as u8 - b'A');
            }
            let pts = self.neighbors(pt);
            if pts.len() != 2 || c.is_ascii_lowercase() {
                return (prev, pt, locks, d);
            }
            let next = pts.into_iter().find(|&pt| pt != prev).unwrap();
            (pt, prev) = (next, pt);
            d += 1;
        }
    }
    fn build_tree(&self, start: Pt) -> Vec<(char, Vec<Edge>)> {
        let mut tree: Vec<(char, Vec<Edge>)> = vec![];
        self.dfs(start, start, &mut tree);
        tree
    }

    fn dfs(&self, prev: Pt, pt: Pt, tree: &mut Vec<(char, Vec<Edge>)>) -> Option<usize> {
        let v = tree.len();
        tree.push((self.g[pt], vec![]));
        let mut good: bool = self.g[pt].is_ascii_lowercase();
        for neigh in self.neighbors(pt) {
            if neigh == prev {
                continue;
            }
            let (prev2, next, locks, len) = self.walk(pt, neigh);
            if let Some(w) = self.dfs(prev2, next, tree) {
                tree[v].1.push(Edge { to: w, len, locks });
                tree[w].1.push(Edge {
                    to: v,
                    len,
                    locks: vec![],
                });
                good = true;
            }
        }
        if good {
            Some(v)
        } else {
            None
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    #[ignore] // Part 2 is slow
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/18.txt")), (5198, 1736));
    }
}
