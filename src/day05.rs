use std::iter::once;

use crate::{parsing::Gather, intcode::State};

pub fn run(input: &str) -> (i64, i64) {
    let init: Vec<i64> = input.trim().split(',').gather();
    let mut state = State::new(&init).with_inputs(once(1));
    state.run_until_halt();
    let part1 = *state.outputs.back().unwrap();

    state = State::new(&init).with_inputs(once(5));
    state.run_until_halt();
    let part2 = *state.outputs.back().unwrap();
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/05.txt")), (11_193_703, 12_410_607));
    }
}
