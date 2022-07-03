use std::collections::HashSet;

use crate::{dir::neighbors4, cellular_auto::{CellAuto, CellAutoSpec}};

pub fn run(input: &str) -> (i64, usize) {
    let init: HashSet<_> = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(j, c)| if c == '#' { Some((i, j)) } else { None })
        })
        .collect();
    let part1 = {
        let mut seen = HashSet::new();
        let mut auto = CellAuto::<SimpleBug> {
            cells: init.clone(),
        };
        loop {
            let sig = auto
                .cells
                .iter()
                .map(|&(x, y)| 1 << (x * 5 + y))
                .sum::<i64>();
            if !seen.insert(sig) {
                break sig;
            }
            auto = auto.next();
        }
    };

    let part2 = {
        let mut auto = CellAuto::<RecursiveBug> {
            cells: init.iter().map(|&(x, y)| (x, y, 0)).collect(),
        };
        for _ in 0..200 {
            auto = auto.next();
        }
        auto.cells.len()
    };
    (part1, part2)
}
struct SimpleBug {}
impl CellAutoSpec for SimpleBug {
    type T = (usize, usize);

    fn neighbors(&(x, y): &Self::T) -> Vec<Self::T> {
        neighbors4(x, y, 5, 5).collect()
    }

    fn rule(alive: bool, alive_neighbors: usize) -> bool {
        if alive {
            alive_neighbors == 1
        } else {
            [1, 2].contains(&alive_neighbors)
        }
    }
}

struct RecursiveBug {}
impl CellAutoSpec for RecursiveBug {
    type T = (usize, usize, i64);

    fn neighbors(&(x, y, lvl): &Self::T) -> Vec<Self::T> {
        let mut v: Vec<Self::T> = neighbors4(x, y, 5, 5)
            .filter(|&c| c != (2, 2))
            .map(|(x, y)| (x, y, lvl))
            .collect();
        if x == 0 {
            v.push((1, 2, lvl - 1));
        }
        if y == 0 {
            v.push((2, 1, lvl - 1));
        }
        if x == 4 {
            v.push((3, 2, lvl - 1));
        }
        if y == 4 {
            v.push((2, 3, lvl - 1));
        }
        if (x, y) == (1, 2) {
            v.extend((0..5).map(|i| (0, i, lvl + 1)));
        }
        if (x, y) == (2, 1) {
            v.extend((0..5).map(|i| (i, 0, lvl + 1)));
        }
        if (x, y) == (3, 2) {
            v.extend((0..5).map(|i| (4, i, lvl + 1)));
        }
        if (x, y) == (2, 3) {
            v.extend((0..5).map(|i| (i, 4, lvl + 1)));
        }
        v
    }

    fn rule(alive: bool, alive_neighbors: usize) -> bool {
        if alive {
            alive_neighbors == 1
        } else {
            [1, 2].contains(&alive_neighbors)
        }
    }
}
#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/24.txt")),
            (18_375_063, 1959)
        );
    }
}
