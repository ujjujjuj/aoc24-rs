use std::collections::HashMap;

fn solve(input: &str) -> String {
    let nums: Vec<_> = input
        .lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if let (Ok(a), Ok(b)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
                    return Some((a, b));
                }
            }
            None
        })
        .collect();

    let mut freqs = HashMap::new();
    nums.iter()
        .for_each(|(_, num)| *freqs.entry(num).or_insert(0) += 1);

    return nums
        .iter()
        .fold(0, |acc, (a, _)| acc + a * (freqs.get(a).unwrap_or(&0)))
        .to_string();
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
        assert_eq!(result, "31")
    }
}
