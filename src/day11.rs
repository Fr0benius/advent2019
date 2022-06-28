use std::collections::HashSet;

use crate::{dir::DIR4, intcode::State};

pub fn run(input: &str) -> (usize, String) {
    let prog = State::from(input);
    let mut white_squares: HashSet<(i64, i64)> = HashSet::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    simulate(prog.clone(), &mut white_squares, &mut visited);
    let part1 = visited.len();

    visited.clear();
    white_squares = [(0, 0)].into();
    simulate(prog, &mut white_squares, &mut visited);
    let part2: String = render(&white_squares);
    println!("{}", part2);
    (part1, part2)
}

fn render(white_squares: &HashSet<(i64, i64)>) -> String {
    let mnx = white_squares.iter().map(|&(x, _)| x).min().unwrap();
    let mxx = white_squares.iter().map(|&(x, _)| x).max().unwrap();
    let mny = white_squares.iter().map(|&(_, y)| y).min().unwrap();
    let mxy = white_squares.iter().map(|&(_, y)| y).max().unwrap();
    let rows: Vec<String> = (mny..=mxy)
        .rev()
        .map(|y| {
            (mnx..=mxx)
                .map(|x| {
                    if white_squares.contains(&(x, y)) {
                        'O'
                    } else {
                        '.'
                    }
                })
                .collect()
        })
        .collect();
    rows.join("\n")
}

fn simulate(
    mut prog: State,
    white_squares: &mut HashSet<(i64, i64)>,
    visited: &mut HashSet<(i64, i64)>,
) {
    let mut dir = 0;
    let mut pt = (0, 0);
    loop {
        prog.send_input(white_squares.contains(&pt) as i64);
        if let Some(color) = prog.get_output() {
            visited.insert(pt);
            if color == 1 {
                white_squares.insert(pt);
            } else {
                white_squares.remove(&pt);
            }
            let rot = prog.get_output().unwrap();
            if rot == 1 {
                dir = (dir + 1) % 4;
            } else {
                dir = (dir + 3) % 4;
            }
            let (dx, dy) = DIR4[dir];
            pt.0 += dx;
            pt.1 += dy;
        } else {
            break;
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        let image = r"
O..O.OOOO..OO..OOOO.O..O.OOO..O....OOO.
O..O....O.O..O.O....O.O..O..O.O....O..O
O..O...O..O..O.OOO..OO...OOO..O....O..O
O..O..O...OOOO.O....O.O..O..O.O....OOO.
O..O.O....O..O.O....O.O..O..O.O....O...
.OO..OOOO.O..O.OOOO.O..O.OOO..OOOO.O...
"
        .trim()
        .into();
        assert_eq!(super::run(include_str!("../input/11.txt")), (2336, image));
    }
}
