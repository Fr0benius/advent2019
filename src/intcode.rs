use Op::{Add, Halt, Mul};

#[derive(Clone, Copy, Debug)]
pub enum Op {
    Add,
    Mul,
    Halt,
}
impl From<i64> for Op {
    fn from(op: i64) -> Self {
        match op {
            1 => Add,
            2 => Mul,
            99 => Halt,
            _ => panic!("Unknown op: {}", op),
        }
    }
}

#[derive(Clone, Debug)]
pub struct State {
    pub instr: usize,
    pub data: Vec<i64>,
}

impl State {
    pub fn step(&mut self) -> bool {
        let op: Op = self.data[self.instr].into();
        match op {
            Add | Mul => {
                self.bin_op(op);
                self.instr += 4;
                true
            }
            Halt => false,
        }
    }
    fn bin_op(&mut self, op: Op) {
        let i = self.instr;
        let a = self.data[i + 1] as usize;
        let b = self.data[i + 2] as usize;
        let target = self.data[i + 3] as usize;
        match op {
            Add => self.data[target] = self.data[a] + self.data[b],
            Mul => self.data[target] = self.data[a] * self.data[b],
            Halt => unreachable!(),
        }
    }
}
