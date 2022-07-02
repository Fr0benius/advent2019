use std::collections::HashMap;

use crate::intcode::State;

pub fn run(input: &str) -> (i64, usize) {
    const B: usize = 100;
    let init = State::from(input);
    let mut memo = HashMap::new();
    let mut check = |i, j| {
        if let Some(&x) = memo.get(&(i, j)) {
            return x;
        }
        let mut prog = init.clone();
        prog.send_input(i);
        prog.send_input(j);
        let x = prog.get_output().unwrap();
        memo.insert((i, j), x);
        x
    };
    let part1 = (0..50)
        .map(|i| (0..50).map(|j| check(i, j)).sum::<i64>())
        .sum();

    let find_bounds = |x| {
        if x < 1000 {
            return (0, 0);
        }

        let l = (x..).find(|&y| check(x, y) == 1).unwrap();
        let mut r = l;
        for i in (0..15).rev() {
            if check(x, r + (1 << i)) == 1 {
                r += 1 << i;
            }
        }
        (l as usize, r as usize)
    };
    let bounds: Vec<_> = (0..2200).map(find_bounds).collect();
    let check_square = |x: usize, y| {
        let (l, r) = bounds[x];
        y >= l && y + B - 1 <= r && bounds[x + B - 1].0 <= y
    };
    let mut part2 = 0;
    'a: for i in 1000..2000 {
        for j in bounds[i].0..bounds[i].1 {
            if check_square(i, j) {
                part2 = i * 10_000 + j;
                break 'a;
            }
        }
    }
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    #[ignore] // Kind of slow
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/19.txt")),
            (110, 17_302_065)
        );
    }
}
