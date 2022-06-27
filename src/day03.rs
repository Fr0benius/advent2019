use std::collections::HashMap;

pub fn run(input: &str) -> (i64, i64) {
    let (a, b) = {
        let mut pts = input
            .lines()
            .map(|line| line.split(',').fold(vec![((0, 0), 0)], next));
        (pts.next().unwrap(), pts.next().unwrap())
    };
    let map: HashMap<(i64, i64), i64> = b.iter().copied().collect();
    let overlap: Vec<_> = a
        .iter()
        .filter(|&&(p, _)| p != (0, 0) && map.contains_key(&p))
        .collect();

    let part1 = overlap.iter().map(|&((a, b), _)| a.abs() + b.abs()).min().unwrap();

    let part2 = overlap.iter().map(|(p, k)| k + map[p]).min().unwrap();
    (part1, part2)
}

type Pt = ((i64, i64), i64);

fn next(mut pts: Vec<Pt>, inst: &str) -> Vec<Pt> {
    let (c, k) = inst.split_at(1);
    let (c, k): (char, i64) = (c.chars().next().unwrap(), k.parse().unwrap());
    let (dx, dy) = match c {
        'U' => (0, 1),
        'R' => (1, 0),
        'D' => (0, -1),
        'L' => (-1, 0),
        _ => unreachable!(),
    };
    for _ in 0..k {
        let &((x, y), j) = pts.last().unwrap();
        pts.push(((x + dx, y + dy), j + 1));
    }
    pts
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(
            super::run(include_str!("../input/03-sample.txt")),
            (159, 610)
        );
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/03.txt")), (489, 93654));
    }
}
