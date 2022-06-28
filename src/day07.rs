use std::iter::once;

use crate::{intcode::State, permutations::all_permutations};

pub fn run(input: &str) -> (i64, i64) {
    let init = State::from(input);
    let part1 = all_permutations(&[0, 1, 2, 3, 4])
        .map(|sig| {
            let progs = sig.into_iter().map(|k| init.clone().with_inputs(once(k)));

            progs.into_iter().fold(0, |prev, mut prog| {
                prog.send_input(prev);
                prog.get_output().unwrap()
            })
        })
        .max()
        .unwrap();
    let part2 = all_permutations(&[5, 6, 7, 8, 9])
        .map(|sig| {
            let mut progs: Vec<_> = sig
                .into_iter()
                .map(|k| init.clone().with_inputs(once(k)))
                .collect();

            let mut prev = 0;
            'a: loop {
                for prog in &mut progs {
                    prog.send_input(prev);
                    if let Some(out) = prog.get_output() {
                        prev = out;
                    } else {
                        break 'a prev;
                    }
                }
            }
        })
        .max()
        .unwrap();
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/07.txt")),
            (18812, 25_534_964)
        );
    }
}
