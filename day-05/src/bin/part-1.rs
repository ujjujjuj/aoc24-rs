use std::collections::HashMap;

fn solve(input: &str) -> String {
    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();

    let parts: Vec<_> = input.split("\n\n").collect();

    let &[rules, tests] = parts.as_slice() else {
        panic!("Input does not split into exactly two parts");
    };

    rules.lines().for_each(|r| {
        let rule_parts: Vec<_> = r.split("|").map(|p| p.parse::<u64>().unwrap()).collect();
        let &[before, after] = rule_parts.as_slice() else {
            panic!("Rule does not split into exactly two parts: {}", r);
        };

        graph.entry(before).or_insert(Vec::new()).push(after);
    });

    tests
        .lines()
        .map(|t| {
            let arr: Vec<_> = t
                .split(",")
                .map(|num| num.parse::<u64>().unwrap())
                .collect();

            let mut invalid = false;
            for i in 0..arr.len() {
                for j in i + 1..arr.len() {
                    if let Some(nodes) = graph.get(&arr[j]) {
                        for node in nodes {
                            if *node == arr[i] {
                                invalid = true;
                            }
                        }
                    }
                }
            }

            if invalid {
                0
            } else {
                arr[arr.len() / 2]
            }
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
        assert_eq!(result, "143")
    }
}
