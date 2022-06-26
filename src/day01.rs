use crate::parsing::Gather;
use std::iter::successors;

pub fn run(input: &str) -> (i64, i64) {
    let fuel: Vec<i64> = input.lines().gather();

    (fuel.iter().map(|&x| x / 3 - 2).sum(), fuel.iter().copied().map(rec_fuel).sum())
}

fn rec_fuel(x: i64) -> i64 {
    successors(Some(x), |&y| Some(y / 3 - 2)).skip(1).take_while(|&y| y > 0).sum()
}

#[cfg(test)] 
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/01.txt")), (3_198_599, 4_795_042));
    }
}
