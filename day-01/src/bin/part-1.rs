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

    let mut col1: Vec<_> = nums.iter().map(|(a, _)| *a).collect();
    let mut col2: Vec<_> = nums.iter().map(|(_, b)| *b).collect();

    col1.sort();
    col2.sort();

    let x = col1
        .into_iter()
        .zip(col2.into_iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(b));

    return x.to_string();
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
        assert_eq!(result, "11")
    }
}
