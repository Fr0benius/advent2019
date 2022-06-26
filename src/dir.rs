pub const DIR4: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
pub const DIR8: [(i64, i64); 8] = [
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
    (-1, 0),
    (-1, 1),
];

pub fn neighbors_iter(
    dirs: &'static [(i64, i64)],
    r: usize,
    c: usize,
    n: usize,
    m: usize,
) -> impl Iterator<Item = (usize, usize)> {
    dirs.iter()
        .map(move |(dr, dc)| ((r as i64 + dr) as usize, (c as i64 + dc) as usize))
        .filter(move |&(a, b)| a < n && b < m)
}

pub fn neighbors4(r: usize, c: usize, n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    neighbors_iter(&DIR4, r, c, n, m)
}

pub fn neighbors8(r: usize, c: usize, n: usize, m: usize) -> impl Iterator<Item = (usize, usize)> {
    neighbors_iter(&DIR8, r, c, n, m)
}
