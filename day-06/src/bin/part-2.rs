use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn r#move(&self, dir: &Direction) -> Coord {
        let mut x = self.x;
        let mut y = self.y;
        match dir {
            Direction::Up => y -= 1,
            Direction::Right => x += 1,
            Direction::Down => y += 1,
            Direction::Left => x -= 1,
        };

        return Coord { x, y };
    }
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Player,
}

fn step(map: &Vec<Vec<Item>>, coord: Coord, dir: &Direction) -> Option<(Coord, Direction)> {
    let mut move_dir = dir.clone();
    for _ in 0..4 {
        let next_coord = coord.r#move(&move_dir);
        if next_coord.x < 0
            || next_coord.y < 0
            || next_coord.y >= map.len() as i64
            || next_coord.x >= map[0].len() as i64
        {
            return None;
        };

        if map[next_coord.y as usize][next_coord.x as usize] == Item::Wall {
            move_dir = move_dir.next();
            continue;
        }

        return Some((next_coord, move_dir));
    }

    return None;
}

fn simu(map: &Vec<Vec<Item>>, player_pos: Coord) -> bool {
    let mut vis_dir: HashSet<(Coord, Direction)> = HashSet::new();

    let mut next_coord = player_pos.clone();
    let mut next_dir = Direction::Up;
    vis_dir.insert((next_coord, next_dir));
    loop {
        match step(map, next_coord, &next_dir) {
            Some((_next_coord, _dir)) => {
                next_coord = _next_coord;
                next_dir = _dir;
            }
            None => {
                return true;
            }
        }
        if vis_dir.contains(&(next_coord, next_dir)) {
            return false;
        }
        vis_dir.insert((next_coord, next_dir));
    }
}

fn solve(input: &str) -> String {
    let map: Vec<Vec<Item>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Item::Empty,
                    '#' => Item::Wall,
                    '^' => Item::Player,
                    _ => panic!("Unexpected value in map: {}", c),
                })
                .collect()
        })
        .collect();

    let mut player_pos = Coord { x: 0, y: 0 };
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Item::Player {
                player_pos = Coord {
                    x: j as i64,
                    y: i as i64,
                };
            }
        }
    }

    let mut ans = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == Item::Empty {
                let mut new_map = map.clone();
                new_map[i][j] = Item::Wall;
                if !simu(&new_map, player_pos) {
                    ans += 1
                }
            }
        }
    }

    ans.to_string()
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
        assert_eq!(result, "6")
    }
}
