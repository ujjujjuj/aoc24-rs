use std::collections::HashSet;

const DIRS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn get_area_peri(
    graph: &Vec<Vec<char>>,
    c: char,
    i: usize,
    j: usize,
    vis_coords: &mut HashSet<(usize, usize)>,
) -> u64 {
    let mut peri = 4;
    vis_coords.insert((i, j));
    for (di, dj) in DIRS {
        let _i = (i as i64) + di;
        let _j = (j as i64) + dj;
        if _i < 0 || _j < 0 || _i >= graph.len() as i64 || _j >= graph[_i as usize].len() as i64 {
            continue;
        }

        if vis_coords.contains(&(_i as usize, _j as usize)) {
            peri -= 1;
            continue;
        }

        if graph[_i as usize][_j as usize] == c {
            peri -= 1;
            peri += get_area_peri(graph, c, _i as usize, _j as usize, vis_coords);
            continue;
        }
    }
    return peri;
}

fn solve(input: &str) -> String {
    let graph: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut vis_coords_global: HashSet<(usize, usize)> = HashSet::new();

    let mut ans = 0;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if !vis_coords_global.contains(&(i, j)) {
                let mut vis_coords: HashSet<(usize, usize)> = HashSet::new();
                let peri = get_area_peri(&graph, graph[i][j], i, j, &mut vis_coords);
                ans += peri * (vis_coords.len() as u64);

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
        assert_eq!(result, "1930")
    }
}
