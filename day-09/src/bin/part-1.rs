fn solve(input: &str) -> String {
    let mut fs_blocks: Vec<Option<u64>> = Vec::new();

    input.chars().enumerate().for_each(|(idx, c)| {
        let num = c.to_digit(10).unwrap();
        let id = (idx / 2) as u64;
        let is_files = idx % 2 == 0;
        for _ in 0..num {
            if is_files {
                fs_blocks.push(Some(id));
            } else {
                fs_blocks.push(None);
            }
        }
    });

    let mut r_idx = fs_blocks.len() - 1;
    let mut l_idx = 0usize;

    loop {
        while fs_blocks[l_idx] != None {
            l_idx += 1;
        }
        while fs_blocks[r_idx] == None {
            r_idx -= 1;

        }
        if l_idx >= r_idx {
            break;
        }

        fs_blocks[l_idx] = fs_blocks[r_idx];
        fs_blocks[r_idx] = None;
    }

    fs_blocks
        .iter()
        .enumerate()
        .map(|(idx, num)| {
            if num.is_some() {
                return (idx as u64) * num.unwrap();
            }
            0
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
        assert_eq!(result, "1928")
    }
}
