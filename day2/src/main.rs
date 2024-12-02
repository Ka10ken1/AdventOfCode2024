use std::{fs::read_to_string, path::Path};

fn main() {
    let path = Path::new("src/data.txt");
    let lines : Vec<String> = read_to_string(path).unwrap().lines().map(String::from).collect();


    let mut count_good = 0;

    lines.iter()
        .map(|x| x.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .for_each(|v| {

            let safe = is_safe(&v) || can_be_safe(&v);

            count_good +=  if safe {1} else {0};
        });

    println!("{}",count_good);
}

fn is_safe(v : &[i32]) -> bool {
    let mut inc = 0;
    let mut dec = 0;

    match v[1] < v[0] {
        true => dec = 1,
        false => inc = 1
    }

    for i in 1..v.len() {
        match (v[i] == v[i-1], inc == 1 && v[i] < v[i-1], dec == 1 && v[i] > v[i-1], (v[i] - v[i-1]).abs() > 3) {
            (true, _, _,_) | (_, true, _,_) | (_, _, true,_) | (_,_,_,true) => {
                return false;
            }
            _ => {}
        }

    }
    true
}

fn can_be_safe (v: &[i32]) -> bool {
     for i in 0..v.len() {
        let mut modified = v.to_vec();
        modified.remove(i);

        if is_safe(&modified) {
            return true;
        }
    }
    false
}
