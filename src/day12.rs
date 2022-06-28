use std::collections::HashMap;

use crate::{parsing::re_parser, num::lcm};

pub fn run(input: &str) -> (i64, i64) {
    let init = {
        let mut x = [vec![], vec![], vec![]];
        {
            let parse = re_parser(r"<x=(.*), y=(.*), z=(.*)>");
            for line in input.lines() {
                let (a, b, c): (i64, i64, i64) = parse(line);
                x[0].push(a);
                x[1].push(b);
                x[2].push(c);
            }
        }
        x
    };
    let n = init[0].len();
    let part1 = {
        let mut x = init.clone();
        let mut v = [vec![0; n], vec![0; n], vec![0; n]];
        for i in 0..3 {
            for _ in 0..1000 {
                step(&mut x[i], &mut v[i]);
            }
        }
        (0..n)
            .map(|j| energy(&[x[0][j], x[1][j], x[2][j]]) * energy(&[v[0][j], v[1][j], v[2][j]]))
            .sum()
    };

    let (start, period) = (0..3)
        .fold((0, 1), |(st, per), i| {
            let (a, b) = find_repeat(&init[i]);
            (st.max(a), lcm(per, b - a))
        });
    (part1, start + period)
}

fn energy(x: &[i64]) -> i64 {
    x.iter().map(|a| a.abs()).sum()
}

fn step(x: &mut [i64], v: &mut [i64]) {
    let n = x.len();
    for i in 0..n {
        for j in i + 1..n {
            let dir = (x[j] - x[i]).signum();
            v[i] += dir;
            v[j] -= dir;
        }
    }
    for (x, &vel) in x.iter_mut().zip(v.iter()) {
        *x += vel;
    }
}

fn find_repeat(x: &[i64]) -> (i64, i64) {
    let mut history: HashMap<(Vec<i64>, Vec<i64>), i64> = HashMap::new();
    let mut x = x.to_vec();
    let mut v = vec![0; x.len()];
    for i in 0.. {
        if let Some(j) = history.insert((x.clone(), v.clone()), i) {
            return (j, i);
        }
        step(&mut x, &mut v);
    }
    unreachable!()
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(super::run(include_str!("../input/12-sample.txt")), (183, 2772));
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/12.txt")), (12070, 500_903_629_351_944));
    }
}
