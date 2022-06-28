use std::{cmp::Ordering, collections::HashSet};

use crate::num::gcd;

pub fn run(input: &str) -> (usize, i64) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let n = grid.len();
    let m = grid[0].len();
    let all_pts: Vec<Pt> = (0..m)
        .flat_map(|x| (0..n).map(move |y| (x as _, y as _)))
        .collect();
    let asteroids: Vec<Pt> = all_pts
        .iter()
        .copied()
        .filter(|&(x, y)| grid[y as usize][x as usize] == '#')
        .collect();
    let (part1, station) = all_pts
        .iter()
        .map(|&pt| {
            let set: HashSet<_> = asteroids
                .iter()
                .map(|&ast| normalize(sub(ast, pt)))
                .collect();
            (set.len() - 1, pt)
        })
        .max()
        .unwrap();

    let mut ord_by_angle: Vec<Pt> = asteroids
        .iter()
        .map(|&ast| sub(ast, station))
        .filter(|&pt| pt != (0, 0))
        .collect();
    ord_by_angle.sort_unstable_by(|&p0, &p1| compare_by_angle(p0, p1));
    let mut ord = vec![(0, 0, (0, 0))];
    for (x, y) in ord_by_angle {
        let &(mut i, mut j, last) = ord.last().unwrap();
        if normalize((x, y)) == normalize(last) {
            i += 1;
        } else {
            j += 1;
            i = 0;
        }
        ord.push((i, j, (x, y)));
    }
    ord.sort_unstable();
    let part2 = {
        let (x, y) = add(ord[200].2, station);
        100 * x + y
    };
    (part1, part2)
}

type Pt = (i64, i64);

fn add((x0, y0): Pt, (x1, y1): Pt) -> Pt {
    (x0 + x1, y0 + y1)
}

fn sub((x0, y0): Pt, (x1, y1): Pt) -> Pt {
    (x0 - x1, y0 - y1)
}

fn cross((x0, y0): Pt, (x1, y1): Pt) -> i64 {
    x0 * y1 - y0 * x1
}

fn dot((x0, y0): Pt, (x1, y1): Pt) -> i64 {
    x0 * x1 + y0 * y1
}

fn left_half((x, y): Pt) -> bool {
    x < 0 || (x == 0) && y > 0
}

fn compare_by_angle(p0: Pt, p1: Pt) -> Ordering {
    if p0 == p1 {
        Ordering::Equal
    } else if (left_half(p0), 0, dot(p0, p0)) < (left_half(p1), cross(p0, p1), dot(p1, p1)) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn normalize((x, y): Pt) -> Pt {
    let d = gcd(x, y);
    if d == 0 {
        (x, y)
    } else {
        (x / d, y / d)
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(
            super::run(include_str!("../input/10-sample.txt")),
            (210, 802)
        );
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/10.txt")), (278, 1417));
    }
}
