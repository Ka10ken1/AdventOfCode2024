use std::{fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("src/data.txt");
    let lines : Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();
    let str_array : Vec<String> = lines.get(0).unwrap().chars().map(String::from).collect();

    let n = str_array.len();
    let mut file_free : Vec<String> = Vec::new();

    let mut k = 0;
    for i in 0..n {
        let char_val = str_array[i].parse::<i32>().unwrap();
        if i % 2 == 0 {
            for _ in 0..char_val{
                file_free.push(k.to_string());
            }
            k+=1;
        }
        else {
            for _ in 0..char_val {
                file_free.push(".".to_string());
            }
        }
   }

    let mut r = file_free.len() - 1;

    let mut i = 0;

    while i < file_free.len() && r > i{
        if file_free[i] == "." {
            file_free[i] = file_free[r].clone();
            file_free[r] = ".".to_string();
            r -= 1;
        }
        else{
            i+=1
        }
    }

    let ans : i64 = file_free.iter().enumerate().filter_map(|(i,x)| x.parse::<i64>().ok().map(|v| v as i64 * i as i64))
        .sum();



    println!("{}", ans);

}
