use std::collections::HashMap;

use crate::parsing::{re_parser, Gather};

pub fn run(input: &str) -> (i64, i64) {
    let recipes: HashMap<&str, (i64, Vec<(i64, &str)>)> = {
        let parse_line = re_parser(r"(.*) => (.*) (.*)");
        input
            .lines()
            .map(|line| {
                let (ch, amt, item): (&str, i64, &str) = parse_line(line);
                (
                    item,
                    (amt, ch.split(", ").flat_map(|w| w.split(' ')).gather()),
                )
            })
            .collect()
    };
    let part2 = {
        const ORE: i64 = 1_000_000_000_000;
        let mut l = 1;
        let mut r = ORE;
        while l < r {
            let m = (l + r + 1) / 2;
            if ore_needed(m, &recipes) <= ORE {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l
    };
    (ore_needed(1, &recipes), part2)
}

fn ore_needed<'a>(fuel: i64, recipes: &HashMap<&'a str, (i64, Vec<(i64, &'a str)>)>) -> i64 {
    let mut needed = HashMap::from([("FUEL", fuel)]);
    let mut extra: HashMap<&str, i64> = HashMap::new();
    let mut ore = 0;
    while let Some(&item) = needed.keys().next() {
        let mut amt = needed.remove(&item).unwrap();
        if item == "ORE" {
            ore += amt;
            continue;
        }
        if let Some(&ext) = extra.get(&item) {
            let k = amt.min(ext);
            amt -= k;
            extra.insert(item, ext - k);
        }
        if amt == 0 {
            continue;
        }
        let (sub_amt, ref sub_recipe) = recipes[&item];
        let scale = (amt + sub_amt - 1) / sub_amt;
        *extra.entry(item).or_insert(0) += scale * sub_amt - amt;
        for &(ch_amt, ch) in sub_recipe {
            *needed.entry(ch).or_insert(0) += ch_amt * scale;
        }
    }
    ore
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_sample() {
        assert_eq!(super::run(include_str!("../input/14-sample.txt")), (13312, 82_892_753));
    }
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/14.txt")), (301_997, 6_216_589));
    }
}
