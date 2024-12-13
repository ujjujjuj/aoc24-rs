use std::collections::{HashMap, HashSet};

const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_area_peri(
    graph: &Vec<Vec<char>>,
    c: char,
    i: usize,
    j: usize,
    vis_coords: &mut HashSet<(usize, usize)>,
    added_walls: &mut HashMap<(i64, u8), Vec<usize>>,
) {
    vis_coords.insert((i, j));
    for (di, dj) in DIRS {
        let mut add_side = || {
            if di == 0 {
                if dj == -1 {
                    added_walls
                        .entry((j as i64, 0))
                        .or_insert_with(Vec::new)
                        .push(i);
                } else {
                    added_walls
                        .entry(((j + 1) as i64, 1))
                        .or_insert_with(Vec::new)
                        .push(i);
                }
            } else {
                if di == -1 {
                    added_walls
                        .entry((-((i + 1) as i64), 0))
                        .or_insert_with(Vec::new)
                        .push(j);
                } else {
                    added_walls
                        .entry((-((i + 2) as i64), 1))
                        .or_insert_with(Vec::new)
                        .push(j);
                }
            }
        };

        let _i = (i as i64) + di;
        let _j = (j as i64) + dj;
        if _i < 0 || _j < 0 || _i >= graph.len() as i64 || _j >= graph[_i as usize].len() as i64 {
            add_side();
            continue;
        }

        if vis_coords.contains(&(_i as usize, _j as usize)) {
            continue;
        }

        if graph[_i as usize][_j as usize] == c {
            get_area_peri(graph, c, _i as usize, _j as usize, vis_coords, added_walls);
            continue;
        } else {
            add_side();
        }
    }
}

fn solve(input: &str) -> String {
    let graph: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut vis_coords_global: HashSet<(usize, usize)> = HashSet::new();

    let mut ans = 0;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if !vis_coords_global.contains(&(i, j)) {
                let mut vis_coords: HashSet<(usize, usize)> = HashSet::new();
                let mut added_walls: HashMap<(i64, u8), Vec<usize>> = HashMap::new();
                get_area_peri(&graph, graph[i][j], i, j, &mut vis_coords, &mut added_walls);

                let mut sides = 0;
                for (_, walls) in &mut added_walls {
                    walls.sort();
                    let mut last_side = 999999usize;
                    for &mut side in walls {
                        if side == 0 || side - 1 != last_side {
                            sides += 1
                        }
                        last_side = side;
                    }
                }

                ans += sides * vis_coords.len() as u64;
                vis_coords_global.extend(vis_coords);
            }
        }
    }

    return ans.to_string();
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
        assert_eq!(result, "1206")
    }
}
