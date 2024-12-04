const DIRS: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

const WORD: &[u8] = b"XMAS";

fn count_xmas(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u64 {
    let mut ans = 0;
    for (di, dj) in DIRS {
        for offset in 0..WORD.len() as i32 {
            let _i = (i as i32) + di * offset;
            let _j = (j as i32) + dj * offset;
            if _i < 0
                || _j < 0
                || _i >= grid.len() as i32
                || _j >= grid[0].len() as i32
                || grid[_i as usize][_j as usize] != WORD[offset as usize]
            {
                break;
            }

            if offset == (WORD.len() - 1) as i32 {
                ans += 1;
            }
        }
    }

    return ans;
}

fn solve(input: &str) -> String {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.bytes().collect()).collect();

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            ans += count_xmas(&grid, i, j);
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
        assert_eq!(result, "18")
    }
}
