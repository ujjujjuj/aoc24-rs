use std::{
    cmp,
    collections::{HashMap, HashSet},
};

type Coord = (i64, i64);

fn solve(input: &str) -> String {
    let mut h = 0i64;
    let mut w = 0i64;
    let mut positions: HashMap<char, Vec<Coord>> = HashMap::new();

    input.lines().enumerate().for_each(|(i, line)| {
        w = cmp::max(w, line.len() as i64);
        h += 1;
        for (j, c) in line.chars().enumerate() {
            if c != '.' {
                positions
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i as i64, j as i64));
            }
        }
    });

    let mut antinodes: HashSet<Coord> = HashSet::new();

    for (_, pos) in positions {
        for i in 0..pos.len() {
            for j in 0..pos.len() {
                if i == j {
                    continue;
                }
                let antinode = (2 * pos[j].0 - pos[i].0, 2 * pos[j].1 - pos[i].1);
                if antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < h && antinode.1 < w {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    return antinodes.len().to_string();
}

fn main() {
    let input = include_str!("../inputs/1.txt");
    let output = solve(input);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(include_str!("../inputs/1.test.txt"));
        assert_eq!(result, "14")
    }
}
