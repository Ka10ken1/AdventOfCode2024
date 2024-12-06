use std::{fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("src/data.txt");
    let lines: Vec<String> = read_to_string(path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let map: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let mut guard_post: (i32, i32) = (0, 0);

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                guard_post = (i as i32, j as i32);
                break;
            }
        }
    }

    println!("Guard_start: {:?}", guard_post);

    let ans = guard_path(&map, guard_post);
    println!("Steps: {:?}", ans);
}

fn guard_path(map: &Vec<Vec<char>>, guard_post: (i32, i32)) -> i32 {
    let (mut i, mut j) = guard_post;
    let m = map.len() as i32;
    let n = map[0].len() as i32;

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; 
    let mut direction_idx = 0;
    let mut steps = 1;

    let mut visited = vec![vec![false; n as usize]; m as usize];
    visited[i as usize][j as usize] = true;


    loop {
        if i == 0 || i == m - 1 || j == 0 || j == n - 1 {
            break;
        }


        let (di, dj) = directions[direction_idx];
        let ni = i + di;
        let nj = j + dj;

        if is_safe(ni, nj, n, m) && map[ni as usize][nj as usize] != '#' {
            i = ni;
            j = nj;

            if !visited[i as usize][j as usize] {
                visited[i as usize][j as usize] = true;
                steps += 1;
            }

            println!("{},{}", i, j);
        } else {
            direction_idx = (direction_idx + 1) % 4;
        }

        if directions.iter().all(|&(di, dj)| {
            let ni = i + di;
            let nj = j + dj;
            !is_safe(ni, nj, n, m) || map[ni as usize][nj as usize] == '#'
        }) {
            break;
        }
    }

    steps
}


fn is_safe(i: i32, j: i32, n: i32, m: i32) -> bool {
    i >= 0 && i < m && j >= 0 && j < n
}

