const MAS_DIRS: [[(i32, i32); 2]; 2] = [[(1, 1), (-1, -1)], [(1, -1), (-1, 1)]];

fn count_x_mas(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u64 {
    if grid[i][j] != b'A' {
        return 0;
    }

    let mut mas_valid = true;
    for mas in MAS_DIRS {
        let mut expected_c = 0;
        for (di, dj) in mas {
            let _i = (i as i32) + di;
            let _j = (j as i32) + dj;
            if _i < 0 || _j < 0 || _i >= grid.len() as i32 || _j >= grid[0].len() as i32 {
                mas_valid = false;
                continue;
            }

            let c = grid[_i as usize][_j as usize];
            if expected_c == 0 {
                if c == b'M' {
                    expected_c = b'S'
                } else if c == b'S' {
                    expected_c = b'M'
                } else {
                    mas_valid = false
                }
            } else if c != expected_c {
                mas_valid = false
            }
        }
    }

    if mas_valid {
        return 1;
    }

    0
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            ans += count_x_mas(&grid, i, j);
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
        assert_eq!(result, "9")
    }
}
