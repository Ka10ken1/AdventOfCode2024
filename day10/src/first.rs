use std::{collections::HashSet, fs::read_to_string, isize, path::Path};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

fn read_data() -> Vec<Vec<char>> {
    let path = Path::new("data.txt");
    let lines: Vec<String> = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    lines.iter().map(|line| line.chars().collect()).collect()
}

fn find_start(data: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut starts: Vec<Coord> = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == '0' {
                starts.push(Coord { x: i, y: j });
            }
        }
    }

    starts
}

fn is_valid(data: &Vec<Vec<char>>, visited: &Vec<Vec<bool>>, pos: Coord, next_digit: char) -> bool {
    pos.x < data.len()
        && pos.y < data[0].len()
        && !visited[pos.x][pos.y]
        && data[pos.x][pos.y] == next_digit
}

fn dfs(
    data: &mut Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start: Coord,
    current: char,
    reached_9s: &mut HashSet<Coord>,
) -> u32 {
    if current == '9' {
        if !reached_9s.contains(&start) {
            reached_9s.insert(start.clone());
            return 1;
        }
        return 0;
    }

    visited[start.x][start.y] = true;
    let next = (current as u8 + 1) as char;
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut cnt = 0;

    for (dx, dy) in directions.iter() {
        let xp = (start.x as isize + dx) as usize;
        let yp = (start.y as isize + dy) as usize;

        if is_valid(data, visited, Coord { x: xp, y: yp }, next) {
            cnt += dfs(data, visited, Coord { x: xp, y: yp }, next, reached_9s);
        }
    }

    cnt
}

pub fn ans() -> u32 {
    let mut data: Vec<Vec<char>> = read_data();
    let starts = find_start(&data);

    let mut sum = 0;

    for start in starts {
        let mut visited = vec![vec![false; data[0].len()]; data.len()];
        let mut reached_9s = HashSet::new();
        sum += dfs(&mut data, &mut visited, start, '0', &mut reached_9s);
    }
    sum
}
