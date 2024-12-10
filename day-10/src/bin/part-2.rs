fn count_trails(graph: &Vec<Vec<u8>>, expected_num: u8, i: usize, j: usize) -> u64 {
    if i >= graph.len() || j >= graph[i].len() {
        return 0;
    }

    if graph[i][j] == expected_num {
        if expected_num == 9 {
            return 1;
        } else {
            let mut ans = count_trails(graph, expected_num + 1, i + 1, j)
                + count_trails(graph, expected_num + 1, i, j + 1);

            if i != 0 {
                ans += count_trails(graph, expected_num + 1, i - 1, j)
            }
            if j != 0 {
                ans += count_trails(graph, expected_num + 1, i, j - 1);
            }

            return ans;
        }
    }

    0
}

fn solve(input: &str) -> String {
    let graph: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            let mut row_vec: Vec<u8> = vec![];
            for c in line.chars() {
                row_vec.push(c.to_digit(10).unwrap() as u8);
            }
            row_vec
        })
        .collect();

    let mut ans = 0;
    for i in 0..graph.len() {
        for j in 0..graph[i].len() {
            if graph[i][j] == 0 {
                ans += count_trails(&graph, 0, i, j);
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
        assert_eq!(result, "81")
    }
}
