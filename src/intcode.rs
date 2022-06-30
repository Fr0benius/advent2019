use std::collections::{HashMap, VecDeque};

use InstructionStatus::{AwaitingInput, Done, Success};
use Op::{Add, AdjustRelBase, Eql, Halt, Input, JumpIfFalse, JumpIfTrue, Lt, Mul, Output};

use crate::parsing::Gather;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Op {
    Add,
    Mul,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    Lt,
    Eql,
    AdjustRelBase,
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
            8 => Eql,
            9 => AdjustRelBase,
            99 => Halt,
            _ => panic!("Unknown op: {}", op),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstructionStatus {
    Success,
    AwaitingInput,
    Done,
}

#[derive(Clone, Debug)]
pub struct State {
    instr: usize,
    data: HashMap<usize, i64>,
    inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    relative_base: i64,
}

impl State {
    pub fn new(data: &[i64]) -> Self {
        Self {
            instr: 0,
            data: data.iter().copied().enumerate().collect(),
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            relative_base: 0,
        }
    }
    pub fn peek(&self, i: usize) -> i64 {
        self.data.get(&i).copied().unwrap_or_default()
    }
    pub fn poke(&mut self, i: usize, val: i64) {
        self.data.insert(i, val);
    }
    #[must_use]
    pub fn with_inputs<T: Iterator<Item = i64>>(mut self, inputs: T) -> Self {
        self.inputs.extend(inputs);
        self
    }
    pub fn send_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    fn step(&mut self) -> InstructionStatus {
        let i = self.instr;
        let code = self.peek(i);
        let op: Op = (code % 100).into();
        let modes = [
            ((code / 100) % 10) as u8,
            ((code / 1000) % 10) as u8,
            ((code / 10000) % 10) as u8,
        ];
        match op {
            Add | Mul | Lt | Eql => {
                self.bin_op(op, modes);
                Success
            }
            Input => {
                if let Some(input) = self.inputs.pop_front() {
                    self.set(i + 1, modes[0], input);
                    self.instr += 2;
                    Success
                } else {
                    AwaitingInput
                }
            }
            Output => {
                self.outputs.push_back(self.get(i + 1, modes[0]));
                self.instr += 2;
                Success
            }
            JumpIfTrue | JumpIfFalse => {
                self.jump(op, modes);
                Success
            }
            AdjustRelBase => {
                let adj = self.get(i + 1, modes[0]);
                self.relative_base += adj;
                self.instr += 2;
                Success
            }
            Halt => Done,
        }
    }

    pub fn run_until_input(&mut self) -> bool {
        loop {
            match self.step() {
                Success => continue,
                AwaitingInput => return true,
                Done => return false,
            }
        }
    }
    pub fn run_until_halt(&mut self) {
        loop {
            match self.step() {
                Success => continue,
                AwaitingInput => panic!("Expecting an input"),
                Done => break,
            }
        }
    }
    pub fn get_output(&mut self) -> Option<i64> {
        loop {
            if let Some(output) = self.outputs.pop_front() {
                return Some(output);
            }
            if self.step() != Success {
                return None;
            }
        }
    }
    fn get(&self, i: usize, mode: u8) -> i64 {
        let imm = self.peek(i);
        match mode {
            0 => self.peek(imm as usize),
            1 => imm,
            2 => self.peek((self.relative_base + imm) as usize),
            _ => unreachable!(),
        }
    }
    fn set(&mut self, i: usize, mode: u8, val: i64) {
        let target = match mode {
            0 => self.peek(i) as usize,
            2 => (self.peek(i) + self.relative_base) as usize,
            _ => unreachable!(),
        };
        self.data.insert(target, val);
    }
    fn bin_op(&mut self, op: Op, modes: [u8; 3]) {
        let i = self.instr;
        let arg0 = self.get(i + 1, modes[0]);
        let arg1 = self.get(i + 2, modes[1]);
        match op {
            Add => self.set(i + 3, modes[2], arg0 + arg1),
            Mul => self.set(i + 3, modes[2], arg0 * arg1),
            Lt => self.set(i + 3, modes[2], (arg0 < arg1) as i64),
            Eql => self.set(i + 3, modes[2], (arg0 == arg1) as i64),
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

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let v: Vec<i64> = s.trim().split(',').gather();
        Self::new(&v)
    }
}
