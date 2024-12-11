fn count_stones(root_stone: u64, blinks_left: u64) -> u64 {
    if blinks_left == 0 {
        return 1;
    }

    if root_stone == 0 {
        return count_stones(1, blinks_left - 1);
    }
    let str_stone = root_stone.to_string();
    if str_stone.len() % 2 == 0 {
        return count_stones(
            str_stone[..(str_stone.len() / 2)].parse::<u64>().unwrap(),
            blinks_left - 1,
        ) + count_stones(
            str_stone[(str_stone.len() / 2)..].parse::<u64>().unwrap(),
            blinks_left - 1,
        );
    } else {
        return count_stones(root_stone * 2024, blinks_left - 1);
    }
}

fn solve(input: &str, blinks: u64) -> String {
    input
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .map(|num| count_stones(num, blinks))
        .sum::<u64>()
        .to_string()
}

fn main() {
    let input = include_str!("../inputs/1.txt");
    let output = solve(input, 25);
    dbg!(output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(include_str!("../inputs/1.test.txt"), 6);
        assert_eq!(result, "22")
    }
}
