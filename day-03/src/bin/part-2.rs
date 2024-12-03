use regex::Regex;

fn solve(input: &str) -> String {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();

    let mut do_mul = true;
    re.captures_iter(input)
        .map(|cap| {
            dbg!(&cap);
            if cap[0].starts_with("d") {
                if cap[0].starts_with("don") {
                    do_mul = false
                } else {
                    do_mul = true
                }
                return 0;
            }

            if !do_mul {
                return 0;
            }

            let x = cap[1].parse::<u64>().unwrap();
            let y = cap[2].parse::<u64>().unwrap();

            return x * y;
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
        let result = solve(include_str!("../inputs/2.test.txt"));
        assert_eq!(result, "48")
    }
}
