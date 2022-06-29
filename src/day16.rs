pub fn run(input: &str) -> (usize, usize) {
    let init: Vec<i32> = input.trim().bytes().map(|b| (b - b'0') as i32).collect();
    let state = (0..100).fold(init.clone(), |s, _| step(&s, 0));
    let part1 = convert(&state, 0, 8);
    let init2 = {
        let mut v = init.clone();
        for _ in 0..9999 {
            v.extend(init.iter().copied());
        }
        v
    };
    let pos = convert(&init, 0, 7);
    let state = (0..100).fold(init2, |s, _| step(&s, pos));
    let part2 = convert(&state, pos, pos + 8);
    (part1, part2)
}

fn convert(a: &[i32], i: usize, j: usize) -> usize {
    a[i..j].iter().copied().fold(0, |x, y| x * 10 + y as usize)
}

fn step(a: &[i32], start_at: usize) -> Vec<i32> {
    let n = a.len();
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let mut b = vec![0; n];
    for k in start_at + 1..=n {
        for i in 1.. {
            if k * (2 * i - 1) > n {
                break;
            }
            let sign = (i as i32) % 2 * 2 - 1;
            b[k - 1] += (s[(k * (2 * i) - 1).min(n)] - s[k * (2 * i - 1) - 1]) * sign;
        }
        b[k - 1] = (b[k - 1] % 10).abs();
    }
    b
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(super::run(include_str!("../input/16-sample.txt")), (24_465_799, 84_462_026));
    }
    #[test]
    fn check_input() {
        assert_eq!(
            super::run(include_str!("../input/16.txt")),
            (44_098_263, 12_482_168)
        );
    }
}
