use crate::intcode::State;
pub fn run(input: &str) -> (i64, i64) {
    let init = State::from(input);
    (part1(&init), part2(&init))
}

fn part1(init: &State) -> i64 {
    let strategy1 = r"
NOT A J
NOT B T
OR T J
AND D J
WALK";
    let strategy2 = r"
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK";
    exec(init, strategy1).max(exec(init, strategy2))
}

fn part2(init: &State) -> i64 {
    let strategy = r"
NOT C J
AND H J
NOT A T
OR T J
NOT B T
OR T J
AND D J
RUN";
    exec(init, strategy)
}

fn exec(init: &State, spring_script: &str) -> i64 {
    let mut prog = init.clone();
    while let Some(n) = prog.get_output() {
        print!("{}", n as u8 as char);
    }
    println!("-----------------------");
    for c in spring_script.trim().bytes() {
        prog.send_input(c as i64);
    }
    prog.send_input(b'\n' as i64);
    while let Some(n) = prog.get_output() {
        if let Ok(c) = u8::try_from(n) {
            print!("{}", c as char);
        } else {
            return n;
        }
    }
    println!("No output score");
    0
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/21.txt")),
            (19_360_724, 1_140_450_681)
        );
    }
}
