use std::{iter::once, collections::VecDeque};

use crate::intcode::State;

pub fn run(input: &str) -> (i64, i64) {
    let init = State::from(input);
    let progs: Vec<State> = (0..50).map(|k| init.clone().with_inputs(once(k))).collect();
    (part1(progs.clone()), part2(progs))
}

fn part1(mut progs: Vec<State>) -> i64 {
    let mut q = vec![VecDeque::new(); 50];
    loop {
        for k in 0..50 {
            while let Some(addr) = progs[k].get_output() {
                let x = progs[k].get_output().unwrap();
                let y = progs[k].get_output().unwrap();
                if addr == 255 {
                    return y;
                }
                q[addr as usize].push_back((x, y));
            }
        }
        for k in 0..50 {
            if q[k].is_empty() {
                progs[k].send_input(-1);
                continue;
            }
            
            while let Some((x, y)) = q[k].pop_front() {
                progs[k].send_input(x);
                progs[k].send_input(y);
            }
        }
    }
}

fn part2(mut progs: Vec<State>) -> i64 {
    let mut q = vec![VecDeque::new(); 50];
    let mut last = None;
    let mut nat = None;
    loop {
        for k in 0..50 {
            while let Some(addr) = progs[k].get_output() {
                let x = progs[k].get_output().unwrap();
                let y = progs[k].get_output().unwrap();
                if addr == 255 {
                    nat = Some((x, y));
                    continue;
                }
                q[addr as usize].push_back((x, y));
            }
        }
        if q.iter().all(|l| l.is_empty()) {
            if let Some((x, y)) = nat {
                if last == Some(y) {
                    return y;
                }
                last = Some(y);
                progs[0].send_input(x);
                progs[0].send_input(y);
                continue;
            }
        }
        for k in 0..50 {
            if q[k].is_empty() {
                progs[k].send_input(-1);
                continue;
            }
            
            while let Some((x, y)) = q[k].pop_front() {
                progs[k].send_input(x);
                progs[k].send_input(y);
            }
        }
    }
}
#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/23.txt")), (19530, 12725));
    }
}
