use std::collections::VecDeque;

use Op::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    Lt,
    Eq,
    Halt,
}
impl From<i64> for Op {
    fn from(op: i64) -> Self {
        match op {
            1 => Add,
            2 => Mul,
            3 => Input,
            4 => Output,
            5 => JumpIfTrue,
            6 => JumpIfFalse,
            7 => Lt,
            8 => Eq,
            99 => Halt,
            _ => panic!("Unknown op: {}", op),
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub instr: usize,
    pub data: Vec<i64>,
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
}

impl State {
    pub fn new(data: Vec<i64>) -> Self {
        Self {
            instr: 0,
            data,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    #[must_use]
    pub fn with_inputs<T: Iterator<Item = i64>>(mut self, inputs: T) -> Self {
        self.inputs.extend(inputs);
        self
    }

    pub fn step(&mut self) -> bool {
        let code = self.data[self.instr];
        let op: Op = (code % 100).into();
        let modes = [
            ((code / 100) % 10) as u8,
            ((code / 1000) % 10) as u8,
            ((code / 10000) % 10) as u8,
        ];
        match op {
            Add | Mul | Lt | Eq => {
                self.bin_op(op, modes);
                true
            }
            Input => {
                let input = self.inputs.pop_front().unwrap();
                let target = self.data[self.instr + 1] as usize;
                self.data[target] = input;
                self.instr += 2;
                true
            }
            Output => {
                self.outputs.push_back(self.get(self.instr + 1, modes[0]));
                self.instr += 2;
                true
            }
            JumpIfTrue | JumpIfFalse => {
                self.jump(op, modes);
                true
            }
            Halt => false,
        }
    }
    pub fn run_until_halt(&mut self) {
        while self.step() {}
    }
    fn get(&self, i: usize, mode: u8) -> i64 {
        let imm = self.data[i];
        if mode == 0 {
            self.data[imm as usize]
        } else {
            imm
        }
    }
    fn bin_op(&mut self, op: Op, modes: [u8; 3]) {
        let i = self.instr;
        let arg0 = self.get(i + 1, modes[0]);
        let arg1 = self.get(i + 2, modes[1]);
        let target = self.data[i + 3] as usize;
        match op {
            Add => self.data[target] = arg0 + arg1,
            Mul => self.data[target] = arg0 * arg1,
            Lt => self.data[target] = (arg0 < arg1) as i64,
            Eq => self.data[target] = (arg0 == arg1) as i64,
            _ => unreachable!(),
        }
        self.instr += 4;
    }
    fn jump(&mut self, op: Op, modes: [u8; 3]) {
        let i = self.instr;
        let arg0 = self.get(i + 1, modes[0]);
        let arg1 = self.get(i + 2, modes[1]);
        if arg0 != 0 && op == JumpIfTrue || arg0 == 0 && op == JumpIfFalse {
            self.instr = arg1 as usize;
        } else {
            self.instr += 3;
        }
    }
}
