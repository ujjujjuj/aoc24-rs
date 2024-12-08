fn get_res(target: u64, arr: &Vec<u64>, idx: usize, curr: u64) -> bool {
    if idx == arr.len() {
        return curr == target;
    }

    get_res(target, arr, idx + 1, curr + arr[idx])
        || get_res(target, arr, idx + 1, curr * arr[idx])
        || get_res(
            target,
            arr,
            idx + 1,
            format!("{}{}", curr, arr[idx]).parse::<u64>().unwrap(),
        )
}

fn solve(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let split: Vec<_> = line.split(":").collect();
            let target = split[0].parse::<u64>().unwrap();
            let nums: Vec<_> = split[1]
                .trim()
                .split_whitespace()
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            if get_res(target, &nums, 0, 0) {
                target
            } else {
                0
            }
        })
        .sum::<u64>()
        .to_string()
}

fn main() {
    let input = include_str!("../inputs/1.txt");
    let output = solve(input);
    dbg!(output);

    println!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(include_str!("../inputs/1.test.txt"));
        assert_eq!(result, "11387")
    }
}
