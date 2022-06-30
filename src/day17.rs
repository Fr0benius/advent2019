use std::{collections::HashMap, iter::repeat_with};

use crate::{
    arr2::Arr2,
    dir::{mv, neighbors4, DIR4},
    intcode::State,
};

pub fn run(input: &str) -> (usize, i64) {
    let mut prog = State::from(input);
    let raw_chars: Vec<char> = repeat_with(|| prog.get_output())
        .take_while(|x| x.is_some())
        .flatten()
        .map(|x| x as u8 as char)
        .collect();
    let chars: Vec<char> = raw_chars.iter().copied().filter(|&x| x != '\n').collect();
    let m = raw_chars
        .iter()
        .enumerate()
        .find(|&(_, &c)| c == '\n')
        .unwrap()
        .0;
    let n = chars.len() / m;
    let grid = Arr2::from_raw(n, m, chars);
    let mut part1 = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] != '.' && neighbors4(i, j, n, m).all(|(r, c)| grid[r][c] != '.') {
                part1 += i * j;
            }
        }
    }

    // Breakdown was found by inspection
    let moves = naive(&grid);
    let a = "L,12,L,8,R,12";
    let b = "L,10,L,8,L,12,R,12";
    let c = "R,12,L,8,L,10";
    let main_routine = decomp(&moves, &[("A", a), ("B", b), ("C", c)].into());

    let mut prog = State::from(input);
    prog.poke(0, 2);
    send(&mut prog, &main_routine);
    send(&mut prog, a);
    send(&mut prog, b);
    send(&mut prog, c);
    send(&mut prog, "n");
    let mut part2 = 0;
    while let Some(out) = prog.get_output() {
        part2 = out;
    }
    (part1, part2)
}

fn send(prog: &mut State, s: &str) {
    for c in s.chars() {
        prog.send_input(c as i64);
    }
    prog.send_input('\n' as i64);
}

fn decomp(mut s: &str, map: &HashMap<&str, &str>) -> String {
    let mut res = String::new();
    'a: while !s.is_empty() {
        for (&x, &sub) in map {
            if s.starts_with(sub) {
                res += x;
                s = &s[sub.len()..];
                continue 'a;
            }
        }
        res += &s[0..1];
        s = &s[1..];
    }
    dbg!(&res);
    res
}

fn naive(grid: &Arr2<char>) -> String {
    let (n, m) = grid.dims();
    let (mut r, mut c, mut dir) = (0, 0, 0);
    'a: for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                '^' => (r, c, dir) = (i, j, 3),
                '>' => (r, c, dir) = (i, j, 0),
                'v' => (r, c, dir) = (i, j, 1),
                '<' => (r, c, dir) = (i, j, 2),
                _ => continue,
            }
            break 'a;
        }
    }
    let mut cur = 0;
    let mut moves = String::new();
    loop {
        let (r1, c1) = mv((r, c), DIR4[dir]);
        if r1 < n && c1 < m && grid[r1][c1] == '#' {
            cur += 1;
            (r, c) = (r1, c1);
            continue;
        }

        dir = (dir + 1) % 4;
        let (r1, c1) = mv((r, c), DIR4[dir]);
        if r1 < n && c1 < m && grid[r1][c1] == '#' {
            if cur > 0 {
                moves += ",";
                moves += &cur.to_string();
                moves += ",";
            }
            moves += "R";
            cur = 1;
            (r, c) = (r1, c1);
            continue;
        }
        dir = (dir + 2) % 4;
        let (r1, c1) = mv((r, c), DIR4[dir]);
        if r1 < n && c1 < m && grid[r1][c1] == '#' {
            if cur > 0 {
                moves += ",";
                moves += &cur.to_string();
                moves += ",";
            }
            moves += "L";
            cur = 1;
            (r, c) = (r1, c1);
            continue;
        }
        break;
    }
    if cur > 0 {
        moves += ",";
        moves += &cur.to_string();
    }
    println!("{}", moves);
    moves
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/17.txt")), (10632, 1_356_191));
    }
}
