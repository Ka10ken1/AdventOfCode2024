use std::{fs::read_to_string, path::Path};


fn main() {
    let lines : Vec<String> = read_to_string(Path::new("src/test.txt")).unwrap().lines().map(String::from).collect();

    
    let (mut v1,mut v2):(Vec<i32>, Vec<i32>) = 
    lines.iter()
        .map(|line| {
            let mut parts = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())

        }).unzip();


    // parts 1
    v1.sort();
    v2.sort();
    let ans1 : i32 = v1.iter().zip(v2.iter()).map(|(x,y)| (x-y).abs()).sum();

    println!("{}", ans1);

    // parts 2
    let ans : i32 = v1.iter()
        .map(|&el| el* v2.iter().filter(|&&x| x == el).count() as i32)
        .sum();

    println!("{}", ans);

}


