use std::iter::once;

use crate::intcode::State;

pub fn run(input: &str) -> (i64, i64) {
    let mut prog = State::from(input).with_inputs(once(1));
    let part1 = prog.get_output().unwrap();
    prog = State::from(input).with_inputs(once(2));
    let part2 = prog.get_output().unwrap();
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    // #[test]
    // fn check_sample() {
    //     assert_eq!(super::run(include_str!("../input/09-sample.txt")), (0, 0));
    // }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/09.txt")), (2_714_716_640, 58879));
    }
}
