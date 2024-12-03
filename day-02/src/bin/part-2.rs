fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let nums: Vec<_> = line
                .split_whitespace()
                .filter_map(|num_str| num_str.parse::<i64>().ok())
                .collect();

            let mut is_increasing = false;

            for _ in 0..2 {
                for i in 0..nums.len() {
                    let mut numsc = nums.clone();
                    numsc.remove(i);
                    if numsc.windows(2).all(|window| match window {
                        &[a, b] => {
                            if is_increasing {
                                b - a <= 3 && b > a
                            } else {
                                a - b <= 3 && (a > b)
                            }
                        }
                        _ => false,
                    }) {
                        return 1;
                    }
                }

                is_increasing = true;
            }
            return 0;
        })
        .sum::<u64>()
        .to_string()
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
        assert_eq!(result, "4")
    }
}
