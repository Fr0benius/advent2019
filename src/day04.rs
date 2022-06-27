use std::iter::once;

use crate::parsing::Gather;

pub fn run(input: &str) -> (i64, i64) {
    let (mn, mx): (i64, i64) = input.trim().split('-').gather();
    let part1 = (mn..=mx)
        .filter(|&n| {
            let s: Vec<char> = n.to_string().chars().collect();
            let w: Vec<_> = s.windows(2).collect();
            w.iter().all(|&z| z[0] <= z[1]) && w.iter().any(|&z| z[0] == z[1])
        })
        .count() as i64;
    let part2 = (mn..=mx)
        .filter(|&n| {
            let s: Vec<char> = once(0 as char)
                .chain(n.to_string().chars())
                .chain(once(255 as char))
                .collect();
            let w: Vec<_> = s.windows(2).collect();
            let w4: Vec<_> = s.windows(4).collect();
            w.iter().all(|&z| z[0] <= z[1])
                && w4
                    .iter()
                    .any(|&z| z[0] != z[1] && z[1] == z[2] && z[2] != z[3])
        })
        .count() as i64;
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/04.txt")), (979, 635));
    }
}
