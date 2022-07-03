use std::iter::repeat_with;

use crate::intcode::State;

const GATHER_ITEMS: &str = r"
south
east
take space heater
west
south
south
east
east
take planetoid
west
west
north
north
north
north
take festive hat
south
east
take spool of cat6
north
north
take hypercube
south
south
west
north
west
take dark matter
north
east
take semiconductor
east
take sand
north
inv
";
pub fn run(input: &str) -> String {
    let mut prog = State::from(input);
    for c in GATHER_ITEMS.bytes() {
        prog.send_input(c as i64);
    }
    println!("{}", output(&mut prog));
    let items: Vec<_> = GATHER_ITEMS
        .lines()
        .filter_map(|line| line.strip_prefix("take "))
        .collect();
    let n = items.len();
    let mut last_msk = (1 << n) - 1;
    for msk in 0..1 << n {
        for i in 0..n {
            if (msk & !last_msk) & (1 << i) != 0 {
                cmd(&mut prog, &format!("take {}\n", items[i]));
            }
            else if (!msk & last_msk) & (1 << i) != 0 {
                cmd(&mut prog, &format!("drop {}\n", items[i]));
            }
        }
        let out = cmd(&mut prog, "west\n");
        if !out.contains("ejected") {
            println!("{}", out);
            for i in 0..n {
                if msk & (1 << i) 
                    != 0 {
                        println!("{}", items[i]);
                    }
            }
            return out;
        }
        last_msk = msk;
    }
    "".into()
}

fn cmd(prog: &mut State, command: &str) -> String {
    for c in command.bytes() {
        prog.send_input(c as i64);
    }
    output(prog)
}

fn _read_input(prog: &mut State) {
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).unwrap();
    for c in command.bytes() {
        prog.send_input(c as i64);
    }
    println!("{}", output(prog));
}

fn output(prog: &mut State) -> String {
    repeat_with(|| prog.get_output())
        .take_while(|x| x.is_some())
        .flatten()
        .map(|c| c as u8 as char)
        .collect()
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        let out = r#"
A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
"Oh, hello! You should be able to get in by typing 35332 on the keypad at the main airlock.""#;
        assert!(super::run(include_str!("../input/25.txt")).contains(out.trim()));
    }
}
