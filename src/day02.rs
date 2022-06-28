use crate::parsing::Gather;

use crate::intcode::State;

pub fn run(input: &str) -> (i64, i64) {
    let mut data: Vec<i64> = input.trim().split(',').gather();
    let part1 = simulate(&mut data, 12, 2);
    let mut part2 = 0;
    'a: for noun in 0..100 {
        for verb in 0..100 {
            if simulate(&mut data, noun, verb) == 19_690_720 {
                part2 = 100 * noun + verb;
                break 'a;
            }
        }
    }
    (part1, part2)
}

fn simulate(data: &mut [i64], noun: i64, verb: i64) -> i64 {
    data[1] = noun;
    data[2] = verb;
    let mut state = State::new(data);
    state.run_until_halt();
    state.peek(0)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/02.txt")),
            (3_760_627, 7195)
        );
    }
}
