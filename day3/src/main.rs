use std::{fs::read_to_string, path::Path};
use regex::Regex;


fn main() {
    let path = Path::new("src/data.txt");

    let lines : Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();

    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let get_number = Regex::new(r"\d{1,3},\d{1,3}").unwrap();
    let mut sum = 0;

    // Part 2
    let mut inside = true;

    lines.iter()
        .for_each(|line| {
            let matches: Vec<&str> = regex.find_iter(line).map(|mat| mat.as_str()).collect();

            // part 2
            let mut filter_match = Vec::new();

            for m in matches {
                if m == "don't()"{
                    inside = false;
                }
                else if m == "do()"{
                    inside = true;
                }


                if inside && m.starts_with("mul(") {
                    filter_match.push(m);
                }
            }

            // println!("{:?}", filter_match);


            for m in filter_match {


                if let Some(numbers) = get_number.find(m) {
                    let parts: Vec<&str> = numbers.as_str().split(',').collect();
                    if parts.len() == 2 {
                        if let (Ok(x), Ok(y)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>()) {
                            sum += x * y;
                        }
                    }
                }


            }
        });

    println!("{}",sum);

}
