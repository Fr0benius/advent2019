use std::collections::{HashMap, VecDeque};

use crate::{arr2::Arr2, dir::neighbors4};

type Pt = (usize, usize);
pub fn run(input: &str) -> (i64, i64) {
    let chars: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let grid = Arr2::from(chars);
    let (n, m) = grid.dims();
    let (pt_to_label, label_to_pts) = {
        let mut pt_to_label: HashMap<Pt, String> = HashMap::new();
        let mut label_to_pts: HashMap<String, Vec<Pt>> = HashMap::new();
        for i in 2..n - 2 {
            for j in 2..m - 2 {
                let c = grid[i][j];
                if c != '.' {
                    continue;
                }
                for (c1, c2) in [
                    (grid[i - 2][j], grid[i - 1][j]),
                    (grid[i][j - 2], grid[i][j - 1]),
                    (grid[i + 1][j], grid[i + 2][j]),
                    (grid[i][j + 1], grid[i][j + 2]),
                ] {
                    if c1.is_uppercase() && c2.is_uppercase() {
                        let s: String = [c1, c2].into_iter().collect();
                        pt_to_label.insert((i, j), s.clone());
                        label_to_pts.entry(s).or_default().push((i, j));
                        break;
                    }
                }
            }
        }
        (pt_to_label, label_to_pts)
    };
    (
        part1(&grid, &pt_to_label, &label_to_pts),
        part2(&grid, &pt_to_label, &label_to_pts),
    )
}

fn part1(
    grid: &Arr2<char>,
    pt_to_label: &HashMap<(usize, usize), String>,
    label_to_pts: &HashMap<String, Vec<(usize, usize)>>,
) -> i64 {
    let (n, m) = grid.dims();
    let start = label_to_pts[&"AA".to_owned()][0];
    let end = label_to_pts[&"ZZ".to_owned()][0];
    let mut q = VecDeque::from([start]);
    let mut d = HashMap::from([(start, 0)]);
    while let Some(v) = q.pop_front() {
        let d0 = d[&v];
        if v == end {
            return d0;
        }
        let mut neighbors: Vec<Pt> = neighbors4(v.0, v.1, n, m)
            .filter(|&pt| grid[pt] == '.')
            .collect();
        if let Some(label) = pt_to_label.get(&v) {
            let pts = &label_to_pts[label];
            if pts.len() == 2 {
                neighbors.push(if pts[0] == v { pts[1] } else { pts[0] });
            }
        }
        for w in neighbors {
            let entry = d.entry(w).or_insert(i64::MAX);
            if *entry > d0 + 1 {
                *entry = d0 + 1;
                q.push_back(w);
            }
        }
    }
    0
}

fn part2(
    grid: &Arr2<char>,
    pt_to_label: &HashMap<(usize, usize), String>,
    label_to_pts: &HashMap<String, Vec<(usize, usize)>>,
) -> i64 {
    let (n, m) = grid.dims();
    let start = label_to_pts[&"AA".to_owned()][0];
    let end = label_to_pts[&"ZZ".to_owned()][0];
    let mut q = VecDeque::from([(start, 0)]);
    let mut d = HashMap::from([((start, 0), 0)]);
    while let Some(v) = q.pop_front() {
        let d0 = d[&v];
        if v == (end, 0) {
            return d0;
        }
        let (pt, lvl) = v;
        let mut neighbors: Vec<_> = neighbors4(pt.0, pt.1, n, m)
            .filter(|&x| grid[x] == '.')
            .map(|x| (x, lvl))
            .collect();
        if let Some(label) = pt_to_label.get(&pt) {
            let pts = &label_to_pts[label];
            if pts.len() == 2 {
                let other = if pts[0] == pt { pts[1] } else { pts[0] };
                if pt.0 != 2 && pt.1 != 2 && pt.0 != n - 3 && pt.1 != m - 3 {
                    neighbors.push((other, lvl + 1));
                } else if lvl > 0 {
                    neighbors.push((other, lvl - 1));
                }
            }
        }
        for w in neighbors {
            let entry = d.entry(w).or_insert(i64::MAX);
            if *entry > d0 + 1 {
                *entry = d0 + 1;
                q.push_back(w);
            }
        }
    }
    0
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(super::run(include_str!("../input/20-sample.txt")), (77, 396));
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/20.txt")), (462, 5288));
    }
}
