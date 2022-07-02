use Op::{Cut, Incr, Rev};

use crate::num::mod_inv;
pub fn run(input: &str) -> (i64, i64) {
    let ops: Vec<Op> = input
        .lines()
        .map(|s| {
            if s == "deal into new stack" {
                Rev
            } else if let Some(x) = s.strip_prefix("cut ") {
                Cut(x.parse().unwrap())
            } else if let Some(x) = s.strip_prefix("deal with increment ") {
                let n: i64 = x.parse().unwrap();
                dbg!(n, mod_inv(n, 119_315_717_514_047));
                Incr(x.parse().unwrap())
            } else {
                unreachable!()
            }
        })
        .collect();
    let m1 = 10007;
    let part1 = eval(fold(ops.clone(), m1, false), 2019, m1);
    let m2 = 119_315_717_514_047;
    let line = fold(ops, m2, true);
    let (line, _) = mat_pow_mod((line, (0, 1)), 101_741_582_076_661, m2);
    let part2 = eval(line, 2020, m2);
    (part1, part2)
}

type Pll = (i64, i64);
type Mat = (Pll, Pll);

fn mul_mod(a: i64, b: i64, m: i64) -> i64 {
    (a as i128 * b as i128 % m as i128) as i64
}
fn mat_mul_mod(((a0, b0), (c0, d0)): Mat, ((a1, b1), (c1, d1)): Mat, m: i64) -> Mat {
    (
        (
            (mul_mod(a0, a1, m) + mul_mod(b0, c1, m)) % m,
            (mul_mod(a0, b1, m) + mul_mod(b0, d1, m)) % m,
        ),
        (
            (mul_mod(c0, a1, m) + mul_mod(d0, c1, m)) % m,
            (mul_mod(c0, b1, m) + mul_mod(d0, d1, m)) % m,
        ),
    )
}
fn mat_pow_mod(mut mat: Mat, mut k: i64, m: i64) -> Mat {
    let mut res = ((1, 0), (0, 1));
    while k > 0 {
        if k % 2 == 1 {
            res = mat_mul_mod(res, mat, m);
        }
        mat = mat_mul_mod(mat, mat, m);
        k /= 2;
    }
    res
}

fn eval((a, b): Pll, x: i64, m: i64) -> i64 {
    (mul_mod(a, x, m) + b) % m
}
fn fold(mut ops: Vec<Op>, m: i64, backwards: bool) -> Pll {
    let (mut a, mut b): Pll = (1, 0);
    if backwards {
        ops.reverse();
    }
    for op in ops {
        match op {
            Rev => {
                a = (m - a) % m;
                b = m - 1 - b;
            }
            Cut(x) => {
                if backwards {
                    b = (b + m + x) % m;
                } else {
                    b = (b + m - x) % m;
                }
            }
            Incr(x) => {
                let mul = if backwards {
                    mod_inv(x, m) 
                } else {
                    x
                };
                a = mul_mod(a, mul, m);
                b = mul_mod(b, mul, m);
            }
        }
    }
    (a, b)
}

#[derive(Clone, Copy, Debug)]
enum Op {
    Rev,
    Cut(i64),
    Incr(i64),
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/22.txt")),
            (1538, 96_196_710_942_473)
        );
    }
}
