use std::collections::HashMap;

fn get_valid_ordering(graph: &HashMap<u64, Vec<u64>>, mut arr: Vec<u64>) -> Vec<u64> {
    for i in 0..arr.len() {
        for j in 1..arr.len() - i {
            if let Some(children) = graph.get(&arr[j]) {
                for child in children {
                    if *child == arr[j - 1] {
                        (arr[j], arr[j - 1]) = (arr[j - 1], arr[j]);
                    }
                }
            }
        }
    }

    return arr;
}

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
                let ordering = get_valid_ordering(&graph, arr);
                ordering[ordering.len() / 2]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(include_str!("../inputs/1.test.txt"));
        assert_eq!(result, "123")
    }
}
