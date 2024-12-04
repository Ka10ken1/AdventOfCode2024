use std::path::Path;
use std::fs::read_to_string;

fn main() {
    let path = Path::new("src/data.txt");
    let lines: Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();

    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    let cnt_xmas = count_xmas(&chars);
    let cnt_xmas_2 = count_xmas_2(&chars);
    println!("{}", cnt_xmas);
    println!("{}", cnt_xmas_2);
}

fn count_xmas(chars: &Vec<Vec<char>>) -> i32 {
    let m = chars.len();
    let n = chars[0].len();
    let word = "XMAS";
    let mut cnt = 0;

    fn find_xmas(chars: &Vec<Vec<char>>, r: i32, c: i32, dr: i32, dc: i32, i: usize, word: &str) -> bool {
        if i == word.len() {
            return true;
        }

        if r < 0 || r >= chars.len() as i32 || c < 0 || c >= chars[0].len() as i32 {
            return false;
        }

        if chars[r as usize][c as usize] != word.chars().nth(i).unwrap() {
            return false;
        }

        find_xmas(
            chars,
            r + dr,
            c + dc,
            dr,
            dc,
            i + 1,
            word
        )
    }

    for r in 0..m {
        for c in 0..n {
            let directions: Vec<(i32, i32)> = vec![
                (0, 1),   
                (0, -1), 
                (1, 0), 
                (-1, 0),
                (1, 1),  
                (-1, -1), 
                (1, -1),  
                (-1, 1),  
            ];

            for (dr, dc) in directions {
                if find_xmas(&chars, r as i32, c as i32, dr, dc, 0, &word) {
                    cnt += 1;
                }
            }
        }
    }

    cnt
}

// part 2
// find 'A' and then find is diagonal neighbors to match the "MAS" or "SAM"

fn count_xmas_2(chars: &Vec<Vec<char>>) -> i32 {
    let m = chars.len();
    let n = chars[0].len();
    let mut cnt = 0;

    fn in_bounds(r: i32, c: i32, m: usize, n: usize) -> bool {
        r >= 0 && r < m as i32 && c >= 0 && c < n as i32
    }

    fn check_cross(chars: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
        let deltas = [
            (-1, 1),  // top-right
            (1, -1),  // bottom-left
            (1, 1),   // bottom-right
            (-1, -1), // top-left
        ];

        let m = chars.len();
        let n = chars[0].len();

        let positions: Vec<(i32, i32)> = deltas
            .iter()
            .map(|&(dr, dc)| (r as i32 + dr, c as i32 + dc))
            .collect();

        if positions.iter().all(|&(r, c)| in_bounds(r, c, m, n)) {
            let top_right = chars[positions[0].0 as usize][positions[0].1 as usize];
            let bottom_left = chars[positions[1].0 as usize][positions[1].1 as usize];
            let bottom_right = chars[positions[2].0 as usize][positions[2].1 as usize];
            let top_left = chars[positions[3].0 as usize][positions[3].1 as usize];

            return (top_right == 'M' && bottom_left == 'S' && bottom_right == 'M' && top_left == 'S') ||
                   (top_right == 'S' && bottom_left == 'M' && bottom_right == 'S' && top_left == 'M') ||
                   (top_right == 'M' && bottom_left == 'S' && bottom_right == 'S' && top_left == 'M') ||
                   (top_right == 'S' && bottom_left == 'M' && bottom_right == 'M' && top_left == 'S');
        }

        false
    }

    for r in 0..m {
        for c in 0..n {
            if chars[r][c] == 'A' {
                if check_cross(chars, r, c) {
                    cnt += 1;
                }
            }
        }
    }

    cnt
}

