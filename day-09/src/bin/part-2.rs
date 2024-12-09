use std::collections::BTreeSet;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
struct Block {
    start: u64,
    size: u64,
}

#[derive(Debug)]
struct FileBlock {
    id: u64,
    block: Block,
}

fn solve(input: &str) -> String {
    let mut empty_blocks: BTreeSet<Block> = BTreeSet::new();
    let mut file_blocks: Vec<FileBlock> = Vec::new();

    let mut mem_idx = 0;
    input.chars().enumerate().for_each(|(idx, c)| {
        let num = c.to_digit(10).unwrap() as u64;
        let id = (idx / 2) as u64;
        let is_files = idx % 2 == 0;
        if is_files {
            file_blocks.push(FileBlock {
                id,
                block: Block {
                    start: mem_idx,
                    size: num,
                },
            });
        } else {
            empty_blocks.insert(Block {
                start: mem_idx,
                size: num,
            });
        }

        mem_idx += num;
    });

    file_blocks.iter_mut().rev().for_each(|file_block| {
        let mut first_empty: Option<Block> = None;
        empty_blocks.iter().for_each(|empty_block| {
            if empty_block.size >= file_block.block.size
                && (first_empty.is_none() || first_empty.unwrap().start > empty_block.start)
                && (file_block.block.start > empty_block.start)
            {
                first_empty = Some(empty_block.clone());
            }
        });
        if let Some(first_empty_block) = first_empty {
            empty_blocks.remove(&first_empty_block);
            file_block.block.start = first_empty_block.start;

            let remaining_space = first_empty_block.size - file_block.block.size;
            if remaining_space > 0 {
                empty_blocks.insert(Block {
                    size: remaining_space,
                    start: first_empty_block.start + file_block.block.size,
                });
            }
        }
    });

    file_blocks
        .iter()
        .map(|file_block| {
            let sum = file_block.block.start * file_block.block.size
                + (file_block.block.size * (file_block.block.size - 1)) / 2;
            sum * file_block.id
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
        assert_eq!(result, "2858")
    }
}
