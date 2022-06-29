use std::collections::HashMap;

use crate::{intcode::State, parsing::Gather};

pub fn run(input: &str) -> (usize, i64) {
    let mut instrs: Vec<i64> = input.trim().split(',').gather();
    let init_state = State::new(&instrs);
    let mut game = Game::new(init_state);
    game.load();
    let part1 = game.board.values().filter(|&&x| x == 2).count();

    instrs[0] = 2;
    let init_state_unlocked = State::new(&instrs);
    game = Game::new(init_state_unlocked);
    let part2 = game.simulate();
    (part1, part2)
}

type Pt = (i64, i64);

struct Game {
    prog: State,
    board: HashMap<Pt, u8>,
    paddle: Pt,
    ball: Pt,
    prev_ball: Pt,
    score: i64,
}

impl Game {
    fn new(init_state: State) -> Self {
        Self {
            prog: init_state,
            board: HashMap::new(),
            paddle: (0, 0),
            ball: (-1, -1),
            prev_ball: (-1, -1),
            score: 0,
        }
    }

    fn load(&mut self) {
        while let Some(x) = self.prog.get_output() {
            let y = self.prog.get_output().unwrap();
            let tile = self.prog.get_output().unwrap();
            self.board.insert((x, y), tile as u8);
            if x == -1 {
                self.score = tile;
            } else if tile == 3 {
                self.paddle = (x, y);
            } else if tile == 4 {
                self.prev_ball = self.ball;
                self.ball = (x, y);
            }
        }
    }
    fn _render(&self) -> String {
        let mx = self.board.keys().map(|pt| pt.0).max().unwrap();
        let my = self.board.keys().map(|pt| pt.1).max().unwrap();
        (0..=my)
            .map(|y| {
                (0..=mx)
                    .map(|x| match self.board.get(&(x, y)) {
                        Some(&0) => ' ',
                        Some(&1) => '#',
                        Some(&2) => 'X',
                        Some(&3) => '_',
                        Some(&4) => 'O',
                        None => '~',
                        Some(_) => unreachable!(),
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn simulate(&mut self) -> i64 {
        loop {
            self.load();
            if self.prog.run_until_input() {
                let (x0, y0) = self.ball;
                let target = if self.board[&(x0, y0)] == 1 || self.prev_ball.0 < x0 {
                    x0 + 1
                } else {
                    x0 - 1
                };
                let mut signal = (target - self.paddle.0).signum();
                if y0 == self.paddle.1 - 1 && x0 == self.paddle.0 {
                    signal = 0;
                }
                self.prog.send_input(signal);
            } else {
                return self.score;
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/13.txt")), (372, 19297));
    }
}
