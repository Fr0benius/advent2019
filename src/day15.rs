use std::collections::{HashMap, VecDeque};

use crate::intcode::State;

type Pt = (i64, i64);
const D: [Pt; 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

pub fn run(input: &str) -> (i64, i64) {
    let mut robot = Robot::new(input.into());
    robot.fill_map();
    let oxy = robot
        .map
        .keys()
        .copied()
        .find(|k| robot.map[k] == 2)
        .unwrap();
    let mut q = VecDeque::from([oxy]);
    let mut d = HashMap::from([(oxy, 0)]);
    while let Some(v) = q.pop_front() {
        let d0 = d[&v];
        let (x0, y0) = v;
        for (dx, dy) in D {
            let pt = (x0 + dx, y0 + dy);
            if robot.map.get(&pt) == Some(&0) {
                continue;
            }
            d.entry(pt).or_insert_with(|| {
                q.push_back(pt);
                d0 + 1
            });
        }
    }
    (d[&(0, 0)], *d.values().max().unwrap())
}

struct Robot {
    prog: State,
    loc: Pt,
    map: HashMap<Pt, i64>,
}

impl Robot {
    fn new(prog: State) -> Self {
        Self {
            prog,
            loc: (0, 0),
            map: HashMap::from([((0, 0), 1)]),
        }
    }
    fn mv(&mut self, dir: usize) -> i64 {
        self.prog.send_input(dir as i64 + 1);
        self.prog.get_output().unwrap()
    }

    fn fill_map(&mut self) {
        let (x0, y0) = self.loc;
        for (dir, (dx, dy)) in D.into_iter().enumerate() {
            let target = (x0 + dx, y0 + dy);
            if self.map.contains_key(&target) {
                continue;
            }
            let k = self.mv(dir);
            self.map.insert(target, k);
            if k == 0 {
                continue;
            }
            self.loc = target;
            self.fill_map();
            self.mv(dir ^ 1);
            self.loc = (x0, y0);
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/15.txt")), (216, 326));
    }
}
