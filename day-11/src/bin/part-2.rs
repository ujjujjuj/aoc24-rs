use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

fn count_stones(root_stone: u64, blinks_left: u64) -> u64 {
    lazy_static! {
        static ref FUNC_CACHE: Mutex<HashMap<(u64, u64), u64>> = Mutex::new(HashMap::new());
    }

    {
        let cache = FUNC_CACHE.lock().unwrap();
        if let Some(&cached_val) = cache.get(&(root_stone, blinks_left)) {
            return cached_val;
        }
    }

    if blinks_left == 0 {
        return 1;
    }

    let ret_val: u64;
    let str_stone = root_stone.to_string();

    if root_stone == 0 {
        ret_val = count_stones(1, blinks_left - 1);
    } else if str_stone.len() % 2 == 0 {
        ret_val = count_stones(
            str_stone[..(str_stone.len() / 2)].parse::<u64>().unwrap(),
            blinks_left - 1,
        ) + count_stones(
            str_stone[(str_stone.len() / 2)..].parse::<u64>().unwrap(),
            blinks_left - 1,
        );
    } else {
        ret_val = count_stones(root_stone * 2024, blinks_left - 1);
    }

    {
        let mut cache = FUNC_CACHE.lock().unwrap();
        cache.insert((root_stone, blinks_left), ret_val);
    }

    return ret_val;
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
    let output = solve(input, 75);
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
