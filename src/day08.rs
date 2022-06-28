use crate::{arr2::Arr2, counter::Counter};

pub fn run(input: &str) -> (usize, String) {
    const N: usize = 6;
    const M: usize = 25;
    let layers: Vec<Arr2<u8>> = input
        .trim()
        .as_bytes()
        .chunks(N * M)
        .map(|s| Arr2::from_raw(N, M, s.to_owned()))
        .collect();

    let part1 = layers
        .iter()
        .map(|layer| {
            let c = layer.iter().copied().counter();
            (c[&b'0'], c[&b'1'] * c[&b'2'])
        })
        .min()
        .unwrap()
        .1;
    let composite = Arr2::from_fn(N, M, |i, j| {
        layers
            .iter()
            .fold(b'2', |c, layer| if c == b'2' { layer[i][j] } else { c })
    });

    // Part 2 is visual - Supposed to look like "BCYEF"
    let lines: Vec<String> = (0..N)
        .map(|i| {
            composite[i]
                .iter()
                .map(|&b| if b == b'1' { 'O' } else { '_' })
                .collect()
        })
        .collect();
    let part2 = lines.join("\n");
    println!("{}", part2);
    (part1, part2)
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        let image = r"
OOO___OO__O___OOOOO_OOOO_
O__O_O__O_O___OO____O____
OOO__O_____O_O_OOO__OOO__
O__O_O______O__O____O____
O__O_O__O___O__O____O____
OOO___OO____O__OOOO_O____
"
        .trim()
        .to_owned();
        assert_eq!(super::run(include_str!("../input/08.txt")), (1620, image));
    }
}
