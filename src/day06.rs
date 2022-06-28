use std::collections::HashMap;

use crate::parsing::Gather;

pub fn run(input: &str) -> (i64, i64) {
    let (g, par) = {
        let mut g = HashMap::new();
        let mut par = HashMap::new();
        for line in input.lines() {
            let (v, w) = line.split(')').gather();
            g.entry(v).or_insert(vec![]).push(w);
            g.entry(w).or_insert(vec![]).push(v);
            par.insert(w, v);
        }
        (g, par)
    };

    let mut dep: HashMap<&str, i64> = HashMap::new();
    dfs("COM", "", &g, &mut dep);
    let part1 = dep.values().copied().sum();

    dep.clear();
    dfs(par["YOU"], "", &g, &mut dep);
    let part2 = dep[par["SAN"]];

    (part1, part2)
}

fn dfs<'a>(v: &'a str, p: &str, g: &'a HashMap<&str, Vec<&str>>, dep: &mut HashMap<&'a str, i64>) {
    if let Some(&d) = dep.get(&p) {
        dep.insert(v, d + 1);
    } else {
        dep.insert(v, 0);
    }
    for &w in &g[v] {
        if w != p {
            dfs(w, v, g, dep);
        }
    }
}

#[cfg(test)]
pub mod tests {
    #[test]
    fn check_input() {
        assert_eq!(super::run(include_str!("../input/06.txt")), (241_064, 418));
    }
}
